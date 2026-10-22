//! External regression tests for native tree layout and Rust ownership.

use highfleet::general::{EscadraString, Tll, TllContainer};
use std::{
    cell::Cell,
    cmp::Ordering,
    collections::HashSet,
    mem::{align_of, offset_of, size_of},
    panic::{catch_unwind, AssertUnwindSafe},
    rc::Rc,
};

#[derive(Clone, Default)]
struct Counts {
    live: Rc<Cell<usize>>,
    drops: Rc<Cell<usize>>,
    clone_attempts: Rc<Cell<usize>>,
    panic_clone_at: Rc<Cell<Option<usize>>>,
    panic_drop_once: Rc<Cell<bool>>,
}

struct Probe {
    counts: Counts,
}

impl Probe {
    fn new(counts: &Counts) -> Self {
        counts.live.set(counts.live.get() + 1);
        Self {
            counts: counts.clone(),
        }
    }
}

impl Clone for Probe {
    fn clone(&self) -> Self {
        let attempt = self.counts.clone_attempts.get() + 1;
        self.counts.clone_attempts.set(attempt);
        assert_ne!(
            self.counts.panic_clone_at.get(),
            Some(attempt),
            "deliberate clone panic"
        );
        Self::new(&self.counts)
    }
}

impl Drop for Probe {
    fn drop(&mut self) {
        self.counts.live.set(self.counts.live.get() - 1);
        self.counts.drops.set(self.counts.drops.get() + 1);
        assert!(
            !self.counts.panic_drop_once.replace(false),
            "deliberate drop panic"
        );
    }
}

#[derive(Clone)]
struct Key {
    id: i32,
    _probe: Probe,
}

