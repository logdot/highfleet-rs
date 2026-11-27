//! Native campaign NPC record for Steam 1.163.

use crate::general::{EscadraString, ISerializable};

/// Campaign NPC record used by Tarkhans and recruitable ships.
///
/// `0x14028B960` allocates `0x158` bytes and calls the constructor at `0x140289280`.
/// The property registration routine at `0x14023FBD0` binds the exposed fields.
///
/// Borrow live records through raw pointers.
/// Do not construct, move or drop a game-owned record through this view.
/// Use native string operations for mutation.
#[repr(C)]
pub struct Npc {
    /// Non-virtual `ISerializable` base.
    pub base: ISerializable,
    /// Serialized as `m_loyalty`.
    pub loyalty: i32,
    /// Uninterpreted bytes at `0x4C..0x50`.
    pub unknown_004ch: [u8; 4],
    /// Serialized as `m_name`; the NPC identifier.
    pub name: EscadraString,
    /// Serialized as `m_fullname`.
    pub full_name: EscadraString,
    /// Serialized as `m_description`.
    pub description: EscadraString,
    /// Serialized as `m_location`; the city name.
    pub location: EscadraString,
    /// Serialized as `m_ship_first`; the ship design name.
    pub ship_first: EscadraString,
    /// Serialized as `m_ship_first_seed`.
    pub ship_first_seed: i32,
    /// Unmodeled state at `0xF4..0xF6`.
    pub unknown_00f4h: [u8; 2],
    /// Serialized as `m_joined`; raw flag byte.
    pub joined: u8,
    /// Unmodeled state at `0xF7..0xF9`.
    pub unknown_00f7h: [u8; 2],
    /// Serialized as `m_tarkhan`; raw flag byte.
    pub tarkhan: u8,
    /// Unmodeled state at `0xFA..0x158`.
    pub unknown_00fah: [u8; 0x5e],
}

const _: () = {
    use core::mem::{align_of, offset_of, size_of};

    assert!(size_of::<Npc>() == 0x158);
    assert!(align_of::<Npc>() == 8);
    assert!(offset_of!(Npc, base) == 0);
    assert!(offset_of!(Npc, loyalty) == 0x48);
    assert!(offset_of!(Npc, unknown_004ch) == 0x4C);
    assert!(offset_of!(Npc, name) == 0x50);
    assert!(offset_of!(Npc, full_name) == 0x70);
    assert!(offset_of!(Npc, description) == 0x90);
    assert!(offset_of!(Npc, location) == 0xB0);
    assert!(offset_of!(Npc, ship_first) == 0xD0);
    assert!(offset_of!(Npc, ship_first_seed) == 0xF0);
    assert!(offset_of!(Npc, unknown_00f4h) == 0xF4);
    assert!(offset_of!(Npc, joined) == 0xF6);
    assert!(offset_of!(Npc, unknown_00f7h) == 0xF7);
    assert!(offset_of!(Npc, tarkhan) == 0xF9);
    assert!(offset_of!(Npc, unknown_00fah) == 0xFA);
};
