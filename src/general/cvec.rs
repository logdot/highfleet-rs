//! Owned, libc-allocated vectors with HighFleet's native three-pointer layout.
//!
//! Rust-constructed vectors own their allocation and initialized elements.
//! Borrow game-owned headers through raw pointers or references; borrowing does not run their destructor.
//! Do not use `ptr::read` to create a second owner of a borrowed native vector.
//! Mutation and destruction require storage compatible with this process's libc allocator.
//! Raw pointer elements do not imply ownership of their targets.
//! See [`CVec`] for access, layout, cleanup, and ownership-transfer requirements.

use serde::{Deserialize, Serialize};
use std::{
    alloc::{handle_alloc_error, Layout},
    mem::{align_of, size_of, ManuallyDrop},
    ptr,
};

/// An owned vector represented by start, initialized-end, and reserved-end pointers.
///
/// The C representation preserves their order and layout; on 64-bit targets this is 24 bytes with 8-byte alignment.
/// Dropping a vector destroys its initialized elements and frees its backing allocation.
/// Cloning allocates independent storage and clones each initialized element.
/// Raw pointer targets are not recursively cloned or destroyed.
/// Do not manually free storage still owned by a vector.
///
/// # Access
///
/// The header fields are private to prevent safe code from fabricating invalid owners without changing the native layout.
/// Use [`Self::as_slice`] to borrow initialized elements or [`Self::items`] to collect references into a Rust vector.
/// [`Self::as_ptr`] and [`Self::as_mut_ptr`] provide non-owning raw access.
/// [`Self::len`] and [`Self::capacity`] report the initialized element count and allocation capacity.
/// Construct vectors with [`Self::empty`] and append elements with [`Self::insert`].
///
/// # Native interoperability
///
/// Borrow game-owned headers rather than adopting their storage.
/// Native storage must remain live and valid, with no conflicting access while Rust references exist.
/// Mutating a borrowed native vector additionally requires exclusive access and libc-compatible storage.
/// [`Self::into_raw_parts`] transfers ownership without destruction.
/// Unsafe [`Self::from_raw_parts`] adopts exclusive ownership of compatible storage.
/// Moving an owned header into native storage with [`ptr::write`] transfers it without reclaiming the destination's previous contents.
///
/// # Invariants
///
/// The pointers are either all null or belong to one exclusively owned libc allocation.
/// Non-null pointers are ordered start <= initialized-end <= reserved-end, with reserved-end one past the allocation's element capacity.
/// Only elements before initialized-end are initialized Rust values.
/// Capacity in bytes must not exceed `isize::MAX`.
/// Zero-sized elements and alignment beyond libc's malloc guarantee are unsupported.
#[repr(C)]
#[derive(Debug)]
pub struct CVec<T> {
    /// Start of the owned allocation, or null for an unallocated vector.
    items: *const T,
    /// End of the initialized elements.
    items_end: *const T,
    /// End of the allocation's element capacity.
    items_rsv_end: *const T,
}

impl<T> CVec<T> {
    /// Creates an empty vector with all three pointers null and no allocation.
    ///
    /// # Panics
    ///
    /// Panics for zero-sized or unsupported over-aligned element types.
    pub fn empty() -> Self {
        assert_supported_element::<T>();
        Self {
            items: ptr::null(),
            items_end: ptr::null(),
            items_rsv_end: ptr::null(),
        }
    }

    /// Returns a non-owning pointer to the allocation, null when unallocated.
    ///
    /// The pointer may be invalidated by growth or destruction.
    pub fn as_ptr(&self) -> *const T {
        self.items
    }