impl Key {
    fn new(id: i32, counts: &Counts) -> Self {
        Self {
            id,
            _probe: Probe::new(counts),
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Key {}
impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

#[test]
fn empty_tree_needs_no_default_payloads_and_drops_none() {
    let mut tree = TllContainer::<Key, Probe>::new();
    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
    assert!(tree.get_items().is_empty());
    unsafe {
        let sentinel = tree.sentinel();
        assert!(!sentinel.is_null());
        assert_eq!((*sentinel).left, sentinel);
        assert_eq!((*sentinel).parent, sentinel);
        assert_eq!((*sentinel).right, sentinel);
        assert!((*sentinel).is_sentinel);
        assert!((*sentinel).is_black);
    }
    let cloned = tree.clone();
    assert_ne!(tree.sentinel(), cloned.sentinel());
    assert!(cloned.is_empty());
}

#[test]
fn dropping_populated_tree_drops_every_real_key_and_value_once() {
    let keys = Counts::default();
    let values = Counts::default();
    {
        let mut tree = TllContainer::new();
        for id in [8, 4, 12, 2, 6, 10, 14, 1, 3] {
            tree.insert(Key::new(id, &keys), Probe::new(&values));
        }
        assert_eq!(tree.len(), 9);
        assert_eq!(keys.live.get(), 9);
        assert_eq!(values.live.get(), 9);
    }
    assert_eq!(keys.live.get(), 0);
    assert_eq!(values.live.get(), 0);
    assert_eq!(keys.drops.get(), 9);
    assert_eq!(values.drops.get(), 9);
}

#[test]
fn duplicate_insert_keeps_original_key_and_drops_replaced_value() {
    let keys = Counts::default();
    let values = Counts::default();
    let mut tree = TllContainer::new();
    tree.insert(Key::new(7, &keys), Probe::new(&values));
    tree.insert(Key::new(7, &keys), Probe::new(&values));
    assert_eq!(tree.len(), 1);
    assert_eq!(keys.live.get(), 1);
    assert_eq!(values.live.get(), 1);
    assert_eq!(keys.drops.get(), 1);
    assert_eq!(values.drops.get(), 1);
    drop(tree);
    assert_eq!(keys.drops.get(), 2);
    assert_eq!(values.drops.get(), 2);
}

unsafe fn allocations<T, U>(sentinel: *mut Tll<T, U>) -> HashSet<usize> {
    let mut result = HashSet::from([sentinel as usize]);
    let mut pending = vec![(*sentinel).parent];
    while let Some(node) = pending.pop() {
        if node == sentinel {
            continue;
        }
        assert!(
            result.insert(node as usize),
            "tree contains a cycle or shared node"
        );
        pending.push((*node).left);
        pending.push((*node).right);
    }
    result
}

#[test]
fn clones_have_disjoint_allocations_and_outlive_the_original() {
    let keys = Counts::default();
    let values = Counts::default();
    let mut tree = TllContainer::new();
    for id in [4, 2, 6, 1, 3, 5, 7] {
        tree.insert(Key::new(id, &keys), Probe::new(&values));
    }
    let mut cloned = tree.clone();
    let original_allocations = unsafe { allocations(tree.sentinel()) };
    let cloned_allocations = unsafe { allocations(cloned.sentinel()) };
    assert_eq!(original_allocations.len(), 8);
    assert_eq!(cloned_allocations.len(), 8);
    assert!(original_allocations.is_disjoint(&cloned_allocations));
    assert_eq!(keys.live.get(), 14);
    assert_eq!(values.live.get(), 14);
    drop(tree);
    assert_eq!(cloned.get_items().len(), 7);
    assert_eq!(keys.live.get(), 7);
    assert_eq!(values.live.get(), 7);
    drop(cloned);
    assert_eq!(keys.live.get(), 0);
    assert_eq!(values.live.get(), 0);
}

#[test]
fn panic_in_either_payload_clone_reclaims_the_partial_clone() {
    for panic_in_keys in [true, false] {
        for failure in 1..=7 {
            let keys = Counts::default();
            let values = Counts::default();
            let mut tree = TllContainer::new();
            for id in [4, 2, 6, 1, 3, 5, 7] {
                tree.insert(Key::new(id, &keys), Probe::new(&values));
            }
            let failing = if panic_in_keys { &keys } else { &values };
            failing.panic_clone_at.set(Some(failure));
            assert!(catch_unwind(AssertUnwindSafe(|| tree.clone())).is_err());
            assert_eq!(keys.live.get(), 7);
            assert_eq!(values.live.get(), 7);
            assert_eq!(tree.get_items().len(), 7);
            drop(tree);
            assert_eq!(keys.live.get(), 0);
            assert_eq!(values.live.get(), 0);
        }
    }
}

#[test]
fn cleanup_continues_after_a_payload_destructor_panics() {
    for panic_in_keys in [true, false] {
        let keys = Counts::default();
        let values = Counts::default();
        let mut tree = TllContainer::new();
        for id in [2, 1, 3] {
            tree.insert(Key::new(id, &keys), Probe::new(&values));
        }
        let failing = if panic_in_keys { &keys } else { &values };
        failing.panic_drop_once.set(true);
        assert!(catch_unwind(AssertUnwindSafe(|| drop(tree))).is_err());
        assert_eq!(keys.live.get(), 0);
        assert_eq!(values.live.get(), 0);
        assert_eq!(keys.drops.get(), 3);
        assert_eq!(values.drops.get(), 3);
    }
}

#[test]
fn replacing_a_panicking_value_leaves_the_new_value_initialized() {
    let values = Counts::default();
    let mut tree = TllContainer::new();
    tree.insert(7, Probe::new(&values));
    values.panic_drop_once.set(true);
    let replacement = Probe::new(&values);
    assert!(catch_unwind(AssertUnwindSafe(|| tree.insert(7, replacement))).is_err());
    assert_eq!(tree.len(), 1);
    assert_eq!(tree.get_items().len(), 1);
    assert_eq!(values.live.get(), 1);
    drop(tree);
    assert_eq!(values.live.get(), 0);
}

#[test]
fn raw_handoff_does_not_drop_payloads_and_adoption_restores_ownership() {
    let keys = Counts::default();
    let values = Counts::default();
    let mut tree = TllContainer::new();
    tree.insert(Key::new(1, &keys), Probe::new(&values));
    let (sentinel, size) = tree.into_raw_parts();
    assert_eq!(keys.drops.get(), 0);
    assert_eq!(values.drops.get(), 0);
    let mut adopted = unsafe { TllContainer::from_raw_parts(sentinel, size) };
    assert_eq!(adopted.get_items().len(), 1);
    drop(adopted);
    assert_eq!(keys.drops.get(), 1);
    assert_eq!(values.drops.get(), 1);
}

#[test]
fn cloned_heap_strings_are_independent_and_values_are_mutably_borrowed() {
    let mut tree = TllContainer::new();
    let mut key = EscadraString::new();
    key.set_string(&"A key longer than inline storage".to_owned());
    let mut value = EscadraString::new();
    value.set_string(&"A value longer than inline storage".to_owned());
    tree.insert(key, value);
    let mut cloned = tree.clone();
    for value in cloned.get_items() {
        value.set_string(&"Changed cloned value with heap storage".to_owned());
    }
    assert_eq!(
        tree.get_items()[0].get_string(),
        "A value longer than inline storage"
    );
    drop(tree);
    assert_eq!(
        cloned.get_items()[0].get_string(),
        "Changed cloned value with heap storage"
    );
}

#[test]
fn node_specialization_preserves_the_old_plane_map_payload_layout() {
    #[repr(C)]
    struct PlanePayload {
        prefix: [u8; 8],
        vector_words: [*const u8; 3],
    }
    type Node = Tll<EscadraString, PlanePayload>;
    assert_eq!(size_of::<TllContainer<EscadraString, PlanePayload>>(), 0x10);
    assert_eq!(align_of::<Node>(), 8);
    assert_eq!(offset_of!(Node, key), 0x20);
    assert_eq!(offset_of!(Node, data), 0x40);
    assert_eq!(size_of::<Node>(), 0x60);
}
