//! Owned, libc-allocated red-black trees with the native HighFleet map layout (`std::map<K, V>`).
//!
//! [`TllContainer`] owns a sentinel and its real [`Tll`] nodes.
//! Use [`TllContainer::get_items`] or [`TllContainer::get_map`] for normal payload access.
//! For ownership and native interoperability, see [`TllContainer`], [`TllContainer::into_raw_parts`], and [`TllContainer::from_raw_parts`].
//!
//! # Example
//!
//! ```
//! use highfleet::general::TllContainer;
//!
//! let mut tree = TllContainer::new();
//! tree.insert(2, String::from("second"));
//! tree.insert(1, String::from("first"));
//! let mut independent = tree.clone();
//! drop(tree);
//! assert_eq!(independent.get_items()[0], "first");
//! ```
//!
//! # Native layout
//!
//! The container header is two machine words: a sentinel pointer and an element count.
//! On 64-bit targets, the node header is three pointers, two boolean flags, and six padding bytes (`0x20` bytes before the key for the usual native payload alignments).
//! For an [`EscadraString`](super::EscadraString) key and a 32-byte plane payload, the node is `0x60` bytes, with the key at `0x20` and payload at `0x40`.
//! Sentinel payload storage is never initialized.
//! This tree is distinct from the circular [`SentinelList`](super::SentinelList) headers used by Node's child/joint collections.

use std::{
    alloc::{handle_alloc_error, Layout},
    hash::Hash,
    mem::{align_of, size_of, ManuallyDrop, MaybeUninit},
    ptr,
};

/// An exclusively owned native-layout map, including its sentinel and all nodes.
///
/// # Ownership and cleanup
///
/// Construction owns libc-allocated storage; see [`Self::new`] for allocation requirements.
/// [`Clone::clone`] creates an independent tree and clones its initialized keys and values.
/// Dropping the container destroys each real key/value once and frees every node and the sentinel, without reading or dropping sentinel payloads.
/// Cleanup follows each payload's own [`Drop`] implementation.
/// Raw pointer targets are not recursively destroyed, and a payload without cleanup, such as the current [`CVec`](super::CVec), retains that behavior.
///
/// # Native interoperability
///
/// The header fields are private to prevent fabrication of invalid owners in safe Rust; their native C layout is unchanged.
/// Use [`Self::sentinel`] for non-owning raw access, [`Self::len`] for the number of real nodes, and unsafe [`Self::get_sentinel`] only when preserving all invariants.
/// Borrow a game-owned header through a raw pointer/reference; borrowing does not run its destructor.
/// [`Self::from_raw_parts`] adopts exclusive ownership and must not be used on a borrowed game tree.
/// [`core::ptr::write`] can move a whole owned header to native storage without dropping it, but does not reclaim the destination's previous contents.
///
/// # Invariants
///
/// Every allocation must come from a compatible libc allocator.
/// Payloads of real nodes are initialized Rust values; sentinel payloads are not.
/// All links are non-null and missing children point to this tree's unique sentinel.
/// The sentinel's parent/left/right are the root/minimum/maximum, or itself for an empty tree.
/// Real-node parent links and the red-black ordering must remain valid, and `size` must equal the number of real nodes.
#[repr(C)]
#[derive(Debug)]
pub struct TllContainer<T, U> {
    /// Owned sentinel pointer; changing it must preserve all ownership invariants.
    sentinel: *mut Tll<T, U>,
    /// Number of real nodes; must agree with the reachable tree.
    size: usize,
}

/// A native red-black-tree node owned by a [`TllContainer`].
///
/// The sentinel acts as the parent of the root and represents missing children instead of null pointers.
/// Payload fields use [`MaybeUninit<T>`] and [`MaybeUninit<U>`] to model the uninitialized sentinel storage without changing field offsets.
/// Only real nodes contain initialized keys and values.
/// Unsafe payload accessors are for real nodes only; normal callers should use [`TllContainer::get_items`] or [`TllContainer::get_map`].
///
/// Raw links do not confer ownership, and individual linked nodes deliberately do not implement [`Clone`].
/// Copying a node header must never create another owning container.
/// Use [`TllContainer::clone`] to clone an entire tree.
#[repr(C)]
#[derive(Debug)]
pub struct Tll<T, U> {
    /// The left child node of this node.
    /// If this is the sentinel node, this points to the leftmost node in the tree.
    pub left: *mut Tll<T, U>,
    /// The parent node of this node.
    /// If this is the sentinel node, this points to the root node.
    pub parent: *mut Tll<T, U>,
    /// The right child node of this node.
    /// If this is the sentinel node, this points to the rightmost node in the tree.
    pub right: *mut Tll<T, U>,
    /// Is this node black?
    pub is_black: bool,
    /// Is this node the sentinel?
    /// This means it is either the root parent node or a null node when traversing the tree.
    pub is_sentinel: bool,
    _padding: [u8; 6],
    /// Initialized key on real nodes; uninitialized on the sentinel.
    pub key: MaybeUninit<T>,
    /// Initialized value on real nodes; uninitialized on the sentinel.
    pub data: MaybeUninit<U>,
}

impl<T, U> TllContainer<T, U> {
    /// Creates an empty owned tree with a libc-allocated sentinel and no real nodes.
    ///
    /// # Panics
    ///
    /// Panics before allocation if the node alignment exceeds the platform's libc malloc guarantee.
    /// Over-aligned payloads require a different allocator.
    pub fn new() -> Self {
        unsafe {
            let sentinel_ptr = allocate_node::<T, U>();

            let sentinel = Tll {
                left: sentinel_ptr,
                parent: sentinel_ptr,
                right: sentinel_ptr,
                is_black: true,
                is_sentinel: true,
                _padding: [0; 6],
                key: MaybeUninit::uninit(),
                data: MaybeUninit::uninit(),
            };

            std::ptr::write(sentinel_ptr, sentinel);

            Self {
                sentinel: sentinel_ptr,
                size: 0,
            }
        }
    }
}

impl<T, U> TllContainer<T, U> {
    /// Returns whether the tree contains no real nodes.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the number of real nodes.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns a non-owning raw pointer to the sentinel for native interoperability.
    ///
    /// The pointer is valid only while this container's allocations remain alive.
    /// Dereferencing it requires unsafe code, which must preserve the ownership and tree invariants and respect any outstanding container borrows.
    pub fn sentinel(&self) -> *mut Tll<T, U> {
        self.sentinel
    }

    /// Borrows the sentinel header for native interoperability.
    ///
    /// # Safety
    /// The caller must preserve the container's ownership and tree invariants, and must never initialize, read, or drop sentinel payloads as `T`/`U`.
    pub unsafe fn get_sentinel(&mut self) -> &mut Tll<T, U> {
        &mut *self.sentinel
    }

    /// Transfers ownership of all allocations without dropping any payloads.
    ///
    /// Returns the sentinel pointer and real-node count needed by [`Self::from_raw_parts`].
    /// The recipient must eventually destroy the payloads and reclaim all nodes with a compatible allocator.
    pub fn into_raw_parts(self) -> (*mut Tll<T, U>, usize) {
        let this = ManuallyDrop::new(self);
        (this.sentinel, this.size)
    }

    /// Adopts exclusive ownership of a native-layout tree.
    ///
    /// # Safety
    /// All container invariants documented on [`TllContainer`] must hold.
    /// Every node, including the non-null sentinel, must be a separate suitably aligned libc allocation compatible with `libc::free` in this process.
    /// Real payloads must be valid owned Rust `T`/`U` values, safe to drop once.
    /// No other owner may free nodes or payloads, and no outstanding references may conflict with this owner.
    /// The payload ordering must agree with [`Ord`] when insertion is used.
    /// Borrowed game memory must not be adopted.
    pub unsafe fn from_raw_parts(sentinel: *mut Tll<T, U>, size: usize) -> Self {
        assert_supported_alignment::<T, U>();
        Self { sentinel, size }
    }

