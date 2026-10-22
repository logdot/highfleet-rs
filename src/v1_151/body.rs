//! Partially interpreted native Body layout for Steam 1.151.

use core::ffi::c_void;

/// Native game object used for projectiles and other bodies.
///
/// Covers the observed `0xA08`-byte allocation, but names only verified members.
/// Unknown byte regions contain game state, not unused storage for mods.
/// This is a non-owning interoperability layout: it does not model construction,
/// virtual dispatch, or destruction. Access requires a live, suitably aligned
/// native object of this version; do not create Rust references with lifetimes
/// or exclusivity guarantees that the game cannot uphold.
///
/// The pointer member gives this Rust view 8-byte alignment on Windows x86-64;
/// this does not establish the original C++ class's declared alignment.
#[repr(C)]
pub struct Body {
    /// Pointer to the native virtual-function table; its entries are not modeled.
    pub vtable: *const c_void,
    /// Uninterpreted native state at offsets `0x008..0x2CC`.
    pub unknown_0008h: [u8; 0x2c4],
    /// Projectile explosive power, not final HP damage.
    pub explosive_power: f32,
    /// Uninterpreted native state at offsets `0x2D0..0x3B4`.
    pub unknown_02d0h: [u8; 0xe4],
    /// Projectile logic selector copied from Ammo; 150 selects shrapnel logic.
    pub logic: i32,
    /// Uninterpreted native state at offsets `0x3B8..0x3BC`.
    pub unknown_03b8h: [u8; 4],
    /// State-machine storage reused by projectile logic; original signedness is unknown.
    pub logic_state: u32,
    /// Logic-specific countdown, used as the parent fuse by shrapnel logic.
    pub logic_timer: f32,
    /// Uninterpreted native state at offsets `0x3C4..0xA08`.
    pub unknown_03c4h: [u8; 0x644],
}

#[cfg(test)]
mod tests {
    use super::Body;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn native_layout() {
        assert_eq!(size_of::<Body>(), 0xa08);
        assert_eq!(align_of::<Body>(), 8);
        assert_eq!(offset_of!(Body, vtable), 0);
        assert_eq!(offset_of!(Body, unknown_0008h), 0x8);
        assert_eq!(offset_of!(Body, explosive_power), 0x2cc);
        assert_eq!(offset_of!(Body, unknown_02d0h), 0x2d0);
        assert_eq!(offset_of!(Body, logic), 0x3b4);
        assert_eq!(offset_of!(Body, unknown_03b8h), 0x3b8);
        assert_eq!(offset_of!(Body, logic_state), 0x3bc);
        assert_eq!(offset_of!(Body, logic_timer), 0x3c0);
        assert_eq!(offset_of!(Body, unknown_03c4h), 0x3c4);
    }
}
