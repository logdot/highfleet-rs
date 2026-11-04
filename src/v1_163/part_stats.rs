//! Native `Stats` (part statistics) record view for Steam 1.163.
//!
//! A live part stores a definition index in `Body::part_index` (`Body + 0x188`).
//! The native `GetStats` at `0x140281e00` resolves that index through the bounds-checked
//! `Stats*` table at `0x143a33bf0`; `-1` and out-of-range indices return null.
//!
//! The record is the native `Stats` class (class code `0x803`). `initStats1` (`0x1402815d0`)
//! constructs `ISerializable` at offset zero and installs the `Stats` vtable;
//! `Stats_ConstructorDefault` (`0x140281e30`) writes the `"Stats"` class name, the `0x803` code,
//! clears `m_category` at `0xb0` and initializes `m_price` at `0x88`.
//!
//! The complete native allocation size is not established. This view is only a **modeled
//! prefix** of `0x150` bytes. Opaque byte arrays represent only gaps inside that prefix; the
//! native tail beginning at `0x150` is not represented by this type and must not be assumed to
//! be padding.
//!
//! Borrow game-owned records through `GetStats`; never construct, copy with `ptr::read`, or drop
//! a game-owned record. Access requires a live, aligned `Stats` object of this version and a
//! borrow whose lifetime the game can uphold. Rust field destruction is not the native virtual
//! destructor.

use crate::general::ISerializable;

/// Modeled native `Stats` record prefix returned by `GetStats(index)`.
///
/// This type covers only the first `0x150` bytes of the native record.
/// The unmodeled tail starts at `0x150` and is not represented here.
#[repr(C)]
pub struct PartStats {
    /// Non-virtual `ISerializable` base at offset zero.
    pub base: ISerializable,
    /// Uninterpreted native state at `0x48..0x88`; includes native strings at `0x48` and `0x68`.
    /// Preserved as opaque bytes rather than interpreted by this prefix view.
    pub unknown_0048h: [u8; 0x40],
    /// Native `m_price` at `0x88`; `GetValue` (`0x140048b80`) returns this DWORD as an `int`.
    pub price: i32,
    /// Uninterpreted native state at `0x8c..0xb0`; includes a native vector header at `0x98..0xb0`.
    pub unknown_008ch: [u8; 0x24],
    /// Native `m_category` bit field at `0xb0`; `GetValue` also reads this DWORD.
    pub category: u32,
    /// Uninterpreted native state at `0xb4..0xf8`.
    pub unknown_00b4h: [u8; 0x44],
    /// Native `m_mdl_thrust` at `0xf8`; `GetThrust` (`0x1400487e0`) reads it with
    /// `MOVSS XMM0,[RAX+0xf8]`.
    pub thrust: f32,
    /// Uninterpreted native state at `0xfc..0x128`.
    pub unknown_00fch: [u8; 0x2c],
    /// Native `m_mdl_sectors_type` at `0x128`; 32-bit mode value.
    /// `UpdateEngineCovered` (`0x14001c4a0`) loads this DWORD and branches on modes 3, 4 and 5;
    /// `Body_ScanSectors` (`0x140052340`) also reads it.
    pub sectors_type: u32,
    /// Uninterpreted native state at `0x12c..0x144`.
    pub unknown_012ch: [u8; 0x18],
    /// Native `m_weapon_caliber` at `0x144`; `ShipStats_GetCaliber` (`0x14015f9a0`)
    /// returns this DWORD in `EAX`.
    pub caliber: i32,
    /// Native `m_weapon_load_amount` at `0x148`; magazine capacity in rounds.
    /// `ShipStats_GetLoadAmount` (`0x14015f9d0`) returns this DWORD and `Part_Tick`
    /// (`0x140054fb0`) converts it as a signed `int`.
    pub magazine_capacity: i32,
    /// Native `m_weapon_load_time` at `0x14c`; seconds.
    /// `ShipStats_GetLoadTime` (`0x14015fa00`) returns it as a `float`.
    ///
    /// The base reload-step interval is `reload_time / magazine_capacity`, before gameplay modifiers.
    pub reload_time: f32,
}

const _: () = {
    use core::mem::{align_of, offset_of, size_of};

    assert!(size_of::<PartStats>() == 0x150);
    assert!(align_of::<PartStats>() == 8);

    assert!(offset_of!(PartStats, base) == 0x00);
    assert!(offset_of!(PartStats, unknown_0048h) == 0x48);
    assert!(offset_of!(PartStats, price) == 0x88);
    assert!(offset_of!(PartStats, unknown_008ch) == 0x8c);
    assert!(offset_of!(PartStats, category) == 0xb0);
    assert!(offset_of!(PartStats, unknown_00b4h) == 0xb4);
    assert!(offset_of!(PartStats, thrust) == 0xf8);
    assert!(offset_of!(PartStats, unknown_00fch) == 0xfc);
    assert!(offset_of!(PartStats, sectors_type) == 0x128);
    assert!(offset_of!(PartStats, unknown_012ch) == 0x12c);
    assert!(offset_of!(PartStats, caliber) == 0x144);
    assert!(offset_of!(PartStats, magazine_capacity) == 0x148);
    assert!(offset_of!(PartStats, reload_time) == 0x14c);
};
