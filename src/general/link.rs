//! Native Link layout shared by Steam 1.151 and 1.163.

use super::Node;
use core::ffi::c_void;

/// Non-owning native Link, tracking a Node pointer and its saved identifier.
///
/// Assigning these fields does not perform native link registration.
/// Use the game's link operations when changing ownership or tracked targets.
#[repr(C)]
pub struct Link {
    /// Native Link vtable pointer; entries are not modeled.
    pub vtable: *const c_void,
    /// Tracked Node, which may be the zero-offset base of a derived object.
    pub target: *mut Node,
    /// Saved Node identifier; master/owner links serialize this member.
    pub saved_id: i64,
}

#[cfg(test)]
mod tests {
    use super::Link;
    use core::mem::{align_of, needs_drop, offset_of, size_of};

    #[test]
    fn native_layout() {
        assert_eq!(size_of::<Link>(), 0x18);
        assert_eq!(align_of::<Link>(), 8);
        assert_eq!(offset_of!(Link, vtable), 0);
        assert_eq!(offset_of!(Link, target), 8);
        assert_eq!(offset_of!(Link, saved_id), 0x10);
        assert!(!needs_drop::<Link>());
    }
}
