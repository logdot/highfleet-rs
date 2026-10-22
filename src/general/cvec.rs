//! Three-pointer vector interoperability with native HighFleet data.
//!
//! Native pointers may refer to borrowed storage: this type does not track ownership and intentionally has no `Drop` implementation.
//! It never automatically destroys elements or frees storage, including allocations from insertion, cloning, or deserialization.
//! Owners must arrange appropriate element destruction and cleanup.
//!
//! Reading requires live, aligned, initialized elements and start/end pointers into the same allocation, with no conflicting mutation while references are held.
//! Insertion additionally requires exclusive access to writable storage compatible with this process's `libc::malloc`/`libc::realloc` allocator.
//! Do not assume native allocations use that allocator.
//! The inherited implementation does not support zero-sized elements or alignment beyond what `malloc` guarantees, and its safe methods do not validate these pointer and allocator requirements.

use serde::{Deserialize, Serialize};

/// A native vector represented by start, initialized-end, and reserved-end pointers.
///
/// The C representation preserves their order and layout; on 64-bit targets this is 24 bytes with 8-byte alignment.
/// See the module's ownership and allocator cautions before reading or mutating native vectors.
#[repr(C)]
#[derive(Debug)]
pub struct CVec<T> {
    /// Pointer to the start of the items array.
    pub items: *const T,
    /// Pointer to the end of the items array.
    pub items_end: *const T,
    /// Pointer to the reserved end of the items array.
    pub items_rsv_end: *const T,
}

impl<T> CVec<T> {
    /// Creates an empty vector with all three pointers null and no allocation.
    pub fn empty() -> Self {
        Self {
            items: std::ptr::null(),
            items_end: std::ptr::null(),
            items_rsv_end: std::ptr::null(),
        }
    }

    /// Appends an item, allocating or growing storage with the libc allocator.
    ///
    /// Existing storage must be exclusively writable and compatible with libc's allocator; borrowed native storage is not necessarily suitable.
    /// Elements must be non-zero-sized and require no more alignment than malloc provides.
    /// This type does not automatically destroy elements or release allocations.
    ///
    /// # Panics
    ///
    /// Panics if allocation or reallocation fails.
    pub fn insert(&mut self, item: T) {
        unsafe {
            if self.items.is_null() {
                // Allocate initial space for one item
                let new_items = libc::malloc(std::mem::size_of::<T>()) as *mut T;
                if new_items.is_null() {
                    panic!("Failed to allocate memory for CVec");
                }
                self.items = new_items;
                self.items_end = new_items;
                self.items_rsv_end = new_items.add(1);
            } else if self.items_end == self.items_rsv_end {
                // Need to reallocate
                let current_len = self.len();
                let new_capacity = if current_len == 0 { 1 } else { current_len * 2 };
                let new_size = new_capacity * std::mem::size_of::<T>();
                let new_items = libc::realloc(self.items as *mut libc::c_void, new_size) as *mut T;
                if new_items.is_null() {
                    panic!("Failed to reallocate memory for CVec");
                }
                self.items = new_items;
                self.items_end = new_items.add(current_len);
                self.items_rsv_end = new_items.add(new_capacity);
            }

            // Insert the new item
            std::ptr::write(self.items_end as *mut T, item);
            self.items_end = self.items_end.add(1);
        }
    }

    /// Collects references to initialized elements into a new Rust vector.
    ///
    /// The native storage must remain valid and unmodified for the references' lifetimes.
    /// Invalid pointers cannot be made safe by the length checks.
    pub fn items(&self) -> Vec<&T> {
        Vec::from(self)
    }

    /// Returns the number of items in the CVec.
    pub fn len(&self) -> usize {
        if self.items.is_null() || self.items_end.is_null() {
            return 0;
        }

        let count = unsafe { self.items_end.offset_from(self.items) };
        if count < 0 {
            log::error!("CVec has negative item count, returning length 0");
            return 0;
        }

        count as usize
    }

    /// Returns true if the CVec is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Serialize> Serialize for CVec<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let vec: Vec<&T> = self.into();
        vec.serialize(serializer)
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

impl<T> From<&CVec<T>> for Vec<&T> {
    fn from(cvec: &CVec<T>) -> Self {
        if cvec.items.is_null() || cvec.items_end.is_null() {
            return Vec::new();
        }

        let count = unsafe { cvec.items_end.offset_from(cvec.items) };
        if count < 0 {
            log::error!("CVec has negative item count, returning empty Vec");
            return Vec::new();
        }

        if count == 0 {
            return Vec::new();
        }

        if count > 10_000 {
            log::warn!("CVec has an unusually high number of items: {}", count);
        }

        let mut result = Vec::with_capacity(count as usize);

        unsafe {
            let mut current = cvec.items;

            while current < cvec.items_end {
                if current.is_null() {
                    break;
                }

                result.push(&(*current));
                current = current.add(1);
            }
        }

        if result.len() < count as usize {
            log::warn!(
                "CVec expected to have {} items, but only read {} items",
                count,
                result.len()
            );
        }

        result
    }
}

impl<T: Clone> Clone for CVec<T> {
    fn clone(&self) -> Self {
        let mut new_cvec = CVec::empty();
        for item in self.items() {
            new_cvec.insert(item.clone());
        }
        new_cvec
    }
}

impl<T> Default for CVec<T> {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::CVec;

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn native_layout() {
        assert_eq!(std::mem::size_of::<CVec<u8>>(), 24);
        assert_eq!(std::mem::align_of::<CVec<u8>>(), 8);
        assert_eq!(std::mem::offset_of!(CVec<u8>, items), 0);
        assert_eq!(std::mem::offset_of!(CVec<u8>, items_end), 8);
        assert_eq!(std::mem::offset_of!(CVec<u8>, items_rsv_end), 16);
    }

    #[test]
    fn empty_and_default_have_null_pointers() {
        for vector in [CVec::<i32>::empty(), CVec::<i32>::default()] {
            assert!(vector.items.is_null());
            assert!(vector.items_end.is_null());
            assert!(vector.items_rsv_end.is_null());
            assert_eq!(vector.len(), 0);
            assert!(vector.is_empty());
            assert!(vector.items().is_empty());
            assert_eq!(serde_json::to_string(&vector).unwrap(), "[]");
        }
    }

    #[test]
    fn insert_and_serde_round_trip() {
        let mut vector = CVec::empty();
        for item in [10_i32, 20, 30] {
            vector.insert(item);
        }
        let len = vector.len();
        let is_empty = vector.is_empty();
        let values: Vec<i32> = vector.items().into_iter().copied().collect();
        let json = serde_json::to_string(&vector);
        // Only libc-owned, trivially destructible test elements are freed here.
        // No references into the allocation survive this point.
        unsafe { libc::free(vector.items as *mut libc::c_void) };

        let json = json.unwrap();
        let decoded: CVec<i32> = serde_json::from_str(&json).unwrap();
        let decoded_values: Vec<i32> = decoded.items().into_iter().copied().collect();
        let decoded_len = decoded.len();
        unsafe { libc::free(decoded.items as *mut libc::c_void) };

        assert_eq!(len, 3);
        assert!(!is_empty);
        assert_eq!(values, [10, 20, 30]);
        assert_eq!(json, "[10,20,30]");
        assert_eq!(decoded_len, 3);
        assert_eq!(decoded_values, values);
    }
}
