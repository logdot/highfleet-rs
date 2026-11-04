//! Native AircraftLogic live view for Steam 1.163.

use crate::general::{ISerializable, Link, SentinelList};

/// Native AircraftLogic layout with the observed complete allocation size of `0x1B0`.
///
/// The constructor at `0x140006B40` calls the non-virtual `ISerializable` constructor at offset zero.
/// The aircraft arming path `0x14002E930` allocates the object with `New(0x1B0)` and stores it in the
/// plane Body's `aircraft_logic` pointer.
/// The constructor initializes the four native Links at `0x48`, `0x100`, `0x150` and `0x178`, and the
/// three sentinel-list headers at `0x120`, `0x130` and `0x140`.
/// `AircraftLogic_InitDefaults` (`0x14000ADF0`) initializes the behavior-mode field at `0x118` to `6`;
/// `AircraftLogic_InitAndPickMode` (`0x14000AE90`) overwrites it from `stateSelectorAircraft_`.
/// `AircraftLogic_SetState` (`0x14000EBB0`) writes the state field at `0x190`.
/// `configure_plane_gun` (`0x14000EA20`) writes the gun fields at `0xA0` and `0xA8..0xB4`, and
/// `AircraftLogic_UpdateAmmoTimers` (`0x14000FA30`) maintains the reload timer at `0xA4` and the payload
/// lists' timers.
///
/// This is a live view only.
/// Opaque regions are native state, not disposable padding or mod storage.
/// Do not construct, move or drop a game-owned AircraftLogic through this view.
/// Rust drop is not the native virtual destructor and does not unregister native Links.
/// The sentinel-list headers are non-owning; their nodes and integer payloads belong to the game.
/// Do not create ownership copies of game-owned strings, lists or Links with `ptr::read`,
/// `read_unaligned` or `from_raw_parts`.
/// Borrow the object through a raw pointer or a short-lived reference, and change Links only through
/// the game's own link operations.
/// Access requires a live, aligned object of this version; do not form references whose lifetime
/// or exclusivity the game cannot uphold.
#[repr(C)]
pub struct AircraftLogic {
    /// Non-virtual `ISerializable` base at offset zero.
    pub base: ISerializable,
    /// Native Link to the aircraft Body associated with this logic; its target is read at `0x50`.
    pub body_link: Link,
    /// Uninterpreted native state at `0x60..0xA0`; not disposable padding.
    pub unknown_0060h: [u8; 0x40],
    /// Gun reload duration in seconds, written by `configure_plane_gun` (`0x14000EA20`).
    /// This is the configured duration, not the remaining reload time.
    pub gun_reload_duration: f32,
    /// Remaining gun reload time in seconds; `AircraftLogic_UpdateAmmoTimers` (`0x14000FA30`)
    /// decrements this value.
    pub gun_reload_timer: f32,
    /// Gun fire delay copied from the ammo record's `fire_delay`.
    pub gun_rate: f32,
    /// Rounds currently loaded in the gun magazine.
    pub gun_load: i32,
    /// Gun magazine capacity copied from the ammo record's `salvo_size`.
    pub gun_max_load: i32,
    /// Gun rounds available outside the current magazine.
    /// `configure_plane_gun` initializes this to `max(0, 50 - magazine_size)`.
    pub gun_reserve_ammo: i32,
    /// Uninterpreted native state at `0xB8..0x100`; not disposable padding.
    pub unknown_00b8h: [u8; 0x48],
    /// Native Link at `0x100` whose application role is not established.
    pub unnamed_link_100h: Link,
    /// Behavior-mode selector read by `AircraftLogic_Update` (`0x14000B370`).
    /// `AircraftLogic_InitDefaults` initializes it to `6`; `AircraftLogic_InitAndPickMode`
    /// overwrites it from the scene's `stateSelectorAircraft_`.
    /// Values are raw native integers, not modeled as an enum.
    pub mode: i32,
    /// Uninterpreted native state at `0x11C..0x120`; not disposable padding.
    pub unknown_011ch: [u8; 0x4],
    /// Non-owning sentinel list of ammo-table indices for bomb-class payload (class 2).
    /// The list header is at `0x120`; its `length` is the qword at `0x128`.
    /// The integers are ammo indices, not payload counts.
    pub bomb_ammo_indices: SentinelList<i32>,
    /// Non-owning sentinel list of ammo-table indices for rocket-class payload (class 3).
    /// The list header is at `0x130`; its `length` is the qword at `0x138`.
    /// The integers are ammo indices, not payload counts.
    pub rocket_ammo_indices: SentinelList<i32>,
    /// Non-owning sentinel list of ammo-table indices for air-to-air missile payload (class 5).
    /// The list header is at `0x140`; its `length` is the qword at `0x148`.
    /// The integers are ammo indices, not payload counts.
    pub missile_ammo_indices: SentinelList<i32>,
    /// Native Link to the current target.
    /// The target node pointer is at `0x158` and its saved identifier is at `0x160`.
    /// Change it through the game's link operations rather than assigning the fields directly.
    pub target_link: Link,
    /// Uninterpreted native state at `0x168..0x178`; not disposable padding.
    pub unknown_0168h: [u8; 0x10],
    /// Native Link at `0x178` whose application role is not established.
    pub unnamed_link_178h: Link,
    /// Aircraft state/phase written by `AircraftLogic_SetState` (`0x14000EBB0`).
    /// Values are raw native integers, not modeled as an enum.
    pub state: i32,
    /// Uninterpreted native state at `0x194..0x1A4`; not disposable padding.
    pub unknown_0194h: [u8; 0x10],
    /// Index of the selected gun ammo in the live ammo table.
    /// Zero means that no gun ammunition is configured.
    pub gun_ammo_index: i32,
    /// Uninterpreted native state at `0x1A8..0x1B0`; not disposable padding.
    pub unknown_01a8h: [u8; 0x8],
}