    /// Returns a non-owning mutable pointer to the allocation, null when unallocated.
    ///
    /// Writing through it must preserve initialization and ownership invariants and respect outstanding borrows.
    /// The pointer may be invalidated by growth or destruction.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.items.cast_mut()
    }

    /// Transfers the allocation and initialized elements without destroying them.
    ///
    /// Returns start, initialized-end, and reserved-end pointers in native order.
    /// The caller must arrange cleanup or transfer ownership elsewhere.
    pub fn into_raw_parts(self) -> (*mut T, *mut T, *mut T) {
        let this = ManuallyDrop::new(self);
        (
            this.items.cast_mut(),
            this.items_end.cast_mut(),
            this.items_rsv_end.cast_mut(),
        )
    }

    /// Adopts exclusive ownership of a native three-pointer vector.
    ///
    /// # Safety
    ///
    /// Pointers must be all null or describe a single live libc malloc/realloc-compatible allocation with positive capacity.
    /// They must be aligned and ordered, with initialized-end and reserved-end on element boundaries within or one past the same allocation.
    /// Reserved-end must describe the usable allocation capacity, whose byte size must not exceed `isize::MAX`.
    /// Every element before initialized-end must be a valid initialized `T` that may be destroyed by Rust.
    /// The caller must transfer exclusive ownership, including the right to mutate, reallocate, and free the allocation.
    /// Do not adopt a borrowed game vector or leave another owner of the same allocation.
    ///
    /// # Panics
    ///
    /// Panics for zero-sized or unsupported over-aligned element types, without adopting the allocation.
    pub unsafe fn from_raw_parts(items: *mut T, items_end: *mut T, items_rsv_end: *mut T) -> Self {
        assert_supported_element::<T>();
        Self {
            items,
            items_end,
            items_rsv_end,
        }
    }

    /// Appends an item, allocating or growing storage with the libc allocator.
    ///
    /// # Panics
    ///
    /// Panics if the required capacity overflows or exceeds `isize::MAX` bytes.
    /// Allocation failure invokes Rust's allocation-error handler.
    pub fn insert(&mut self, item: T) {
        let len = self.len();
        if len == self.capacity() {
            let capacity = len.checked_mul(2).expect("CVec capacity overflow").max(1);
            let layout = allocation_layout::<T>(capacity);
            unsafe {
                let allocation = if self.items.is_null() {
                    libc::malloc(layout.size())
                } else {
                    libc::realloc(self.items.cast_mut().cast(), layout.size())
                };
                if allocation.is_null() {
                    handle_alloc_error(layout);
                }
                let items = allocation.cast::<T>();
                self.items = items;
                self.items_end = items.add(len);
                self.items_rsv_end = items.add(capacity);
            }
        }
        unsafe {
            self.items_end.cast_mut().write(item);
            self.items_end = self.items_end.add(1);
        }
    }

    /// Borrows the initialized elements without allocating.
    pub fn as_slice(&self) -> &[T] {
        if self.items.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.items, self.len()) }
        }
    }

    /// Collects references to initialized elements into a new Rust vector.
    pub fn items(&self) -> Vec<&T> {
        self.as_slice().iter().collect()
    }

    /// Returns the number of initialized elements.
    pub fn len(&self) -> usize {
        if self.items.is_null() {
            0
        } else {
            unsafe { self.items_end.offset_from(self.items) as usize }
        }
    }

    /// Returns the number of elements the allocation can hold without growing.
    pub fn capacity(&self) -> usize {
        if self.items.is_null() {
            0
        } else {
            unsafe { self.items_rsv_end.offset_from(self.items) as usize }
        }
    }

    /// Returns true if the vector has no initialized elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Serialize> Serialize for CVec<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_slice().serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for CVec<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec: Vec<T> = Vec::deserialize(deserializer)?;
        let mut cvec = CVec::empty();
        for item in vec {
            cvec.insert(item);
        }
        Ok(cvec)
    }
}

impl<'a, T> From<&'a CVec<T>> for Vec<&'a T> {
    fn from(cvec: &'a CVec<T>) -> Self {
        cvec.items()
    }
}

impl<T: Clone> Clone for CVec<T> {
    fn clone(&self) -> Self {
        let mut cloned = Self::empty();
        for item in self.as_slice() {
            cloned.insert(item.clone());
        }
        cloned
    }
}