    /// Returns mutable values in key order, borrowing this entire container.
    pub fn get_items(&mut self) -> Vec<&mut U> {
        let mut items = Vec::new();
        unsafe {
            in_order_traverse((*self.sentinel).parent, &mut items);
        }
        items.into_iter().map(|(_, data)| data).collect()
    }

    /// Returns shared keys and disjoint mutable values, borrowing this container.
    pub fn get_map(&mut self) -> std::collections::HashMap<&T, &mut U>
    where
        T: Eq + Hash,
    {
        let mut items = Vec::new();
        unsafe {
            in_order_traverse((*self.sentinel).parent, &mut items);
        }
        items.into_iter().collect()
    }

    /// Inserts a key/value pair, replacing and dropping the previous value if present.
    pub fn insert(&mut self, key: T, value: U)
    where
        T: Ord,
    {
        unsafe {
            // Check if the tree is empty, aka there are no nodes other than the sentinel
            let sentinel = self.sentinel;
            if std::ptr::eq((*sentinel).parent, sentinel) {
                // Tree is empty, insert as root
                let new_node_ptr = Tll::new(key, value, sentinel);
                (*new_node_ptr).is_black = true; // Root must be black

                // Update sentinel to point to new root
                (*self.sentinel).parent = new_node_ptr;
                (*self.sentinel).left = new_node_ptr;
                (*self.sentinel).right = new_node_ptr;

                self.size += 1;
                return;
            }

            // Find insertion point with standard BST insertion logic
            let mut current = (*sentinel).parent;
            let mut parent = sentinel;

            let mut insert_left = false;
            while !(*current).is_sentinel {
                parent = current;
                match key.cmp((*current).key.assume_init_ref()) {
                    std::cmp::Ordering::Less => {
                        insert_left = true;
                        current = (*current).left;
                    }
                    std::cmp::Ordering::Greater => {
                        insert_left = false;
                        current = (*current).right;
                    }
                    std::cmp::Ordering::Equal => {
                        // Install first so a panicking old-value destructor cannot
                        // leave the tree with uninitialized payload storage.
                        let old =
                            ptr::replace(ptr::addr_of_mut!((*current).data).cast::<U>(), value);
                        drop(old);
                        return;
                    }
                }
            }

            let new_node = Tll::new(key, value, sentinel);
            (*new_node).parent = parent;

            if insert_left {
                (*parent).left = new_node;
            } else {
                (*parent).right = new_node;
            }

            self.size += 1;

            self.insert_fixup(new_node);

            self.update_sentinel_min_max();
        }
    }

    unsafe fn insert_fixup(&mut self, mut node: *mut Tll<T, U>) {
        let sentinel = self.sentinel;

        while !(*(*node).parent).is_black {
            let parent = (*node).parent;
            let gp = (*node)
                .grandparent()
                .expect("red parent must have a real grandparent");

            if std::ptr::eq(parent, (*gp).left) {
                let uncle = (*gp).right;

                if !(*uncle).is_black {
                    (*parent).is_black = true;
                    (*uncle).is_black = true;
                    (*gp).is_black = false;
                    node = gp;
                } else {
                    if std::ptr::eq(node, (*parent).right) {
                        node = parent;
                        (*node).rotate_left(sentinel);
                    }

                    // Re-read parent and grandparent after potential rotation
                    let parent = (*node).parent;
                    let grandparent = (*node).grandparent().unwrap();

                    (*parent).is_black = true;
                    (*grandparent).is_black = false;
                    (*grandparent).rotate_right(sentinel);
                }
            } else {
                // Mirror cases for right side
                let uncle = (*gp).left;

                if !(*uncle).is_black {
                    (*parent).is_black = true;
                    (*uncle).is_black = true;
                    (*gp).is_black = false;
                    node = gp;
                } else {
                    if std::ptr::eq(node, (*parent).left) {
                        node = parent;
                        (*node).rotate_right(sentinel);
                    }

                    // Re-read after potential rotation
                    let parent = (*node).parent;
                    let grandparent = (*node).grandparent().unwrap();

                    (*parent).is_black = true;
                    (*grandparent).is_black = false;
                    (*grandparent).rotate_left(sentinel);
                }
            }
        }

        // Ensure root is black
        (*(*sentinel).parent).is_black = true;
    }

    unsafe fn update_sentinel_min_max(&mut self) {
        let sentinel = self.sentinel;
        let root = (*sentinel).parent;

        if (*root).is_sentinel {
            // Empty tree
            (*sentinel).left = sentinel;
            (*sentinel).right = sentinel;
            return;
        }

        // Find leftmost (minimum)
        let mut min = root;
        while !(*(*min).left).is_sentinel {
            min = (*min).left;
        }
        (*sentinel).left = min;

        // Find rightmost (maximum)
        let mut max = root;
        while !(*(*max).right).is_sentinel {
            max = (*max).right;
        }
        (*sentinel).right = max;
    }
}

impl<T, U> Tll<T, U> {
    /// Borrows the initialized key of a real node.
    ///
    /// # Safety
    /// This must be a real node with an initialized, valid key, not a sentinel.
    pub unsafe fn key(&self) -> &T {
        self.key.assume_init_ref()
    }

    /// Borrows the initialized value of a real node.
    ///
    /// # Safety
    /// This must be a real node with an initialized, valid value, not a sentinel.
    pub unsafe fn data(&self) -> &U {
        self.data.assume_init_ref()
    }

    /// Mutably borrows the initialized value of a real node.
    ///
    /// # Safety
    /// This must be a real node with an initialized, valid value, not a sentinel.
    /// The caller must have exclusive access to the value for the borrow.
    pub unsafe fn data_mut(&mut self) -> &mut U {
        self.data.assume_init_mut()
    }

    unsafe fn new(key: T, value: U, sentinel: *mut Tll<T, U>) -> *mut Tll<T, U> {
        let node_ptr = allocate_node::<T, U>();

        let node = Tll {
            left: sentinel,
            parent: sentinel,
            right: sentinel,
            is_black: false,
            is_sentinel: false,
            _padding: [0; 6],
            key: MaybeUninit::new(key),
            data: MaybeUninit::new(value),
        };

        std::ptr::write(node_ptr, node);
        node_ptr
    }

    unsafe fn rotate_left(&mut self, sentinel: *mut Tll<T, U>) -> *mut Tll<T, U> {
        let x = self.right;
        let y = (*x).left;
        let parent = self.parent;

        // Perform rotation
        (*x).left = self;
        self.right = y;

        // Update parents
        (*x).parent = parent;
        self.parent = x;

        if !(*y).is_sentinel {
            (*y).parent = self;
        }

        if (*parent).is_sentinel {
            // self was the root, update sentinel
            (*sentinel).parent = x;
        } else if std::ptr::eq((*parent).left, self) {
            // self was a left child
            (*parent).left = x;
        } else {
            // self was a right child
            (*parent).right = x;
        }

        x
    }

    unsafe fn rotate_right(&mut self, sentinel: *mut Tll<T, U>) -> *mut Tll<T, U> {
        let x = self.left;
        let y = (*x).right;
        let parent = self.parent;

        // Perform rotation
        (*x).right = self;
        self.left = y;

        // Update parents
        (*x).parent = parent;
        self.parent = x;

        if !(*y).is_sentinel {
            (*y).parent = self;
        }

        if (*parent).is_sentinel {
            // self was the root, update sentinel
            (*sentinel).parent = x;
        } else if std::ptr::eq((*parent).left, self) {
            // self was a left child
            (*parent).left = x;
        } else {
            // self was a right child
            (*parent).right = x;
        }

        x
    }

