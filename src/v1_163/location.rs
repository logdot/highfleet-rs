//! Native campaign location for Steam 1.163.

use super::Mark;

/// `Location` live view with the allocation size `0x370`.
///
/// The factory at `0x1402024E0` allocates this class (`0xC7`).
/// `0x1400E4460` constructs its `Mark` base.
/// `0x140102320` binds the exposed properties to native storage.
///
/// Borrow live locations through raw pointers.
/// Do not construct, move or drop game-owned locations through this view.
#[repr(C)]
pub struct Location {
    /// Non-virtual `Mark` base; the city name is in its embedded `Node`.
    pub base: Mark,
    /// Serialized as `m_citysize`; used by terrain setup and spawn selection.
    pub city_size: f32,
    /// Unmodeled state at `0x24C..0x25C`.
    pub unknown_024ch: [u8; 0x10],
    /// Serialized as `m_bonus_type`; city type used by shops and world generation.
    pub bonus_type: i32,
    /// Unmodeled state at `0x260..0x26C`.
    pub unknown_0260h: [u8; 0xc],
    /// Serialized as `m_fuel`.
    pub fuel: f32,
    /// Serialized as `m_fuel_capacity`.
    pub fuel_capacity: f32,
    /// Serialized as `m_fuel_granted`.
    pub fuel_granted: f32,
    /// Unmodeled state at `0x278..0x2E8`.
    pub unknown_0278h: [u8; 0x70],
    /// Serialized as `m_discovered`; raw discovery state, not a boolean.
    pub discovered: i32,
    /// Unmodeled state at `0x2EC..0x370`.
    pub unknown_02ech: [u8; 0x84],
}

const _: () = {
    use core::mem::{align_of, offset_of, size_of};

    assert!(size_of::<Location>() == 0x370);
    assert!(align_of::<Location>() == 8);
    assert!(offset_of!(Location, base) == 0);
    assert!(offset_of!(Location, city_size) == 0x248);
    assert!(offset_of!(Location, unknown_024ch) == 0x24C);
    assert!(offset_of!(Location, bonus_type) == 0x25C);
    assert!(offset_of!(Location, unknown_0260h) == 0x260);
    assert!(offset_of!(Location, fuel) == 0x26C);
    assert!(offset_of!(Location, fuel_capacity) == 0x270);
    assert!(offset_of!(Location, fuel_granted) == 0x274);
    assert!(offset_of!(Location, unknown_0278h) == 0x278);
    assert!(offset_of!(Location, discovered) == 0x2E8);
    assert!(offset_of!(Location, unknown_02ech) == 0x2EC);
};