const _: () = {
    use core::mem::{align_of, offset_of, size_of};

    assert!(size_of::<ISerializable>() == 0x48);
    assert!(size_of::<Link>() == 0x18);
    assert!(size_of::<SentinelList<i32>>() == 0x10);
    assert!(offset_of!(AircraftLogic, base) == 0x00);
    assert!(offset_of!(AircraftLogic, body_link) == 0x48);
    assert!(offset_of!(AircraftLogic, unknown_0060h) == 0x60);
    assert!(offset_of!(AircraftLogic, gun_reload_duration) == 0xA0);
    assert!(offset_of!(AircraftLogic, gun_reload_timer) == 0xA4);
    assert!(offset_of!(AircraftLogic, gun_rate) == 0xA8);
    assert!(offset_of!(AircraftLogic, gun_load) == 0xAC);
    assert!(offset_of!(AircraftLogic, gun_max_load) == 0xB0);
    assert!(offset_of!(AircraftLogic, gun_reserve_ammo) == 0xB4);
    assert!(offset_of!(AircraftLogic, unknown_00b8h) == 0xB8);
    assert!(offset_of!(AircraftLogic, unnamed_link_100h) == 0x100);
    assert!(offset_of!(AircraftLogic, mode) == 0x118);
    assert!(offset_of!(AircraftLogic, unknown_011ch) == 0x11C);
    assert!(offset_of!(AircraftLogic, bomb_ammo_indices) == 0x120);
    assert!(offset_of!(AircraftLogic, rocket_ammo_indices) == 0x130);
    assert!(offset_of!(AircraftLogic, missile_ammo_indices) == 0x140);
    assert!(offset_of!(AircraftLogic, target_link) == 0x150);
    assert!(offset_of!(AircraftLogic, unknown_0168h) == 0x168);
    assert!(offset_of!(AircraftLogic, unnamed_link_178h) == 0x178);
    assert!(offset_of!(AircraftLogic, state) == 0x190);
    assert!(offset_of!(AircraftLogic, unknown_0194h) == 0x194);
    assert!(offset_of!(AircraftLogic, gun_ammo_index) == 0x1A4);
    assert!(offset_of!(AircraftLogic, unknown_01a8h) == 0x1A8);
    assert!(size_of::<AircraftLogic>() == 0x1B0);
    assert!(align_of::<AircraftLogic>() == 8);
};