    unsafe fn grandparent(&self) -> Option<*mut Tll<T, U>> {
        if (*self.parent).is_sentinel || (*(*self.parent).parent).is_sentinel {
            None
        } else {
            Some((*self.parent).parent)
        }
    }
}

impl<T, U> Default for TllContainer<T, U> {
    fn default() -> Self {
        Self::new()
    }
}

/// Clones every real key/value into independent libc-allocated nodes, preserving topology and colors.
///
/// If a key or value clone panics, completed payloads and partially constructed tree allocations are reclaimed.
impl<T: Clone, U: Clone> Clone for TllContainer<T, U> {
    fn clone(&self) -> Self {
        let mut result = Self::new();
        unsafe {
            let sentinel = result.sentinel;

            // Each completed node is attached before cloning its children. If a
            // payload clone panics, result's Drop owns the entire partial tree.
            result.clone_branch(
                (*self.sentinel).parent,
                sentinel,
                ptr::addr_of_mut!((*sentinel).parent),
            );
            result.update_sentinel_min_max();
        }
        result
    }
}

impl<T: Clone, U: Clone> TllContainer<T, U> {
    unsafe fn clone_branch(
        &mut self,
        source: *const Tll<T, U>,
        parent: *mut Tll<T, U>,
        destination: *mut *mut Tll<T, U>,
    ) {
        if (*source).is_sentinel {
            return;
        }

        // Keep these as locals until both clones succeed, so a value-clone
        // panic also drops the newly cloned key without allocating a node.
        let key = (*source).key.assume_init_ref().clone();
        let data = (*source).data.assume_init_ref().clone();

        let node = Tll::new(key, data, self.sentinel);
        (*node).parent = parent;
        (*node).is_black = (*source).is_black;
        destination.write(node);

        self.size += 1;
        self.clone_branch((*source).left, node, ptr::addr_of_mut!((*node).left));
        self.clone_branch((*source).right, node, ptr::addr_of_mut!((*node).right));
    }
}

/// Destroys real payloads and frees every node and the sentinel without allocating or recursing.
///
/// A cleanup guard continues reclaiming the remaining tree if a payload destructor panics.
/// A second panic while already unwinding aborts, as with standard collections.
/// Sentinel key/value storage is never read or dropped.
impl<T, U> Drop for TllContainer<T, U> {
    fn drop(&mut self) {
        unsafe {
            let mut cleanup = TreeCleanup {
                sentinel: self.sentinel,
                next: (*self.sentinel).parent,
            };

            // If a payload destructor unwinds, cleanup's Drop resumes from the
            // already-detached node's parent and eventually frees the sentinel.
            cleanup.finish();
        }
    }
}

// Stack-local destruction state only; this does not change the native header.
struct TreeCleanup<T, U> {
    sentinel: *mut Tll<T, U>,
    next: *mut Tll<T, U>,
}

impl<T, U> TreeCleanup<T, U> {
    unsafe fn finish(&mut self) {
        let sentinel = self.sentinel;
        if sentinel.is_null() {
            return;
        }
        // Destructively walk to leaves using parent links, without allocating
        // or recursing. Cached sentinel minimum/maximum links are never used.
        while self.next != sentinel {
            let node = self.next;
            if (*node).left != sentinel {
                self.next = (*node).left;
            } else if (*node).right != sentinel {
                self.next = (*node).right;
            } else {
                let parent = (*node).parent;
                if parent == sentinel {
                    (*sentinel).parent = sentinel;
                } else if (*parent).left == node {
                    (*parent).left = sentinel;
                } else {
                    (*parent).right = sentinel;
                }

                // Advance and detach before invoking user destructors.
                // Move both payloads out and free the node.
                // Tuple drop glue still drops the value if the key destructor panics.
                self.next = parent;
                let key = ptr::addr_of!((*node).key).cast::<T>().read();
                let data = ptr::addr_of!((*node).data).cast::<U>().read();
                libc::free(node.cast());
                drop((key, data));
            }
        }
        libc::free(sentinel.cast());
        self.sentinel = ptr::null_mut();
    }
}

impl<T, U> Drop for TreeCleanup<T, U> {
    fn drop(&mut self) {
        // A second payload panic during unwinding aborts, as with Vec's drop.
        unsafe {
            self.finish();
        }
    }
}

fn assert_supported_alignment<T, U>() {
    // libc does not expose max_align_t on Windows. The CRT guarantees 16-byte
    // malloc alignment on 64-bit Windows and 8-byte alignment on 32-bit Windows.
    #[cfg(windows)]
    let malloc_alignment = if cfg!(target_pointer_width = "64") {
        16
    } else {
        8
    };

    #[cfg(not(windows))]
    let malloc_alignment = align_of::<libc::max_align_t>();
    assert!(
        align_of::<Tll<T, U>>() <= malloc_alignment,
        "Tll node alignment exceeds libc malloc alignment"
    );
}

unsafe fn allocate_node<T, U>() -> *mut Tll<T, U> {
    assert_supported_alignment::<T, U>();
    let node = libc::malloc(size_of::<Tll<T, U>>()).cast::<Tll<T, U>>();
    if node.is_null() {
        handle_alloc_error(Layout::new::<Tll<T, U>>());
    }
    node
}

