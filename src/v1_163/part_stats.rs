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
//! The factory at `0x1402024e0` allocates `0x188` bytes for class code `0x803`.
//! The property registration routine at `0x1402821c0` binds the fields to their native storage.
//! Property names are used where runtime semantics are not established.
//!
//! Borrow game-owned records through `GetStats`; never construct, copy with `ptr::read`, or drop
//! a game-owned record. Access requires a live, aligned `Stats` object of this version and a
//! borrow whose lifetime the game can uphold. Rust field destruction is not the native virtual
//! destructor.

use crate::general::{CVec, EscadraString, ISerializable};

/// Native `Stats` record returned by `GetStats(index)`.
#[repr(C)]
pub struct PartStats {
    /// Non-virtual `ISerializable` base at offset zero.
    pub base: ISerializable,
    /// Serialized as `m_oid`; key used by the part-stats name index.
    pub oid: EscadraString,
    /// Serialized as `m_icon_name`.
    pub icon_name: EscadraString,
    /// Native `m_price` at `0x88`; `GetValue` (`0x140048b80`) returns this DWORD as an `int`.
    pub price: i32,
    /// Serialized as `m_important`; raw byte.
    pub important: u8,
    /// Uninterpreted bytes at `0x8d..0x90`.
    pub unknown_008dh: [u8; 3],
    /// Serialized as `m_timecost`.
    pub time_cost: f32,
    /// Serialized as `m_repair_cost`.
    pub repair_cost: f32,
    /// Serialized as `m_resources` through the signed integer vector codec.
    pub resources: CVec<i32>,
    /// Native `m_category` bit field at `0xb0`; `GetValue` also reads this DWORD.
    pub category: u32,
    /// Serialized as `m_sort_index`.
    pub sort_index: u32,
    /// Serialized as `m_burn_hp_max`.
    pub burn_hp_max: f32,
    /// Serialized as `m_burn_ignite`.
    pub burn_ignite: f32,
    /// Serialized as `m_burn_heat`.
    pub burn_heat: f32,
    /// Serialized as `m_mdl_armor`.
    pub armor: f32,
    /// Serialized as `m_mdl_fuel_capacity`.
    pub fuel_capacity: f64,
    /// Serialized as `m_mdl_fuel_need`.
    pub fuel_need: f32,
    /// Serialized as `m_mdl_crew_capacity`.
    pub crew_capacity: i32,
    /// Serialized as `m_mdl_crew_need`.
    pub crew_need: i32,
    /// Serialized as `m_mdl_power`.
    pub power: f32,
    /// Serialized as `m_mdl_power_need`.
    pub power_need: f32,
    /// Serialized as `m_mdl_power_priority`.
    pub power_priority: i32,
    /// Serialized as `m_mdl_ammobox`.
    pub ammobox: f32,
    /// Serialized as `m_mdl_ammobox_need`.
    pub ammobox_need: f32,
    /// Serialized as `m_mdl_ammobox_priority`.
    pub ammobox_priority: i32,
    /// Serialized as `m_mdl_fss_capacity`.
    pub fss_capacity: f32,
    /// Native `m_mdl_thrust` at `0xf8`; `GetThrust` (`0x1400487e0`) reads it with
    /// `MOVSS XMM0,[RAX+0xf8]`.
    pub thrust: f32,
    /// Serialized as `m_mdl_thrust_map`.
    pub map_thrust: f32,
    /// Serialized as `m_mdl_at`.
    pub angular_thrust: f32,
    /// Serialized as `m_mdl_leg_force`.
    pub leg_force: f32,
    /// Serialized as `m_mdl_rotation`.
    pub rotation: f32,
    /// Serialized as `m_mdl_radar`.
    pub radar: f32,
    /// Serialized as `m_mdl_tracking`.
    pub tracking: f32,
    /// Serialized as `m_mdl_guiding`.
    pub guiding: i32,
    /// Serialized as `m_mdl_elint`.
    pub elint: f32,
    /// Serialized as `m_mdl_irst`.
    pub irst: f32,
    /// Serialized as `m_mdl_jammer`.
    pub jammer: f32,
    /// Serialized as `m_mdl_lifesupport`.
    pub life_support: f32,
    /// Native `m_mdl_sectors_type` at `0x128`; 32-bit mode value.
    /// `UpdateEngineCovered` (`0x14001c4a0`) loads this DWORD and branches on modes 3, 4 and 5;
    /// `Body_ScanSectors` (`0x140052340`) also reads it.
    pub sectors_type: u32,
    /// Serialized as `m_mdl_sectors_shift`.
    pub sectors_shift: f32,
    /// Serialized as `m_mdl_sectors_threshold`.
    pub sectors_threshold: f32,
    /// Serialized as `m_signature_ir`.
    pub signature_ir: f32,
    /// Serialized as `m_fxpos.x`.
    pub fx_position_x: f32,
    /// Serialized as `m_fxpos.y`.
    pub fx_position_y: f32,
    /// Serialized as `m_weapon_priority`.
    pub weapon_priority: i32,
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
    /// Serialized as `m_weapon_rate`; `FireGun` uses this rate in rounds per minute.
    pub weapon_rate: f32,
    /// Serialized as `m_acceleration_time`.
    pub acceleration_time: f32,
    /// Serialized as `m_launched_speed`.
    pub launched_speed: f32,
    /// Serialized as `m_launched_speed2`.
    pub launched_speed2: f32,
    /// Serialized as `m_launched_range`.
    pub launched_range: f32,
    /// Serialized as `m_missile_type`; raw selector.
    pub missile_type: i32,
    /// Serialized as `m_missile_explosive`.
    pub missile_explosive: f32,
    /// Serialized as `m_missile_explosive_cluster`.
    pub missile_explosive_cluster: f32,
    /// Serialized as `m_missile_ap`.
    pub missile_ap: f32,
    /// Serialized as `m_missile_agility`.
    pub missile_agility: f32,
    /// Serialized as `m_missile_radar_range`.
    pub missile_radar_range: f32,
    /// Serialized as `m_missile_radar_cone`.
    pub missile_radar_cone: f32,
    /// Serialized as `m_missile_fuze_type`; raw selector.
    pub missile_fuze_type: i32,
    /// Serialized as `m_launched_health`.
    pub launched_health: f32,
}

