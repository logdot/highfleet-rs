//! Partially interpreted native Body layout for Steam 1.163.

use core::ffi::c_void;

/// Native game object used for projectiles and other bodies.
///
/// Covers the observed `0xA40`-byte allocation, but names only verified members.
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
    /// Uninterpreted native state at offsets `0x008..0x2DC`.
    pub unknown_0008h: [u8; 0x2d4],
    /// Projectile explosive power, not final HP damage.
    pub explosive_power: f32,
    /// Uninterpreted native state at offsets `0x2E0..0x3C4`.
    pub unknown_02e0h: [u8; 0xe4],
    /// Projectile logic selector copied from Ammo; 150 selects shrapnel logic.
    pub logic: i32,
    /// Uninterpreted native state at offsets `0x3C8..0x3CC`.
    pub unknown_03c8h: [u8; 4],
    /// State-machine storage reused by projectile logic; original signedness is unknown.
    pub logic_state: u32,
    /// Logic-specific countdown, used as the parent fuse by shrapnel logic.
    pub logic_timer: f32,
    /// Uninterpreted native state at offsets `0x3D4..0xA40`.
    pub unknown_03d4h: [u8; 0x66c],
}

#[cfg(test)]
mod tests {
    use super::Body;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn native_layout() {
        assert_eq!(size_of::<Body>(), 0xa40);
        assert_eq!(align_of::<Body>(), 8);
        assert_eq!(offset_of!(Body, vtable), 0);
        assert_eq!(offset_of!(Body, unknown_0008h), 0x8);
        assert_eq!(offset_of!(Body, explosive_power), 0x2dc);
        assert_eq!(offset_of!(Body, unknown_02e0h), 0x2e0);
        assert_eq!(offset_of!(Body, logic), 0x3c4);
        assert_eq!(offset_of!(Body, unknown_03c8h), 0x3c8);
        assert_eq!(offset_of!(Body, logic_state), 0x3cc);
        assert_eq!(offset_of!(Body, logic_timer), 0x3d0);
        assert_eq!(offset_of!(Body, unknown_03d4h), 0x3d4);
    }
}