// The caller must exclusively own the valid subtree for the result lifetime.
// Nodes must be unique and acyclic; only real-node payloads are borrowed.
unsafe fn in_order_traverse<T, U>(node: *mut Tll<T, U>, result: &mut Vec<(&T, &mut U)>) {
    if node.is_null() || (*node).is_sentinel {
        return;
    }

    in_order_traverse((*node).left, result);

    // Do not create a whole-node mutable reference while references into
    // previously visited nodes remain live. Only borrow disjoint payload fields.
    let right = (*node).right;
    let key = &*ptr::addr_of!((*node).key).cast::<T>();
    let data = &mut *ptr::addr_of_mut!((*node).data).cast::<U>();
    result.push((key, data));

    in_order_traverse(right, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn test_native_layout() {
        use std::mem::offset_of;
        type Node = Tll<u32, u64>;
        assert_eq!(offset_of!(Node, left), 0x00);
        assert_eq!(offset_of!(Node, parent), 0x08);
        assert_eq!(offset_of!(Node, right), 0x10);
        assert_eq!(offset_of!(Node, is_black), 0x18);
        assert_eq!(offset_of!(Node, is_sentinel), 0x19);
        assert_eq!(offset_of!(Node, key), 0x20);
        assert_eq!(offset_of!(Node, data), 0x28);
        assert_eq!(size_of::<Node>(), 0x30);
        assert_eq!(offset_of!(TllContainer<u32, u64>, sentinel), 0);
        assert_eq!(offset_of!(TllContainer<u32, u64>, size), 8);
        assert_eq!(size_of::<TllContainer<u32, u64>>(), 0x10);
    }

    #[test]
    #[should_panic(expected = "Tll node alignment exceeds libc malloc alignment")]
    fn test_rejects_overaligned_payloads() {
        #[repr(align(4096))]
        struct Overaligned;
        let _tree = TllContainer::<Overaligned, ()>::new();
    }

    #[test]
    fn test_raw_ownership_round_trip() {
        let mut tree = TllContainer::new();
        tree.insert(1, String::from("owned"));
        let sentinel = tree.sentinel();
        assert_eq!(tree.len(), 1);
        let (pointer, size) = tree.into_raw_parts();
        assert_eq!(pointer, sentinel);
        assert_eq!(size, 1);
        let mut adopted = unsafe { TllContainer::from_raw_parts(pointer, size) };
        assert_eq!(adopted.get_items()[0], "owned");
    }

    #[test]
    fn test_payload_drop_panic_finishes_remaining_tree() {
        use std::{cell::Cell, rc::Rc};

        struct Probe {
            drops: Rc<Cell<usize>>,
            panic: bool,
        }
        impl Drop for Probe {
            fn drop(&mut self) {
                self.drops.set(self.drops.get() + 1);
                assert!(!self.panic, "injected payload drop panic");
            }
        }

        for panic_key in [false, true] {
            for panic_node in 0..3 {
                let drops = Rc::new(Cell::new(0));
                let mut tree = TllContainer::<Probe, Probe>::new();
                unsafe {
                    let sentinel = tree.sentinel();
                    let mut nodes = [sentinel; 3];
                    for (index, node) in nodes.iter_mut().enumerate() {
                        *node = Tll::new(
                            Probe {
                                drops: drops.clone(),
                                panic: panic_key && index == panic_node,
                            },
                            Probe {
                                drops: drops.clone(),
                                panic: !panic_key && index == panic_node,
                            },
                            sentinel,
                        );
                    }
                    let [left, root, right] = nodes;
                    (*root).left = left;
                    (*root).right = right;
                    (*root).is_black = true;
                    (*left).parent = root;
                    (*right).parent = root;
                    (*sentinel).parent = root;
                    (*sentinel).left = left;
                    (*sentinel).right = right;
                    tree.size = 3;
                }
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(tree)));
                assert!(result.is_err());
                assert_eq!(drops.get(), 6, "every key and value must be dropped once");
            }
        }
    }

    #[test]
    fn test_clone_preserves_structure_without_ordering_bounds() {
        #[derive(Clone)]
        struct Payload;

        let empty = TllContainer::<Payload, Payload>::default();
        let cloned = empty.clone();
        assert!(cloned.is_empty());
        assert_ne!(empty.sentinel, cloned.sentinel);

        unsafe fn compare(source: *const Tll<i32, String>, cloned: *const Tll<i32, String>) {
            assert_ne!(source, cloned);
            assert_eq!((*source).is_sentinel, (*cloned).is_sentinel);
            assert_eq!((*source).is_black, (*cloned).is_black);
            if (*source).is_sentinel {
                return;
            }
            assert_eq!((*source).key(), (*cloned).key());
            assert_eq!((*source).data(), (*cloned).data());
            if !(*(*cloned).left).is_sentinel {
                assert_eq!((*(*cloned).left).parent, cloned.cast_mut());
            }
            if !(*(*cloned).right).is_sentinel {
                assert_eq!((*(*cloned).right).parent, cloned.cast_mut());
            }
            compare((*source).left, (*cloned).left);
            compare((*source).right, (*cloned).right);
        }

        let mut source = TllContainer::new();
        for key in [50, 30, 70, 20, 40, 60, 80, 10, 25] {
            source.insert(key, key.to_string());
        }
        let mut cloned = source.clone();
        assert_eq!(source.len(), cloned.len());
        unsafe {
            compare((*source.sentinel).parent, (*cloned.sentinel).parent);
        }
        for value in cloned.get_items() {
            value.push('!');
        }
        assert!(source.get_items().iter().all(|value| !value.ends_with('!')));
        drop(source);
        assert_eq!(cloned.get_items().len(), 9);
    }

    #[test]
    fn test_clone_panic_reclaims_completed_payloads() {
        use std::{cell::Cell, rc::Rc};

        struct Tracked {
            live: Rc<Cell<usize>>,
            budget: Rc<Cell<usize>>,
        }
        impl Clone for Tracked {
            fn clone(&self) -> Self {
                let budget = self.budget.get();
                assert!(budget > 0, "injected clone panic");
                self.budget.set(budget - 1);
                self.live.set(self.live.get() + 1);
                Self {
                    live: self.live.clone(),
                    budget: self.budget.clone(),
                }
            }
        }
        impl Drop for Tracked {
            fn drop(&mut self) {
                self.live.set(self.live.get() - 1);
            }
        }

        let live = Rc::new(Cell::new(0));
        let budget = Rc::new(Cell::new(0));
        let mut tree = TllContainer::new();
        for key in [4, 2, 6, 1, 3, 5, 7] {
            live.set(live.get() + 1);
            tree.insert(
                key,
                Tracked {
                    live: live.clone(),
                    budget: budget.clone(),
                },
            );
        }
        for successful_clones in 0..7 {
            budget.set(successful_clones);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tree.clone()));
            assert!(result.is_err());
            assert_eq!(live.get(), 7, "partial clones must be reclaimed");
            assert_eq!(tree.get_items().len(), 7);
        }
        budget.set(7);
        let cloned = tree.clone();
        assert_eq!(live.get(), 14);
        drop(tree);
        assert_eq!(live.get(), 7);
        drop(cloned);
        assert_eq!(live.get(), 0);
    }

    #[test]
    fn test_traversal_borrows_disjoint_payloads() {
        let mut tree = TllContainer::new();
        for key in [2, 1, 3] {
            tree.insert(key, key * 10);
        }
        let mut values = tree.get_items();
        *values[0] += 1;
        *values[1] += 2;
        *values[2] += 3;
        assert_eq!(
            values.into_iter().map(|v| *v).collect::<Vec<_>>(),
            [11, 22, 33]
        );
        let mut map = tree.get_map();
        **map.get_mut(&2).unwrap() += 100;
        assert_eq!(
            map.into_iter()
                .map(|(key, value)| (*key, *value))
                .collect::<std::collections::BTreeMap<_, _>>(),
            [(1, 11), (2, 122), (3, 33)].into_iter().collect()
        );
    }

    // Helper function to create a test node
    unsafe fn create_node(
        key: i32,
        data: &str,
        sentinel: *mut Tll<i32, String>,
    ) -> *mut Tll<i32, String> {
        Tll::new(key, data.to_string(), sentinel)
    }

    #[test]
    fn test_insert_multiple_nodes() {
        let mut container = TllContainer::<i32, String>::new();

        container.insert(50, "fifty".to_string());
        container.insert(30, "thirty".to_string());
        container.insert(70, "seventy".to_string());
        container.insert(20, "twenty".to_string());
        container.insert(40, "forty".to_string());

        assert_eq!(container.len(), 5);

        // Verify BST property by checking in-order traversal
        let items = container.get_items();
        let values: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
        assert_eq!(
            values,
            vec!["twenty", "thirty", "forty", "fifty", "seventy"]
        );
    }

    #[test]
    fn test_insert_duplicate_key() {
        let mut container = TllContainer::<i32, String>::new();

        container.insert(50, "fifty".to_string());
        container.insert(50, "FIFTY_UPDATED".to_string());

        assert_eq!(container.len(), 1);
        let items = container.get_items();
        assert_eq!(items[0].as_str(), "FIFTY_UPDATED");
    }

    #[test]
    fn test_rotate_left_non_root() {
        unsafe {
            let container = TllContainer::<i32, String>::new();
            let sentinel = container.sentinel;

            // Build tree:
            //       50 (root)
            //      /
            //     30
            //    /  \
            //   20   40
            //       /
            //      35

            let root = create_node(50, "fifty", sentinel);
            let n30 = create_node(30, "thirty", sentinel);
            let n20 = create_node(20, "twenty", sentinel);
            let n40 = create_node(40, "forty", sentinel);
            let n35 = create_node(35, "thirty-five", sentinel);

            // Setup tree structure
            (*sentinel).parent = root;
            (*root).left = n30;
            (*n30).parent = root;
            (*n30).left = n20;
            (*n30).right = n40;
            (*n20).parent = n30;
            (*n40).parent = n30;
            (*n40).left = n35;
            (*n35).parent = n40;

            // Rotate left on 30
            // Expected result:
            //       50 (root)
            //      /
            //     40
            //    /  \
            //   30   (sentinel)
            //  /  \
            // 20   35

            let new_subtree_root = (*n30).rotate_left(sentinel);

            // Verify the rotation
            assert_eq!(new_subtree_root, n40, "40 should be new subtree root");
            assert_eq!((*n40).parent, root, "40's parent should be 50");
            assert_eq!((*root).left, n40, "50's left child should be 40");
            assert_eq!((*n40).left, n30, "40's left child should be 30");
            assert_eq!((*n30).parent, n40, "30's parent should be 40");
            assert_eq!((*n30).right, n35, "30's right child should be 35");
            assert_eq!((*n35).parent, n30, "35's parent should be 30");
            assert_eq!((*n30).left, n20, "30's left child should still be 20");
            assert_eq!((*n20).parent, n30, "20's parent should still be 30");

            // Sentinel root should NOT have changed
            assert_eq!((*sentinel).parent, root, "Root should still be 50");

            // The owned container reclaims every node and its String payload.
            drop(container);
        }
    }

    #[test]
    fn test_rotate_left_root() {
        unsafe {
            let container = TllContainer::<i32, String>::new();
            let sentinel = container.sentinel;

            // Build tree:
            //     30 (root)
            //    /  \
            //   20   40
            //       /  \
            //      35   50

            let root = create_node(30, "thirty", sentinel);
            let n20 = create_node(20, "twenty", sentinel);
            let n40 = create_node(40, "forty", sentinel);
            let n35 = create_node(35, "thirty-five", sentinel);
            let n50 = create_node(50, "fifty", sentinel);

            (*sentinel).parent = root;
            (*root).left = n20;
            (*root).right = n40;
            (*n20).parent = root;
            (*n40).parent = root;
            (*n40).left = n35;
            (*n40).right = n50;
            (*n35).parent = n40;
            (*n50).parent = n40;

            // Rotate left on root
            // Expected:
            //     40 (new root)
            //    /  \
            //   30   50
            //  /  \
            // 20  35

            let new_root = (*root).rotate_left(sentinel);

            assert_eq!(new_root, n40, "40 should be new root");
            assert_eq!((*sentinel).parent, n40, "Sentinel should point to new root");
            assert_eq!(
                (*n40).parent,
                sentinel,
                "New root's parent should be sentinel"
            );
            assert_eq!((*n40).left, root, "40's left should be old root (30)");
            assert_eq!((*root).parent, n40, "30's parent should be 40");
            assert_eq!((*root).right, n35, "30's right should be 35");
            assert_eq!((*n35).parent, root, "35's parent should be 30");

            // The owned container reclaims every node and its String payload.
            drop(container);
        }
    }

    #[test]
    fn test_rotate_right_non_root() {
        unsafe {
            let container = TllContainer::<i32, String>::new();
            let sentinel = container.sentinel;

            // Build tree:
            //       50 (root)
            //      /
            //     40
            //    /  \
            //   30   (sentinel)
            //  /  \
            // 20   35

            let root = create_node(50, "fifty", sentinel);
            let n40 = create_node(40, "forty", sentinel);
            let n30 = create_node(30, "thirty", sentinel);
            let n20 = create_node(20, "twenty", sentinel);
            let n35 = create_node(35, "thirty-five", sentinel);

            (*sentinel).parent = root;
            (*root).left = n40;
            (*n40).parent = root;
            (*n40).left = n30;
            (*n30).parent = n40;
            (*n30).left = n20;
            (*n30).right = n35;
            (*n20).parent = n30;
            (*n35).parent = n30;

            // Rotate right on 40
            // Expected:
            //       50 (root)
            //      /
            //     30
            //    /  \
            //   20   40
            //       /
            //      35

            let new_subtree_root = (*n40).rotate_right(sentinel);

            assert_eq!(new_subtree_root, n30, "30 should be new subtree root");
            assert_eq!((*n30).parent, root, "30's parent should be 50");
            assert_eq!((*root).left, n30, "50's left child should be 30");
            assert_eq!((*n30).right, n40, "30's right child should be 40");
            assert_eq!((*n40).parent, n30, "40's parent should be 30");
            assert_eq!((*n40).left, n35, "40's left child should be 35");
            assert_eq!((*n35).parent, n40, "35's parent should be 40");

            // The owned container reclaims every node and its String payload.
            drop(container);
        }
    }

    #[test]
    fn test_rotate_right_root() {
        unsafe {
            let container = TllContainer::<i32, String>::new();
            let sentinel = container.sentinel;

            // Build tree:
            //       40 (root)
            //      /  \
            //     30   50
            //    /  \
            //   20   35

            let root = create_node(40, "forty", sentinel);
            let n30 = create_node(30, "thirty", sentinel);
            let n50 = create_node(50, "fifty", sentinel);
            let n20 = create_node(20, "twenty", sentinel);
            let n35 = create_node(35, "thirty-five", sentinel);

            (*sentinel).parent = root;
            (*root).left = n30;
            (*root).right = n50;
            (*n30).parent = root;
            (*n50).parent = root;
            (*n30).left = n20;
            (*n30).right = n35;
            (*n20).parent = n30;
            (*n35).parent = n30;

            // Rotate right on root
            // Expected:
            //       30 (new root)
            //      /  \
            //     20   40
            //         /  \
            //        35   50

            let new_root = (*root).rotate_right(sentinel);

            assert_eq!(new_root, n30, "30 should be new root");
            assert_eq!((*sentinel).parent, n30, "Sentinel should point to new root");
            assert_eq!(
                (*n30).parent,
                sentinel,
                "New root's parent should be sentinel"
            );
            assert_eq!((*n30).right, root, "30's right should be old root (40)");
            assert_eq!((*root).parent, n30, "40's parent should be 30");
            assert_eq!((*root).left, n35, "40's left should be 35");
            assert_eq!((*n35).parent, root, "35's parent should be 40");

            // The owned container reclaims every node and its String payload.
            drop(container);
        }
    }

    #[test]
    fn test_rotate_left_with_sentinel_child() {
        unsafe {
            let container = TllContainer::<i32, String>::new();
            let sentinel = container.sentinel;

            // Build simple tree:
            //     30 (root)
            //    /  \
            //   20   40 (no children)

            let root = create_node(30, "thirty", sentinel);
            let n20 = create_node(20, "twenty", sentinel);
            let n40 = create_node(40, "forty", sentinel);

            (*sentinel).parent = root;
            (*root).left = n20;
            (*root).right = n40;
            (*n20).parent = root;
            (*n40).parent = root;

            // Rotate left on root
            // 40 has no left child (sentinel), so 30's right should become sentinel

            let new_root = (*root).rotate_left(sentinel);

            assert_eq!(new_root, n40);
            assert_eq!((*sentinel).parent, n40, "Sentinel should point to new root");
            assert!(
                (*root).right.is_null() || (*(*root).right).is_sentinel,
                "30's right should be sentinel"
            );

            // The owned container reclaims every node and its String payload.
            drop(container);
        }
    }

    #[test]
    fn test_rotate_right_with_sentinel_child() {
        unsafe {
            let container = TllContainer::<i32, String>::new();
            let sentinel = container.sentinel;

            // Build simple tree:
            //     40 (root)
            //    /  \
            //   30   50 (no children)

            let root = create_node(40, "forty", sentinel);
            let n30 = create_node(30, "thirty", sentinel);
            let n50 = create_node(50, "fifty", sentinel);

            (*sentinel).parent = root;
            (*root).left = n30;
            (*root).right = n50;
            (*n30).parent = root;
            (*n50).parent = root;

            // Rotate right on root
            // 30 has no right child (sentinel), so 40's left should become sentinel

            let new_root = (*root).rotate_right(sentinel);

            assert_eq!(new_root, n30);
            assert_eq!((*sentinel).parent, n30, "Sentinel should point to new root");
            assert!(
                (*root).left.is_null() || (*(*root).left).is_sentinel,
                "40's left should be sentinel"
            );

            // The owned container reclaims every node and its String payload.
            drop(container);
        }
    }

    // Helper function to verify red-black tree properties
    unsafe fn verify_rb_properties<T: Ord, U>(node: *const Tll<T, U>) -> (bool, usize) {
        // Property 1: Root must be black
        if (*node).is_sentinel {
            return (true, 1);
        }

        // Property 2: Red nodes must have black children
        if !(*node).is_black {
            if !(*(*node).left).is_sentinel && !(*(*node).left).is_black {
                return (false, 0);
            }
            if !(*(*node).right).is_sentinel && !(*(*node).right).is_black {
                return (false, 0);
            }
        }

        // Property 3: All paths must have same black height
        let (left_valid, left_black_height) = verify_rb_properties((*node).left);
        let (right_valid, right_black_height) = verify_rb_properties((*node).right);

        if !left_valid || !right_valid {
            return (false, 0);
        }

        if left_black_height != right_black_height {
            return (false, 0);
        }

        let black_height = left_black_height + if (*node).is_black { 1 } else { 0 };
        (true, black_height)
    }

    #[test]
    fn test_insert_root_is_black() {
        let mut container = TllContainer::<i32, String>::new();
        container.insert(50, "fifty".to_string());

        unsafe {
            let root = (*container.sentinel).parent;
            assert!((*root).is_black, "Root must be black");
            assert_eq!(container.len(), 1);
        }
    }

    #[test]
    fn test_insert_maintains_bst_property() {
        let mut container = TllContainer::<i32, String>::new();

        // Insert in random order
        let values = vec![50, 25, 75, 10, 30, 60, 80, 5, 15, 27, 35];
        for &val in &values {
            container.insert(val, format!("value_{}", val));
        }

        // Verify in-order traversal is sorted
        let mut prev = i32::MIN;

        unsafe {
            let sentinel = container.sentinel;
            let root = (*sentinel).parent;

            let mut result = Vec::new();
            in_order_traverse(root, &mut result);

            for (key, _) in result {
                assert!(
                    *key > prev,
                    "BST property violated: {} should be > {}",
                    key,
                    prev
                );
                prev = *key;
            }
        }
    }

    #[test]
    fn test_insert_maintains_rb_properties() {
        let mut container = TllContainer::<i32, String>::new();

        // Insert values that will trigger various rebalancing cases
        let values = vec![50, 25, 75, 10, 30, 60, 80];
        for &val in &values {
            container.insert(val, format!("value_{}", val));
        }

        unsafe {
            let sentinel = container.sentinel;
            let root = (*sentinel).parent;

            // Verify root is black
            assert!((*root).is_black, "Root must be black");

            // Verify all red-black properties
            let (valid, _) = verify_rb_properties(root);
            assert!(valid, "Red-black tree properties violated");
        }
    }

    #[test]
    fn test_insert_case_uncle_red_recoloring() {
        let mut container = TllContainer::<i32, String>::new();

        // This sequence triggers uncle-is-red case (recoloring)
        container.insert(50, "fifty".to_string());
        container.insert(25, "twenty-five".to_string());
        container.insert(75, "seventy-five".to_string());
        container.insert(10, "ten".to_string()); // This triggers recoloring

        unsafe {
            let sentinel = container.sentinel;
            let root = (*sentinel).parent;

            // Root must be black
            assert!((*root).is_black, "Root must be black");

            // Verify properties maintained
            let (valid, _) = verify_rb_properties(root);
            assert!(valid, "Red-black tree properties violated after recoloring");
        }

        assert_eq!(container.len(), 4);
    }

    #[test]
    fn test_insert_case_left_left_rotation() {
        let mut container = TllContainer::<i32, String>::new();

        // This sequence triggers left-left case (right rotation on grandparent)
        container.insert(50, "fifty".to_string());
        container.insert(30, "thirty".to_string());
        container.insert(20, "twenty".to_string()); // Triggers left-left case

        unsafe {
            let sentinel = container.sentinel;
            let root = (*sentinel).parent;

            // Verify properties
            let (valid, _) = verify_rb_properties(root);
            assert!(
                valid,
                "Red-black tree properties violated after left-left rotation"
            );

            // Root should be black
            assert!((*root).is_black, "Root must be black");
        }

        assert_eq!(container.len(), 3);
    }

    #[test]
    fn test_insert_case_left_right_rotation() {
        let mut container = TllContainer::<i32, String>::new();

        // This sequence triggers left-right case (left rotation on parent, then right on grandparent)
        container.insert(50, "fifty".to_string());
        container.insert(30, "thirty".to_string());
        container.insert(40, "forty".to_string()); // Triggers left-right case

        unsafe {
            let sentinel = container.sentinel;
            let root = (*sentinel).parent;

            // Verify properties
            let (valid, _) = verify_rb_properties(root);
            assert!(
                valid,
                "Red-black tree properties violated after left-right rotation"
            );

            // Root should be black
            assert!((*root).is_black, "Root must be black");
        }

        assert_eq!(container.len(), 3);
    }

    #[test]
    fn test_insert_case_right_right_rotation() {
        let mut container = TllContainer::<i32, String>::new();

        // This sequence triggers right-right case (left rotation on grandparent)
        container.insert(50, "fifty".to_string());
        container.insert(70, "seventy".to_string());
        container.insert(80, "eighty".to_string()); // Triggers right-right case

        unsafe {
            let sentinel = container.sentinel;
            let root = (*sentinel).parent;

            // Verify properties
            let (valid, _) = verify_rb_properties(root);
            assert!(
                valid,
                "Red-black tree properties violated after right-right rotation"
            );

            // Root should be black
            assert!((*root).is_black, "Root must be black");
        }

        assert_eq!(container.len(), 3);
    }

    #[test]
    fn test_insert_case_right_left_rotation() {
        let mut container = TllContainer::<i32, String>::new();

        // This sequence triggers right-left case (right rotation on parent, then left on grandparent)
        container.insert(50, "fifty".to_string());
        container.insert(70, "seventy".to_string());
        container.insert(60, "sixty".to_string()); // Triggers right-left case

        unsafe {
            let sentinel = container.sentinel;
            let root = (*sentinel).parent;

            // Verify properties
            let (valid, _) = verify_rb_properties(root);
            assert!(
                valid,
                "Red-black tree properties violated after right-left rotation"
            );

            // Root should be black
            assert!((*root).is_black, "Root must be black");
        }

        assert_eq!(container.len(), 3);
    }

    #[test]
    fn test_insert_sequential_ascending() {
        let mut container = TllContainer::<i32, String>::new();

        // Insert in ascending order (worst case for unbalanced BST)
        for i in 1..=10 {
            container.insert(i, format!("value_{}", i));
        }

        unsafe {
            let sentinel = container.sentinel;
            let root = (*sentinel).parent;

            // Verify properties are maintained even with sequential insertion
            let (valid, _) = verify_rb_properties(root);
            assert!(
                valid,
                "Red-black tree properties violated after sequential insertion"
            );

            // Root should be black
            assert!((*root).is_black, "Root must be black");
        }

        assert_eq!(container.len(), 10);

        // Verify BST property
        let items = container.get_items();
        assert_eq!(items.len(), 10);
    }

    #[test]
    fn test_insert_sequential_descending() {
        let mut container = TllContainer::<i32, String>::new();

        // Insert in descending order
        for i in (1..=10).rev() {
            container.insert(i, format!("value_{}", i));
        }

        unsafe {
            let sentinel = container.sentinel;
            let root = (*sentinel).parent;

            // Verify properties
            let (valid, _) = verify_rb_properties(root);
            assert!(
                valid,
                "Red-black tree properties violated after descending insertion"
            );

            // Root should be black
            assert!((*root).is_black, "Root must be black");
        }

        assert_eq!(container.len(), 10);
    }

    #[test]
    fn test_insert_alternating_pattern() {
        let mut container = TllContainer::<i32, String>::new();

        // Insert in alternating pattern to trigger various cases
        let values = vec![50, 30, 70, 20, 40, 60, 80, 10, 25, 35, 45];
        for &val in &values {
            container.insert(val, format!("value_{}", val));

            unsafe {
                let sentinel = container.sentinel;
                let root = (*sentinel).parent;

                // After each insertion, verify properties hold
                let (valid, _) = verify_rb_properties(root);
                assert!(
                    valid,
                    "Red-black tree properties violated after inserting {}",
                    val
                );

                // Root must always be black
                assert!(
                    (*root).is_black,
                    "Root must be black after inserting {}",
                    val
                );
            }
        }

        assert_eq!(container.len(), values.len());

        // Verify final tree structure
        let items = container.get_items();
        assert_eq!(items.len(), values.len());
    }

    #[test]
    fn test_insert_with_updates() {
        let mut container = TllContainer::<i32, String>::new();

        // Insert initial values
        container.insert(50, "fifty_v1".to_string());
        container.insert(30, "thirty_v1".to_string());
        container.insert(70, "seventy_v1".to_string());

        // Update existing values
        container.insert(50, "fifty_v2".to_string());
        container.insert(30, "thirty_v2".to_string());

        // Size should not change on updates
        assert_eq!(container.len(), 3);

        // Verify updated values
        let map = container.get_map();
        assert_eq!(map.get(&50).unwrap().as_str(), "fifty_v2");
        assert_eq!(map.get(&30).unwrap().as_str(), "thirty_v2");
        assert_eq!(map.get(&70).unwrap().as_str(), "seventy_v1");

        unsafe {
            let sentinel = container.sentinel;
            let root = (*sentinel).parent;

            // Properties should still be valid
            let (valid, _) = verify_rb_properties(root);
            assert!(valid, "Red-black tree properties violated after updates");
        }
    }

    #[test]
    fn test_sentinel_min_max_single_node() {
        let mut container = TllContainer::<i32, String>::new();
        container.insert(50, "fifty".to_string());

        unsafe {
            let sentinel = container.sentinel;
            let min_node = (*sentinel).left;
            let max_node = (*sentinel).right;

            // With one node, min and max should be the same
            assert_eq!(
                min_node, max_node,
                "Min and max should be same for single node"
            );
            assert_eq!((*min_node).key.assume_init(), 50, "Min should be 50");
            assert_eq!((*max_node).key.assume_init(), 50, "Max should be 50");
        }
    }

    #[test]
    fn test_sentinel_min_max_ascending_insertion() {
        let mut container = TllContainer::<i32, String>::new();

        // Insert in ascending order
        for i in 1..=10 {
            container.insert(i, format!("value_{}", i));

            unsafe {
                let sentinel = container.sentinel;
                let min_node = (*sentinel).left;
                let max_node = (*sentinel).right;

                // Min should always be 1
                assert_eq!(
                    (*min_node).key.assume_init(),
                    1,
                    "Min should be 1 after inserting {}",
                    i
                );

                // Max should be the largest value inserted so far
                assert_eq!(
                    (*max_node).key.assume_init(),
                    i,
                    "Max should be {} after inserting {}",
                    i,
                    i
                );
            }
        }
    }

    #[test]
    fn test_sentinel_min_max_descending_insertion() {
        let mut container = TllContainer::<i32, String>::new();

        // Insert in descending order
        for i in (1..=10).rev() {
            container.insert(i, format!("value_{}", i));

            unsafe {
                let sentinel = container.sentinel;
                let min_node = (*sentinel).left;
                let max_node = (*sentinel).right;

                // Min should be the smallest value inserted so far
                assert_eq!(
                    (*min_node).key.assume_init(),
                    i,
                    "Min should be {} after inserting {}",
                    i,
                    i
                );

                // Max should always be 10
                assert_eq!(
                    (*max_node).key.assume_init(),
                    10,
                    "Max should be 10 after inserting {}",
                    i
                );
            }
        }
    }

    #[test]
    fn test_sentinel_min_max_random_insertion() {
        let mut container = TllContainer::<i32, String>::new();

        // Insert in random order
        let values = vec![50, 25, 75, 10, 30, 60, 80, 5, 15, 70, 90];
        let mut current_min = i32::MAX;
        let mut current_max = i32::MIN;

        for &val in &values {
            container.insert(val, format!("value_{}", val));
            current_min = current_min.min(val);
            current_max = current_max.max(val);

            unsafe {
                let sentinel = container.sentinel;
                let min_node = (*sentinel).left;
                let max_node = (*sentinel).right;

                assert_eq!(
                    (*min_node).key.assume_init(),
                    current_min,
                    "Min should be {} after inserting {}",
                    current_min,
                    val
                );

                assert_eq!(
                    (*max_node).key.assume_init(),
                    current_max,
                    "Max should be {} after inserting {}",
                    current_max,
                    val
                );
            }
        }
    }

    #[test]
    fn test_sentinel_min_max_after_rotations() {
        let mut container = TllContainer::<i32, String>::new();

        // This insertion pattern will trigger rotations
        // but min/max should still be maintained correctly
        container.insert(50, "fifty".to_string());
        container.insert(25, "twenty-five".to_string());
        container.insert(75, "seventy-five".to_string());
        container.insert(10, "ten".to_string());
        container.insert(30, "thirty".to_string());
        container.insert(60, "sixty".to_string());
        container.insert(80, "eighty".to_string());
        container.insert(5, "five".to_string()); // New min
        container.insert(90, "ninety".to_string()); // New max

        unsafe {
            let sentinel = container.sentinel;
            let min_node = (*sentinel).left;
            let max_node = (*sentinel).right;

            assert_eq!((*min_node).key.assume_init(), 5, "Min should be 5");
            assert_eq!((*max_node).key.assume_init(), 90, "Max should be 90");

            // Verify min node has no left child
            assert!(
                (*(*min_node).left).is_sentinel,
                "Min node should have no left child"
            );

            // Verify max node has no right child
            assert!(
                (*(*max_node).right).is_sentinel,
                "Max node should have no right child"
            );
        }
    }

    #[test]
    fn test_sentinel_min_max_with_duplicates() {
        let mut container = TllContainer::<i32, String>::new();

        container.insert(50, "fifty".to_string());
        container.insert(25, "twenty-five".to_string());
        container.insert(75, "seventy-five".to_string());

        unsafe {
            let sentinel = container.sentinel;
            let min_before = (*sentinel).left;
            let max_before = (*sentinel).right;

            // Update existing values (no structural change)
            container.insert(50, "FIFTY_UPDATED".to_string());
            container.insert(25, "TWENTY_FIVE_UPDATED".to_string());

            let min_after = (*sentinel).left;
            let max_after = (*sentinel).right;

            // Min and max pointers should not change
            assert_eq!(
                min_before, min_after,
                "Min pointer should not change on update"
            );
            assert_eq!(
                max_before, max_after,
                "Max pointer should not change on update"
            );

            assert_eq!((*min_after).key.assume_init(), 25, "Min should still be 25");
            assert_eq!((*max_after).key.assume_init(), 75, "Max should still be 75");
        }
    }

    #[test]
    fn test_sentinel_min_max_comprehensive() {
        let mut container = TllContainer::<i32, String>::new();

        // Build a tree and verify min/max at each step
        let insertions = vec![
            (50, 50, 50), // (value, expected_min, expected_max)
            (30, 30, 50),
            (70, 30, 70),
            (20, 20, 70),
            (40, 20, 70),
            (60, 20, 70),
            (80, 20, 80),
            (10, 10, 80),
            (90, 10, 90),
        ];

        for (value, expected_min, expected_max) in insertions {
            container.insert(value, format!("value_{}", value));

            unsafe {
                let sentinel = container.sentinel;
                let min_node = (*sentinel).left;
                let max_node = (*sentinel).right;

                assert_eq!(
                    (*min_node).key.assume_init(),
                    expected_min,
                    "After inserting {}, min should be {}",
                    value,
                    expected_min
                );

                assert_eq!(
                    (*max_node).key.assume_init(),
                    expected_max,
                    "After inserting {}, max should be {}",
                    value,
                    expected_max
                );

                // Verify min has no left child
                assert!(
                    (*(*min_node).left).is_sentinel,
                    "Min node should have no left child after inserting {}",
                    value
                );

                // Verify max has no right child
                assert!(
                    (*(*max_node).right).is_sentinel,
                    "Max node should have no right child after inserting {}",
                    value
                );
            }
        }
    }
    #[test]
    fn test_game_loadout_tree_structure() {
        // Recreate the exact tree structure from the game memory dump
        // This tests inserting the 10 loadouts in an order that matches the game
        let mut container = TllContainer::<String, String>::new();

        // Insert loadouts in the order they appear in the game
        let loadouts = vec![
            "LOADOUT_LA29_FAB100",
            "LOADOUT_LA29_FAB250",
            "LOADOUT_LA29_GUN37",
            "LOADOUT_LA29_NURS122",
            "LOADOUT_T7_FAB100",
            "LOADOUT_T7_FAB250",
            "LOADOUT_T7_GUN37",
            "LOADOUT_T7_K13",
            "LOADOUT_T7_NURS122",
            "LOADOUT_T7_NURS340",
        ];

        for loadout in &loadouts {
            container.insert(loadout.to_string(), format!("data_{}", loadout));
        }

        assert_eq!(container.len(), 10);

        unsafe {
            let sentinel = container.sentinel;
            let root = (*sentinel).parent;

            // Verify root is black (unlike the game dump which shows is_black=false)
            assert!((*root).is_black, "Root must be black");

            // Verify red-black properties
            let (valid, _) = verify_rb_properties(root);
            assert!(valid, "Tree must maintain red-black properties");

            // Verify BST ordering
            let items = container.get_items();
            assert_eq!(items.len(), 10);
        }
    }

    #[test]
    fn test_insert_loadout_la29_gun40() {
        // Test inserting LOADOUT_LA29_GUN40 into a tree with the game's loadouts
        // This is the specific case that was causing the panic
        let mut container = TllContainer::<String, String>::new();

        // Build the game tree
        let loadouts = vec![
            "LOADOUT_LA29_FAB100",
            "LOADOUT_LA29_FAB250",
            "LOADOUT_LA29_GUN37",
            "LOADOUT_LA29_NURS122",
            "LOADOUT_T7_FAB100",
            "LOADOUT_T7_FAB250",
            "LOADOUT_T7_GUN37",
            "LOADOUT_T7_K13",
            "LOADOUT_T7_NURS122",
            "LOADOUT_T7_NURS340",
        ];

        for loadout in &loadouts {
            container.insert(loadout.to_string(), format!("data_{}", loadout));
        }

        // Now insert the problematic loadout
        container.insert(
            "LOADOUT_LA29_GUN40".to_string(),
            "custom_loadout".to_string(),
        );

        assert_eq!(
            container.len(),
            11,
            "Should have 11 loadouts after insertion"
        );

        unsafe {
            let sentinel = container.sentinel;
            let root = (*sentinel).parent;

            // Verify root is still black
            assert!((*root).is_black, "Root must remain black after insertion");

            // Verify red-black properties are maintained
            let (valid, _) = verify_rb_properties(root);
            assert!(
                valid,
                "Red-black properties must be maintained after inserting LOADOUT_LA29_GUN40"
            );

            // Verify the new loadout is in the tree
            let map = container.get_map();
            assert!(
                map.contains_key(&"LOADOUT_LA29_GUN40".to_string()),
                "New loadout should be in the tree"
            );

            // Verify BST ordering is maintained
            let mut result = Vec::new();
            in_order_traverse(root, &mut result);
            let keys: Vec<String> = result.iter().map(|(k, _)| (*k).clone()).collect();

            // Check that keys are in sorted order
            let mut sorted_keys = keys.clone();
            sorted_keys.sort();
            assert_eq!(keys, sorted_keys, "Keys should be in sorted order");

            // Verify LOADOUT_LA29_GUN40 is between GUN37 and NURS122
            let gun40_pos = keys.iter().position(|k| k == "LOADOUT_LA29_GUN40").unwrap();
            let gun37_pos = keys.iter().position(|k| k == "LOADOUT_LA29_GUN37").unwrap();
            let nurs122_pos = keys
                .iter()
                .position(|k| k == "LOADOUT_LA29_NURS122")
                .unwrap();

            assert!(
                gun37_pos < gun40_pos && gun40_pos < nurs122_pos,
                "GUN40 should be between GUN37 and NURS122"
            );
        }
    }

    #[test]
    fn test_all_game_loadouts_sorted() {
        // Verify that all game loadouts maintain sorted order
        let mut container = TllContainer::<String, String>::new();

        let loadouts = vec![
            "LOADOUT_LA29_FAB100",
            "LOADOUT_LA29_FAB250",
            "LOADOUT_LA29_GUN37",
            "LOADOUT_LA29_NURS122",
            "LOADOUT_T7_FAB100",
            "LOADOUT_T7_FAB250",
            "LOADOUT_T7_GUN37",
            "LOADOUT_T7_K13",
            "LOADOUT_T7_NURS122",
            "LOADOUT_T7_NURS340",
        ];

        for loadout in &loadouts {
            container.insert(loadout.to_string(), format!("data_{}", loadout));
        }

        // Get all items in order
        unsafe {
            let sentinel = container.sentinel;
            let root = (*sentinel).parent;
            let mut result = Vec::new();
            in_order_traverse(root, &mut result);

            let keys: Vec<&str> = result.iter().map(|(k, _)| k.as_str()).collect();

            // Expected order (alphabetically sorted)
            let expected = vec![
                "LOADOUT_LA29_FAB100",
                "LOADOUT_LA29_FAB250",
                "LOADOUT_LA29_GUN37",
                "LOADOUT_LA29_NURS122",
                "LOADOUT_T7_FAB100",
                "LOADOUT_T7_FAB250",
                "LOADOUT_T7_GUN37",
                "LOADOUT_T7_K13",
                "LOADOUT_T7_NURS122",
                "LOADOUT_T7_NURS340",
            ];

            assert_eq!(keys, expected, "Loadouts should be in alphabetical order");
        }
    }
}
