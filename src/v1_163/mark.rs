//! Native strategic-map base for Steam 1.163.

use crate::general::Node;

/// `Mark` live view with the allocation size `0x248`.
///
/// The factory at `0x1402024E0` allocates this class (`0x47`).
/// `0x1400E46F0` constructs its `Node` base.
/// `0x140103B10` registers the map position, movement and altitude properties.
///
/// Borrow live objects through raw pointers.
/// Do not construct, move or drop game-owned marks through this view.
#[repr(C)]
pub struct Mark {
    /// Non-virtual `Node` base.
    pub base: Node,
    /// Unmodeled state at `0x128..0x150`.
    pub unknown_0128h: [u8; 0x28],
    /// Serialized as `m_position.x`.
    pub position_x: f32,
    /// Serialized as `m_position.y`.
    pub position_y: f32,
    /// Serialized as `m_velocity`; scalar speed.
    pub speed: f32,
    /// Serialized as `m_course`.
    pub course: f32,
    /// Unmodeled state at `0x160..0x168`.
    pub unknown_0160h: [u8; 8],
    /// Serialized as `m_altitude`.
    pub altitude: f32,
    /// Unmodeled state at `0x16C..0x248`.
    pub unknown_016ch: [u8; 0xdc],
}

const _: () = {
    use core::mem::{align_of, offset_of, size_of};

    assert!(size_of::<Mark>() == 0x248);
    assert!(align_of::<Mark>() == 8);
    assert!(offset_of!(Mark, base) == 0);
    assert!(offset_of!(Mark, unknown_0128h) == 0x128);
    assert!(offset_of!(Mark, position_x) == 0x150);
    assert!(offset_of!(Mark, position_y) == 0x154);
    assert!(offset_of!(Mark, speed) == 0x158);
    assert!(offset_of!(Mark, course) == 0x15C);
    assert!(offset_of!(Mark, unknown_0160h) == 0x160);
    assert!(offset_of!(Mark, altitude) == 0x168);
    assert!(offset_of!(Mark, unknown_016ch) == 0x16C);
};