impl<T> Default for CVec<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Drop for CVec<T> {
    fn drop(&mut self) {
        if !self.items.is_null() {
            let _allocation = AllocationCleanup(self.items.cast_mut().cast());
            // Slice drop glue destroys remaining elements if one destructor panics.
            // The guard then frees storage even while unwinding.
            unsafe {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                    self.items.cast_mut(),
                    self.len(),
                ));
            }
        }
    }
}

struct AllocationCleanup(*mut libc::c_void);

impl Drop for AllocationCleanup {
    fn drop(&mut self) {
        unsafe { libc::free(self.0) };
    }
}

fn assert_supported_element<T>() {
    assert!(
        size_of::<T>() != 0,
        "CVec does not support zero-sized elements"
    );
    // libc does not expose max_align_t on Windows.
    // The CRT guarantees 16-byte malloc alignment on 64-bit Windows and 8-byte alignment on 32-bit Windows.
    #[cfg(windows)]
    let malloc_alignment = if cfg!(target_pointer_width = "64") {
        16
    } else {
        8
    };
    #[cfg(not(windows))]
    let malloc_alignment = align_of::<libc::max_align_t>();
    assert!(
        align_of::<T>() <= malloc_alignment,
        "CVec element alignment exceeds libc malloc alignment"
    );
}