const _: () = {
    use core::mem::{align_of, offset_of, size_of};

    assert!(size_of::<PartStats>() == 0x188);
    assert!(align_of::<PartStats>() == 8);

    assert!(offset_of!(PartStats, base) == 0x00);
    assert!(offset_of!(PartStats, oid) == 0x48);
    assert!(offset_of!(PartStats, icon_name) == 0x68);
    assert!(offset_of!(PartStats, price) == 0x88);
    assert!(offset_of!(PartStats, important) == 0x8c);
    assert!(offset_of!(PartStats, unknown_008dh) == 0x8d);
    assert!(offset_of!(PartStats, time_cost) == 0x90);
    assert!(offset_of!(PartStats, repair_cost) == 0x94);
    assert!(offset_of!(PartStats, resources) == 0x98);
    assert!(offset_of!(PartStats, category) == 0xb0);
    assert!(offset_of!(PartStats, sort_index) == 0xb4);
    assert!(offset_of!(PartStats, burn_hp_max) == 0xb8);
    assert!(offset_of!(PartStats, burn_ignite) == 0xbc);
    assert!(offset_of!(PartStats, burn_heat) == 0xc0);
    assert!(offset_of!(PartStats, armor) == 0xc4);
    assert!(offset_of!(PartStats, fuel_capacity) == 0xc8);
    assert!(offset_of!(PartStats, fuel_need) == 0xd0);
    assert!(offset_of!(PartStats, crew_capacity) == 0xd4);
    assert!(offset_of!(PartStats, crew_need) == 0xd8);
    assert!(offset_of!(PartStats, power) == 0xdc);
    assert!(offset_of!(PartStats, power_need) == 0xe0);
    assert!(offset_of!(PartStats, power_priority) == 0xe4);
    assert!(offset_of!(PartStats, ammobox) == 0xe8);
    assert!(offset_of!(PartStats, ammobox_need) == 0xec);
    assert!(offset_of!(PartStats, ammobox_priority) == 0xf0);
    assert!(offset_of!(PartStats, fss_capacity) == 0xf4);
    assert!(offset_of!(PartStats, thrust) == 0xf8);
    assert!(offset_of!(PartStats, map_thrust) == 0xfc);
    assert!(offset_of!(PartStats, angular_thrust) == 0x100);
    assert!(offset_of!(PartStats, leg_force) == 0x104);
    assert!(offset_of!(PartStats, rotation) == 0x108);
    assert!(offset_of!(PartStats, radar) == 0x10c);
    assert!(offset_of!(PartStats, tracking) == 0x110);
    assert!(offset_of!(PartStats, guiding) == 0x114);
    assert!(offset_of!(PartStats, elint) == 0x118);
    assert!(offset_of!(PartStats, irst) == 0x11c);
    assert!(offset_of!(PartStats, jammer) == 0x120);
    assert!(offset_of!(PartStats, life_support) == 0x124);
    assert!(offset_of!(PartStats, sectors_type) == 0x128);
    assert!(offset_of!(PartStats, sectors_shift) == 0x12c);
    assert!(offset_of!(PartStats, sectors_threshold) == 0x130);
    assert!(offset_of!(PartStats, signature_ir) == 0x134);
    assert!(offset_of!(PartStats, fx_position_x) == 0x138);
    assert!(offset_of!(PartStats, fx_position_y) == 0x13c);
    assert!(offset_of!(PartStats, weapon_priority) == 0x140);
    assert!(offset_of!(PartStats, caliber) == 0x144);
    assert!(offset_of!(PartStats, magazine_capacity) == 0x148);
    assert!(offset_of!(PartStats, reload_time) == 0x14c);
    assert!(offset_of!(PartStats, weapon_rate) == 0x150);
    assert!(offset_of!(PartStats, acceleration_time) == 0x154);
    assert!(offset_of!(PartStats, launched_speed) == 0x158);
    assert!(offset_of!(PartStats, launched_speed2) == 0x15c);
    assert!(offset_of!(PartStats, launched_range) == 0x160);
    assert!(offset_of!(PartStats, missile_type) == 0x164);
    assert!(offset_of!(PartStats, missile_explosive) == 0x168);
    assert!(offset_of!(PartStats, missile_explosive_cluster) == 0x16c);
    assert!(offset_of!(PartStats, missile_ap) == 0x170);
    assert!(offset_of!(PartStats, missile_agility) == 0x174);
    assert!(offset_of!(PartStats, missile_radar_range) == 0x178);
    assert!(offset_of!(PartStats, missile_radar_cone) == 0x17c);
    assert!(offset_of!(PartStats, missile_fuze_type) == 0x180);
    assert!(offset_of!(PartStats, launched_health) == 0x184);
};
