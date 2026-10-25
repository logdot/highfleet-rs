//! Native Profile prefix live view for Steam 1.163 cash reads.

/// Prefix view of a live native Profile, spanning only `0x2B0` bytes.
///
/// This is not the complete native allocation.
/// Opaque state may include native ownership that this view does not model.
/// Do not:
/// - construct profiles in Rust,
/// - copy native objects into Rust ownership,
/// - use this view to destroy or deallocate a game-owned object.
///
/// Borrow only a live object through a raw pointer or a short-lived reference.
#[repr(C)]
pub struct Profile {
    /// Uninterpreted native state at `0x000..0x2A8`.
    pub unknown_0000h: [u64; 0x55],
    /// Native cash balance at `0x2A8`.
    pub cash: i32,
    /// Uninterpreted native state at `0x2AC..0x2B0`.
    pub unknown_02ach: [u8; 4],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn native_prefix_layout() {
        assert_eq!(size_of::<Profile>(), 0x2b0);
        assert_eq!(align_of::<Profile>(), 8);
        assert_eq!(offset_of!(Profile, unknown_0000h), 0);
        assert_eq!(offset_of!(Profile, cash), 0x2a8);
        assert_eq!(offset_of!(Profile, unknown_02ach), 0x2ac);
    }

    #[test]
    fn explicit_fields_cover_every_byte_without_implicit_padding() {
        let fields: &[(usize, usize)] = &[
            (offset_of!(Profile, unknown_0000h), size_of::<[u64; 0x55]>()),
            (offset_of!(Profile, cash), size_of::<i32>()),
            (offset_of!(Profile, unknown_02ach), size_of::<[u8; 4]>()),
        ];
        let mut end = 0;
        for &(offset, width) in fields {
            assert_eq!(offset, end);
            end = offset + width;
        }
        assert_eq!(end, size_of::<Profile>());
    }
}