fn allocation_layout<T>(capacity: usize) -> Layout {
    Layout::array::<T>(capacity).expect("CVec capacity overflow")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn native_layout() {
        assert_eq!(size_of::<CVec<u8>>(), 24);
        assert_eq!(align_of::<CVec<u8>>(), 8);
        assert_eq!(std::mem::offset_of!(CVec<u8>, items), 0);
        assert_eq!(std::mem::offset_of!(CVec<u8>, items_end), 8);
        assert_eq!(std::mem::offset_of!(CVec<u8>, items_rsv_end), 16);
        assert!(std::mem::needs_drop::<CVec<u8>>());
    }

    #[test]
    fn empty_and_default_have_null_pointers() {
        for vector in [CVec::<i32>::empty(), CVec::<i32>::default()] {
            assert!(vector.as_ptr().is_null());
            assert!(vector.items_end.is_null());
            assert!(vector.items_rsv_end.is_null());
            assert_eq!(vector.len(), 0);
            assert_eq!(vector.capacity(), 0);
            assert!(vector.is_empty());
            assert!(vector.items().is_empty());
            assert!(vector.as_slice().is_empty());
            assert_eq!(serde_json::to_string(&vector).unwrap(), "[]");
        }
    }

    #[test]
    fn insert_and_serde_round_trip() {
        let mut vector = CVec::empty();
        for item in [10_i32, 20, 30] {
            vector.insert(item);
        }
        assert_eq!(vector.len(), 3);
        assert_eq!(vector.capacity(), 4);
        assert!(!vector.is_empty());
        assert_eq!(vector.as_slice(), [10, 20, 30]);
        let json = serde_json::to_string(&vector).unwrap();
        assert_eq!(json, "[10,20,30]");
        let decoded: CVec<i32> = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.as_slice(), vector.as_slice());
    }

    #[test]
    fn growth_preserves_owned_elements_and_drops_them_once() {
        let owner = Rc::new(());
        let mut vector = CVec::empty();
        for count in 1..=100 {
            vector.insert(owner.clone());
            assert_eq!(vector.len(), count);
            assert!(vector.capacity() >= count);
        }
        assert_eq!(Rc::strong_count(&owner), 101);
        drop(vector);
        assert_eq!(Rc::strong_count(&owner), 1);
    }

    #[test]
    fn clone_has_independent_storage_and_elements() {
        let mut original = CVec::empty();
        original.insert(String::from("heap-allocated vector element"));
        let mut cloned = original.clone();
        assert_ne!(original.as_ptr(), cloned.as_ptr());
        assert_ne!(
            original.as_slice()[0].as_ptr(),
            cloned.as_slice()[0].as_ptr()
        );
        unsafe { (*cloned.as_mut_ptr()).push('!') };
        drop(original);
        assert_eq!(cloned.as_slice()[0], "heap-allocated vector element!");
    }

    #[test]
    fn clone_panic_reclaims_completed_elements() {
        struct Probe {
            live: Rc<Cell<usize>>,
            budget: Rc<Cell<usize>>,
        }
        impl Clone for Probe {
            fn clone(&self) -> Self {
                assert!(self.budget.get() > 0, "clone panic");
                self.budget.set(self.budget.get() - 1);
                self.live.set(self.live.get() + 1);
                Self {
                    live: self.live.clone(),
                    budget: self.budget.clone(),
                }
            }
        }
        impl Drop for Probe {
            fn drop(&mut self) {
                self.live.set(self.live.get() - 1);
            }
        }
        let live = Rc::new(Cell::new(3));
        let budget = Rc::new(Cell::new(1));
        let mut vector = CVec::empty();
        for _ in 0..3 {
            vector.insert(Probe {
                live: live.clone(),
                budget: budget.clone(),
            });
        }
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| vector.clone()));
        assert!(result.is_err());
        assert_eq!(live.get(), 3);
        drop(vector);
        assert_eq!(live.get(), 0);
    }

    #[test]
    fn drop_panic_still_destroys_remaining_elements() {
        struct Probe {
            drops: Rc<Cell<usize>>,
            panic: bool,
        }
        impl Drop for Probe {
            fn drop(&mut self) {
                self.drops.set(self.drops.get() + 1);
                assert!(!self.panic, "drop panic");
            }
        }
        for panic_index in 0..3 {
            let drops = Rc::new(Cell::new(0));
            let mut vector = CVec::empty();
            for index in 0..3 {
                vector.insert(Probe {
                    drops: drops.clone(),
                    panic: index == panic_index,
                });
            }
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(vector)));
            assert!(result.is_err());
            assert_eq!(drops.get(), 3);
        }
    }

    #[test]
    fn raw_parts_transfer_ownership_without_dropping() {
        let owner = Rc::new(());
        let mut vector = CVec::empty();
        vector.insert(owner.clone());
        let (start, end, reserved) = vector.into_raw_parts();
        assert_eq!(Rc::strong_count(&owner), 2);
        let mut adopted = unsafe { CVec::from_raw_parts(start, end, reserved) };
        assert_eq!(adopted.as_ptr(), start);
        adopted.insert(owner.clone());
        drop(adopted);
        assert_eq!(Rc::strong_count(&owner), 1);
        let (start, end, reserved) = CVec::<i32>::empty().into_raw_parts();
        drop(unsafe { CVec::from_raw_parts(start, end, reserved) });
    }

    #[test]
    fn allocated_empty_vector_frees_storage_and_can_grow() {
        let allocation = unsafe { libc::malloc(size_of::<i32>()).cast::<i32>() };
        assert!(!allocation.is_null());
        let mut vector = unsafe { CVec::from_raw_parts(allocation, allocation, allocation.add(1)) };
        assert!(vector.is_empty());
        assert_eq!(vector.capacity(), 1);
        vector.insert(7);
        vector.insert(8);
        assert_eq!(vector.as_slice(), [7, 8]);
    }

    #[test]
    fn raw_pointer_elements_do_not_own_their_targets() {
        let owner = Rc::new(());
        let mut target = Box::new(owner.clone());
        let mut vector = CVec::empty();
        vector.insert(ptr::from_mut(&mut *target));
        drop(vector);
        assert_eq!(Rc::strong_count(&owner), 2);
        drop(target);
        assert_eq!(Rc::strong_count(&owner), 1);
    }

    #[test]
    #[should_panic(expected = "zero-sized")]
    fn rejects_zero_sized_elements() {
        CVec::<()>::empty();
    }

    #[test]
    #[should_panic(expected = "alignment")]
    fn rejects_overaligned_elements() {
        #[repr(align(4096))]
        struct Overaligned(u8);
        let value = Overaligned(0);
        assert_eq!(value.0, 0);
        CVec::<Overaligned>::empty();
    }

    #[test]
    #[should_panic(expected = "capacity overflow")]
    fn rejects_capacity_overflow() {
        allocation_layout::<u64>(usize::MAX);
    }
}
