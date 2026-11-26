//! Native strategic fleet for Steam 1.163.

use super::Mark;

/// `Escadra` live view with the allocation size `0x618`.
///
/// The factory at `0x1402024E0` allocates this class (`0x147`).
/// `0x1400E3920` constructs its `Mark` base.
/// `0x1400FEB90` registers the fleet's own properties.
///
/// Borrow live fleets through raw pointers.
/// Do not construct, move or drop game-owned fleets through this view.
#[repr(C)]
pub struct Escadra {
    /// Non-virtual `Mark` base.
    pub base: Mark,
    /// Unmodeled state at `0x248..0x3A8`.
    pub unknown_0248h: [u8; 0x160],
    /// Serialized as `m_role`; raw fleet role.
    pub role: i32,
    /// Unmodeled state at `0x3AC..0x498`.
    pub unknown_03ach: [u8; 0xec],
    /// Serialized as `m_tele_fuel_total`.
    pub fuel: f64,
    /// Serialized as `m_tele_fuel_capacity`.
    pub fuel_capacity: f64,
    /// Unmodeled state at `0x4A8..0x52C`.
    pub unknown_04a8h: [u8; 0x84],
    /// Serialized as `canBeUnloaded`; raw flag byte.
    pub can_be_unloaded: u8,
    /// Unmodeled state at `0x52D..0x618`.
    pub unknown_052dh: [u8; 0xeb],
}

const _: () = {
    use core::mem::{align_of, offset_of, size_of};

    assert!(size_of::<Escadra>() == 0x618);
    assert!(align_of::<Escadra>() == 8);
    assert!(offset_of!(Escadra, base) == 0);
    assert!(offset_of!(Escadra, unknown_0248h) == 0x248);
    assert!(offset_of!(Escadra, role) == 0x3A8);
    assert!(offset_of!(Escadra, unknown_03ach) == 0x3AC);
    assert!(offset_of!(Escadra, fuel) == 0x498);
    assert!(offset_of!(Escadra, fuel_capacity) == 0x4A0);
    assert!(offset_of!(Escadra, unknown_04a8h) == 0x4A8);
    assert!(offset_of!(Escadra, can_be_unloaded) == 0x52C);
    assert!(offset_of!(Escadra, unknown_052dh) == 0x52D);
};
