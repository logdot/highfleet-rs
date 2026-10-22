//! Native Body view for Steam 1.163, recovered from custom serialization and runtime accesses.

use crate::general::{CVec, EscadraString, ISerializable, Link, Node};

/// Native Body layout, with the observed complete allocation size.
///
/// Embedded bases model the RTTI-confirmed non-virtual chain `Body -> Node -> ISerializable -> IObject`, all at offset zero.
/// Named fields follow serializer keys and verified access widths, not the inconsistent saved composite datatype.
/// Keys do not establish enum variants or application-level semantics beyond their native representations.
///
/// Rust-owned values must be constructed with valid initialized fields, including every `EscadraString` and `CVec` header.
/// Rust drop releases string storage, drops initialized vector elements, and frees vector storage through the field types' destructors.
/// Raw pointer elements do not own their targets, and non-owning sentinel-list headers do not free their nodes.
/// Rust drop is not the native virtual destructor.
///
/// Borrow game-owned objects through raw pointers or references; never use `ptr::read` to create an owning copy or drop a game-owned object.
/// Access requires a live, aligned object of the selected version.
/// Avoid Rust references whose lifetime or exclusivity the game cannot uphold.
/// Unknown bytes are native state, not mod storage.
#[repr(C)]
pub struct Body {
    /// Non-virtual Node base at offset zero; the primary vtable is in `base.base.base.vtable`.
    pub base: Node,
    /// Serialized as `m_position.x`; native storage at `0x128`.
    pub position_x: f32,
    /// Serialized as `m_position.y`; native storage at `0x12C`.
    pub position_y: f32,
    /// Serialized as `m_velocity.x`; native storage at `0x130`.
    pub velocity_x: f32,
    /// Serialized as `m_velocity.y`; native storage at `0x134`.
    pub velocity_y: f32,
    /// Serialized as `m_angle`; native storage at `0x138`.
    pub angle: f32,
    /// Serialized as `m_ang_velocity`; native storage at `0x13C`.
    pub ang_velocity: f32,
    /// Uninterpreted native state at `0x140..0x188`; not disposable padding.
    pub unknown_0140h: [u8; 0x48],
    /// Runtime definition-table index used by `GetStats`; native signed index at `0x188`, not serialized.
    /// A value of `-1` means no definition, and `GetStats` range-checks the index before indexing its pointer table.
    pub part_index: i32,
    /// Uninterpreted native state at `0x18C..0x190`; not disposable padding.
    pub unknown_018ch: [u8; 0x4],
    /// Serialized as `m_oid`; native storage at `0x190`.
    pub oid: EscadraString,
    /// Serialized as `m_b2bstate`; native storage at `0x1B0`.
    pub b2bstate: i32,
    /// Uninterpreted native state at `0x1B4..0x1B8`; not disposable padding.
    pub unknown_01b4h: [u8; 0x4],
    /// Serialized as `m_mesh`; native storage at `0x1B8`.
    pub mesh: *mut ISerializable,
    /// Serialized as `m_meta`; native storage at `0x1C0`.
    pub meta: u64,
    /// Serialized as `m_material`; native storage at `0x1C8`.
    pub material: u64,
    /// Serialized as `m_density`; native storage at `0x1D0`.
    pub density: f32,
    /// Serialized as `m_friction`; native storage at `0x1D4`.
    pub friction: f32,
    /// Serialized as `m_mass`; native storage at `0x1D8`.
    pub mass: f32,
    /// Serialized as `m_drag_lin`; native storage at `0x1DC`.
    pub drag_lin: f32,
    /// Serialized as `m_drag_ang`; native storage at `0x1E0`.
    pub drag_ang: f32,
    /// Uninterpreted native state at `0x1E4..0x1F0`; not disposable padding.
    pub unknown_01e4h: [u8; 0xc],
    /// Serialized as `m_layer`; native storage at `0x1F0`.
    pub layer: i32,
    /// Uninterpreted native state at `0x1F4..0x1FC`; not disposable padding.
    pub unknown_01f4h: [u8; 0x8],
    /// Serialized as `m_bullet`; native storage at `0x1FC`.
    /// Raw flag byte, not a Rust `bool`.
    pub bullet: u8,
    /// Uninterpreted native state at `0x1FD..0x208`; not disposable padding.
    pub unknown_01fdh: [u8; 0xb],
    /// Serialized as `m_isPhysicUpdate`; native storage at `0x208`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_physic_update: u8,
    /// Uninterpreted native state at `0x209..0x210`; not disposable padding.
    pub unknown_0209h: [u8; 0x7],
    /// Serialized as `m_center.x`; native storage at `0x210`.
    pub center_x: f32,
    /// Serialized as `m_center.y`; native storage at `0x214`.
    pub center_y: f32,
    /// Uninterpreted native state at `0x218..0x24C`; not disposable padding.
    pub unknown_0218h: [u8; 0x34],
    /// Serialized as `refuelCounter`; native storage at `0x24C`.
    pub refuel_counter: f32,
    /// Serialized as `aimerCreatureId`; native storage at `0x250`.
    pub aimer_creature_id: i64,
    /// Serialized as `survivorsPenalty`; native storage at `0x258`.
    /// Raw flag byte, not a Rust `bool`.
    pub survivors_penalty: u8,
    /// Serialized as `isRightDirection`; native storage at `0x259`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_right_direction: u8,
    /// Uninterpreted native state at `0x25A..0x270`; not disposable padding.
    pub unknown_025ah: [u8; 0x16],
    /// Serialized as `shipConvertionError`; native storage at `0x270`.
    pub ship_convertion_error: EscadraString,
    /// Uninterpreted native state at `0x290..0x2B8`; not disposable padding.
    pub unknown_0290h: [u8; 0x28],
    /// Serialized as `m_collisions`; native storage at `0x2B8`.
    pub collisions: i32,
    /// Uninterpreted native state at `0x2BC..0x2C0`; not disposable padding.
    pub unknown_02bch: [u8; 0x4],
    /// Serialized as `m_ttl`; native storage at `0x2C0`.
    pub ttl: f32,
    /// Serialized as `m_ttl_desolvetime`; native storage at `0x2C4`.
    pub ttl_desolvetime: f32,
    /// Serialized as `m_health`; native storage at `0x2C8`.
    pub health: f32,
    /// Serialized as `m_health_max`; native storage at `0x2CC`.
    pub health_max: f32,
    /// Serialized as `m_health_lock`; native storage at `0x2D0`.
    /// Raw flag byte, not a Rust `bool`.
    pub health_lock: u8,
    /// Uninterpreted native state at `0x2D1..0x2D4`; not disposable padding.
    pub unknown_02d1h: [u8; 0x3],
    /// Serialized as `m_contamination`; native storage at `0x2D4`.
    pub contamination: f32,
    /// Uninterpreted native state at `0x2D8..0x2DC`; not disposable padding.
    pub unknown_02d8h: [u8; 0x4],
    /// Serialized as `m_explosive`; native storage at `0x2DC`.
    /// Explosive power, not final HP damage.
    pub explosive_power: f32,
    /// Serialized as `m_armorPiercing`; native storage at `0x2E0`.
    pub armor_piercing: f32,
    /// Serialized as `m_explosive_type`; native storage at `0x2E4`.
    pub explosive_type: i32,
    /// Serialized as `m_explosive_temperature`; native storage at `0x2E8`.
    pub explosive_temperature: f32,
    /// Uninterpreted native state at `0x2EC..0x334`; not disposable padding.
    pub unknown_02ech: [u8; 0x48],
    /// Serialized as `m_burn_hp`; native storage at `0x334`.
    pub burn_hp: f32,
    /// Serialized as `m_routs`; native storage at `0x338`.
    pub routs: CVec<*mut ISerializable>,
    /// Serialized as `m_slots`; native storage at `0x350`.
    pub slots: CVec<*mut ISerializable>,
    /// Serialized as `m_slot_type`; native storage at `0x368`.
    pub slot_type: u64,
    /// Serialized as `m_spasmcode`; native storage at `0x370`.
    pub spasmcode: CVec<i32>,
    /// Uninterpreted native state at `0x388..0x38C`; not disposable padding.
    pub unknown_0388h: [u8; 0x4],
    /// Serialized as `m_collision_sensor`; native storage at `0x38C`.
    pub collision_sensor: f32,
    /// Uninterpreted native state at `0x390..0x3C4`; not disposable padding.
    pub unknown_0390h: [u8; 0x34],
    /// Serialized as `m_logic`; native storage at `0x3C4`.
    pub logic: i32,
    /// Serialized as `m_logic_active`; native storage at `0x3C8`.
    /// Raw flag byte, not a Rust `bool`.
    pub logic_active: u8,
    /// Uninterpreted native state at `0x3C9..0x3CC`; not disposable padding.
    pub unknown_03c9h: [u8; 0x3],
    /// Runtime logic state-machine storage; original signedness is not established.
    pub logic_state: u32,
    /// Serialized as `m_logic_f1`; native storage at `0x3D0`.
    /// Logic-specific countdown; the shrapnel fuse for logic 150.
    pub logic_f1: f32,
    /// Serialized as `m_logic_f2`; native storage at `0x3D4`.
    pub logic_f2: f32,
    /// Serialized as `m_logic_f3`; native storage at `0x3D8`.
    pub logic_f3: f32,
    /// Serialized as `m_logic_f4`; native storage at `0x3DC`.
    pub logic_f4: f32,
    /// Uninterpreted native state at `0x3E0..0x3E4`; not disposable padding.
    pub unknown_03e0h: [u8; 0x4],
    /// Serialized as `m_logic_i1`; native storage at `0x3E4`.
    pub logic_i1: i32,
    /// Serialized as `m_logic_i2`; native storage at `0x3E8`.
    pub logic_i2: i32,
    /// Serialized as `m_logic_i3`; native storage at `0x3EC`.
    pub logic_i3: i32,
    /// Serialized as `m_logic_i4`; native storage at `0x3F0`.
    pub logic_i4: i32,
    /// Uninterpreted native state at `0x3F4..0x3F8`; not disposable padding.
    pub unknown_03f4h: [u8; 0x4],
    /// Serialized as `m_logic_s1`; native storage at `0x3F8`.
    pub logic_s1: EscadraString,
    /// Uninterpreted native state at `0x418..0x4D8`; not disposable padding.
    pub unknown_0418h: [u8; 0xc0],
    /// Native Link whose `saved_id` member serializes as `m_logic_l1_saved.id`.
    pub logic_l1_saved: Link,
    /// Serialized as `m_logic_editor`; native storage at `0x4F0`.
    pub logic_editor: CVec<i32>,
    /// Serialized as `m_sectors`; native storage at `0x508`.
    pub sectors: CVec<f32>,
    /// Uninterpreted native state at `0x520..0x7A8`; not disposable padding.
    pub unknown_0520h: [u8; 0x288],
    /// Serialized as `m_count`; native storage at `0x7A8`.
    pub count: i32,
    /// Serialized as `is_loot`; native storage at `0x7AC`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_loot: u8,
    /// Serialized as `is_blueprint`; native storage at `0x7AD`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_blueprint: u8,
    /// Uninterpreted native state at `0x7AE..0x7AF`; not disposable padding.
    pub unknown_07aeh: [u8; 0x1],
    /// Serialized as `is_wheel`; native storage at `0x7AF`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_wheel: u8,
    /// Serialized as `is_leg`; native storage at `0x7B0`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_leg: u8,
    /// Uninterpreted native state at `0x7B1..0x7F8`; not disposable padding.
    pub unknown_07b1h: [u8; 0x47],
    /// Serialized as `m_mdl_fuel`; native storage at `0x7F8`.
    pub mdl_fuel: f64,
    /// Serialized as `m_mdl_fss`; native storage at `0x800`.
    pub mdl_fss: f32,
    /// Serialized as `m_mdl_crew`; native storage at `0x804`.
    pub mdl_crew: i32,
    /// Serialized as `m_mdl_side`; native storage at `0x808`.
    pub mdl_side: i32,
    /// Uninterpreted native state at `0x80C..0x838`; not disposable padding.
    pub unknown_080ch: [u8; 0x2c],
    /// Serialized as `m_mdl_pump`; native storage at `0x838`.
    pub mdl_pump: f32,
    /// Serialized as `m_mdl_turbine`; native storage at `0x83C`.
    pub mdl_turbine: f32,
    /// Uninterpreted native state at `0x840..0x848`; not disposable padding.
    pub unknown_0840h: [u8; 0x8],
    /// Serialized as `m_mdl_covered`; native storage at `0x848`.
    pub mdl_covered: f32,
    /// Serialized as `m_weapon_type`; native storage at `0x84C`.
    pub weapon_type: i32,
    /// Serialized as `m_weapon_ammo`; native storage at `0x850`.
    pub weapon_ammo: i32,
    /// Serialized as `m_weapon_ammo_extra`; native storage at `0x854`.
    pub weapon_ammo_extra: i32,
    /// Serialized as `m_weapon_ammo_amount`; native storage at `0x858`.
    pub weapon_ammo_amount: i32,
    /// Serialized as `m_weapon_ammo_extra_amount`; native storage at `0x85C`.
    pub weapon_ammo_extra_amount: i32,
    /// Uninterpreted native state at `0x860..0x877`; not disposable padding.
    pub unknown_0860h: [u8; 0x17],
    /// Serialized as `m_rescue_explosive`; native storage at `0x877`.
    /// Raw flag byte, not a Rust `bool`.
    pub rescue_explosive: u8,
    /// Serialized as `m_rescue_flameable`; native storage at `0x878`.
    /// Raw flag byte, not a Rust `bool`.
    pub rescue_flameable: u8,
    /// Serialized as `m_rescue_radioactive`; native storage at `0x879`.
    /// Raw flag byte, not a Rust `bool`.
    pub rescue_radioactive: u8,
    /// Serialized as `m_rescue_toxic`; native storage at `0x87A`.
    /// Raw flag byte, not a Rust `bool`.
    pub rescue_toxic: u8,
    /// Serialized as `m_rescue_scare`; native storage at `0x87B`.
    /// Raw flag byte, not a Rust `bool`.
    pub rescue_scare: u8,
    /// Uninterpreted native state at `0x87C..0x884`; not disposable padding.
    pub unknown_087ch: [u8; 0x8],
    /// Serialized as `isCRTScreen`; native storage at `0x884`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_crt_screen: u8,
    /// Uninterpreted native state at `0x885..0x888`; not disposable padding.
    pub unknown_0885h: [u8; 0x3],
    /// Serialized as `repairType`; native storage at `0x888`.
    pub repair_type: i32,
    /// Serialized as `repairPos`; native storage at `0x88C`.
    pub repair_pos: i32,
    /// Serialized as `craftActivated`; native storage at `0x890`.
    /// Raw flag byte, not a Rust `bool`.
    pub craft_activated: u8,
    /// Uninterpreted native state at `0x891..0x8B8`; not disposable padding.
    pub unknown_0891h: [u8; 0x27],
    /// Serialized as `m_mask`; native storage at `0x8B8`.
    pub mask: *mut ISerializable,
    /// Serialized as `m_sprites`; native storage at `0x8C0`.
    pub sprites: CVec<*mut ISerializable>,
    /// Serialized as `m_stage`; native storage at `0x8D8`.
    pub stage: i32,
    /// Uninterpreted native state at `0x8DC..0x8EC`; not disposable padding.
    pub unknown_08dch: [u8; 0x10],
    /// Serialized as `m_mesh_render`; native storage at `0x8EC`.
    /// Raw flag byte, not a Rust `bool`.
    pub mesh_render: u8,
    /// Uninterpreted native state at `0x8ED..0x8F0`; not disposable padding.
    pub unknown_08edh: [u8; 0x3],
    /// Serialized as `m_mesh_color`; native storage at `0x8F0`.
    pub mesh_color: u32,
    /// Serialized as `m_mask_mesh`; native storage at `0x8F4`.
    /// Raw flag byte, not a Rust `bool`.
    pub mask_mesh: u8,
    /// Uninterpreted native state at `0x8F5..0x8F8`; not disposable padding.
    pub unknown_08f5h: [u8; 0x3],
    /// Serialized as `m_opacity`; native storage at `0x8F8`.
    pub opacity: f32,
    /// Uninterpreted native state at `0x8FC..0x90C`; not disposable padding.
    pub unknown_08fch: [u8; 0x10],
    /// Serialized as `m_z`; native storage at `0x90C`.
    pub z: f32,
    /// Uninterpreted native state at `0x910..0x914`; not disposable padding.
    pub unknown_0910h: [u8; 0x4],
    /// Serialized as `m_scalemode_max`; native storage at `0x914`.
    pub scalemode_max: i32,
    /// Serialized as `m_scalemode_min`; native storage at `0x918`.
    pub scalemode_min: i32,
    /// Serialized as `m_gui`; native storage at `0x91C`.
    /// Raw flag byte, not a Rust `bool`.
    pub gui: u8,
    /// Serialized as `m_scalable`; native storage at `0x91D`.
    /// Raw flag byte, not a Rust `bool`.
    pub scalable: u8,
    /// Uninterpreted native state at `0x91E..0x91F`; not disposable padding.
    pub unknown_091eh: [u8; 0x1],
    /// Serialized as `m_issuperlay`; native storage at `0x91F`.
    /// Raw flag byte, not a Rust `bool`.
    pub issuperlay: u8,
    /// Serialized as `m_scale.x`; native storage at `0x920`.
    pub scale_x: f32,
    /// Serialized as `m_scale.y`; native storage at `0x924`.
    pub scale_y: f32,
    /// Serialized as `m_notaproximate`; native storage at `0x928`.
    /// Raw flag byte, not a Rust `bool`.
    pub notaproximate: u8,
    /// Serialized as `m_renderlines`; native storage at `0x929`.
    /// Raw flag byte, not a Rust `bool`.
    pub renderlines: u8,
    /// Uninterpreted native state at `0x92A..0x92C`; not disposable padding.
    pub unknown_092ah: [u8; 0x2],
    /// Serialized as `m_light_pow`; native storage at `0x92C`.
    pub light_pow: f32,
    /// Serialized as `m_light_shift.x`; native storage at `0x930`.
    pub light_shift_x: f32,
    /// Serialized as `m_light_shift.y`; native storage at `0x934`.
    pub light_shift_y: f32,
    /// Serialized as `m_light_dir.x`; native storage at `0x938`.
    pub light_dir_x: f32,
    /// Serialized as `m_light_dir.y`; native storage at `0x93C`.
    pub light_dir_y: f32,
    /// Serialized as `m_light_directional`; native storage at `0x940`.
    pub light_directional: f32,
    /// Serialized as `m_shadow`; native storage at `0x944`.
    pub shadow: f32,
    /// Uninterpreted native state at `0x948..0x9B0`; not disposable padding.
    pub unknown_0948h: [u8; 0x68],
    /// Serialized as `m_floor_type`; native storage at `0x9B0`.
    pub floor_type: u64,
    /// Serialized as `m_floor`; native storage at `0x9B8`.
    pub floor: i32,
    /// Serialized as `m_tile_group`; native storage at `0x9BC`.
    pub tile_group: i32,
    /// Serialized as `m_tile_script`; native storage at `0x9C0`.
    pub tile_script: i32,
    /// Uninterpreted native state at `0x9C4..0xA00`; not disposable padding.
    pub unknown_09c4h: [u8; 0x3c],
    /// Serialized as `m_light_color.r`; native storage at `0xA00`.
    pub light_color_r: f32,
    /// Serialized as `m_light_color.g`; native storage at `0xA04`.
    pub light_color_g: f32,
    /// Serialized as `m_light_color.b`; native storage at `0xA08`.
    pub light_color_b: f32,
    /// Uninterpreted native state at `0xA0C..0xA10`; not disposable padding.
    pub unknown_0a0ch: [u8; 0x4],
    /// Serialized as `m_light_fade`; native storage at `0xA10`.
    pub light_fade: f32,
    /// Serialized as `m_ambient`; native storage at `0xA14`.
    pub ambient: f32,
    /// Serialized as `m_tg_price`; native storage at `0xA18`.
    pub tg_price: f32,
    /// Uninterpreted native state at `0xA1C..0xA20`; not disposable padding.
    pub unknown_0a1ch: [u8; 0x4],
    /// Serialized as `aircraftLogic`; native storage at `0xA20`.
    pub aircraft_logic: *mut ISerializable,
    /// Serialized as `m_ed_rotate`; native storage at `0xA28`.
    pub ed_rotate: f32,
    /// Uninterpreted native state at `0xA2C..0xA34`; not disposable padding.
    pub unknown_0a2ch: [u8; 0x8],
    /// Serialized as `m_help_index`; native storage at `0xA34`.
    pub help_index: i32,
    /// Serialized as `m_focusable`; native storage at `0xA38`.
    /// Raw flag byte, not a Rust `bool`.
    pub focusable: u8,
    /// Serialized as `m_dragable`; native storage at `0xA39`.
    /// Raw flag byte, not a Rust `bool`.
    pub dragable: u8,
    /// Serialized as `m_slideable`; native storage at `0xA3A`.
    /// Raw flag byte, not a Rust `bool`.
    pub slideable: u8,
    /// Serialized as `m_wheelable`; native storage at `0xA3B`.
    /// Raw flag byte, not a Rust `bool`.
    pub wheelable: u8,
    /// Serialized as `m_selectable`; native storage at `0xA3C`.
    /// Raw flag byte, not a Rust `bool`.
    pub selectable: u8,
    /// Serialized as `m_deletable`; native storage at `0xA3D`.
    /// Raw flag byte, not a Rust `bool`.
    pub deletable: u8,
    /// Uninterpreted native state at `0xA3E..0xA40`.
    pub unknown_0a3eh: [u8; 0x2],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, needs_drop, offset_of, size_of};

    #[test]
    fn native_layout() {
        assert_eq!(size_of::<Body>(), 0xa40);
        assert_eq!(align_of::<Body>(), 8);
        assert_eq!(size_of::<EscadraString>(), 32);
        assert!(needs_drop::<Body>());
        assert_eq!(offset_of!(Body, base), 0x0);
        assert_eq!(offset_of!(Body, position_x), 0x128);
        assert_eq!(offset_of!(Body, position_y), 0x12c);
        assert_eq!(offset_of!(Body, velocity_x), 0x130);
        assert_eq!(offset_of!(Body, velocity_y), 0x134);
        assert_eq!(offset_of!(Body, angle), 0x138);
        assert_eq!(offset_of!(Body, ang_velocity), 0x13c);
        assert_eq!(offset_of!(Body, unknown_0140h), 0x140);
        assert_eq!(offset_of!(Body, part_index), 0x188);
        assert_eq!(offset_of!(Body, unknown_018ch), 0x18c);
        assert_eq!(offset_of!(Body, oid), 0x190);
        assert_eq!(offset_of!(Body, b2bstate), 0x1b0);
        assert_eq!(offset_of!(Body, unknown_01b4h), 0x1b4);
        assert_eq!(offset_of!(Body, mesh), 0x1b8);
        assert_eq!(offset_of!(Body, meta), 0x1c0);
        assert_eq!(offset_of!(Body, material), 0x1c8);
        assert_eq!(offset_of!(Body, density), 0x1d0);
        assert_eq!(offset_of!(Body, friction), 0x1d4);
        assert_eq!(offset_of!(Body, mass), 0x1d8);
        assert_eq!(offset_of!(Body, drag_lin), 0x1dc);
        assert_eq!(offset_of!(Body, drag_ang), 0x1e0);
        assert_eq!(offset_of!(Body, unknown_01e4h), 0x1e4);
        assert_eq!(offset_of!(Body, layer), 0x1f0);
        assert_eq!(offset_of!(Body, unknown_01f4h), 0x1f4);
        assert_eq!(offset_of!(Body, bullet), 0x1fc);
        assert_eq!(offset_of!(Body, unknown_01fdh), 0x1fd);
        assert_eq!(offset_of!(Body, is_physic_update), 0x208);
        assert_eq!(offset_of!(Body, unknown_0209h), 0x209);
        assert_eq!(offset_of!(Body, center_x), 0x210);
        assert_eq!(offset_of!(Body, center_y), 0x214);
        assert_eq!(offset_of!(Body, unknown_0218h), 0x218);
        assert_eq!(offset_of!(Body, refuel_counter), 0x24c);
        assert_eq!(offset_of!(Body, aimer_creature_id), 0x250);
        assert_eq!(offset_of!(Body, survivors_penalty), 0x258);
        assert_eq!(offset_of!(Body, is_right_direction), 0x259);
        assert_eq!(offset_of!(Body, unknown_025ah), 0x25a);
        assert_eq!(offset_of!(Body, ship_convertion_error), 0x270);
        assert_eq!(offset_of!(Body, unknown_0290h), 0x290);
        assert_eq!(offset_of!(Body, collisions), 0x2b8);
        assert_eq!(offset_of!(Body, unknown_02bch), 0x2bc);
        assert_eq!(offset_of!(Body, ttl), 0x2c0);
        assert_eq!(offset_of!(Body, ttl_desolvetime), 0x2c4);
        assert_eq!(offset_of!(Body, health), 0x2c8);
        assert_eq!(offset_of!(Body, health_max), 0x2cc);
        assert_eq!(offset_of!(Body, health_lock), 0x2d0);
        assert_eq!(offset_of!(Body, unknown_02d1h), 0x2d1);
        assert_eq!(offset_of!(Body, contamination), 0x2d4);
        assert_eq!(offset_of!(Body, unknown_02d8h), 0x2d8);
        assert_eq!(offset_of!(Body, explosive_power), 0x2dc);
        assert_eq!(offset_of!(Body, armor_piercing), 0x2e0);
        assert_eq!(offset_of!(Body, explosive_type), 0x2e4);
        assert_eq!(offset_of!(Body, explosive_temperature), 0x2e8);
        assert_eq!(offset_of!(Body, unknown_02ech), 0x2ec);
        assert_eq!(offset_of!(Body, burn_hp), 0x334);
        assert_eq!(offset_of!(Body, routs), 0x338);
        assert_eq!(offset_of!(Body, slots), 0x350);
        assert_eq!(offset_of!(Body, slot_type), 0x368);
        assert_eq!(offset_of!(Body, spasmcode), 0x370);
        assert_eq!(offset_of!(Body, unknown_0388h), 0x388);
        assert_eq!(offset_of!(Body, collision_sensor), 0x38c);
        assert_eq!(offset_of!(Body, unknown_0390h), 0x390);
        assert_eq!(offset_of!(Body, logic), 0x3c4);
        assert_eq!(offset_of!(Body, logic_active), 0x3c8);
        assert_eq!(offset_of!(Body, unknown_03c9h), 0x3c9);
        assert_eq!(offset_of!(Body, logic_state), 0x3cc);
        assert_eq!(offset_of!(Body, logic_f1), 0x3d0);
        assert_eq!(offset_of!(Body, logic_f2), 0x3d4);
        assert_eq!(offset_of!(Body, logic_f3), 0x3d8);
        assert_eq!(offset_of!(Body, logic_f4), 0x3dc);
        assert_eq!(offset_of!(Body, unknown_03e0h), 0x3e0);
        assert_eq!(offset_of!(Body, logic_i1), 0x3e4);
        assert_eq!(offset_of!(Body, logic_i2), 0x3e8);
        assert_eq!(offset_of!(Body, logic_i3), 0x3ec);
        assert_eq!(offset_of!(Body, logic_i4), 0x3f0);
        assert_eq!(offset_of!(Body, unknown_03f4h), 0x3f4);
        assert_eq!(offset_of!(Body, logic_s1), 0x3f8);
        assert_eq!(offset_of!(Body, unknown_0418h), 0x418);
        assert_eq!(offset_of!(Body, logic_l1_saved), 0x4d8);
        assert_eq!(offset_of!(Body, logic_editor), 0x4f0);
        assert_eq!(offset_of!(Body, sectors), 0x508);
        assert_eq!(offset_of!(Body, unknown_0520h), 0x520);
        assert_eq!(offset_of!(Body, count), 0x7a8);
        assert_eq!(offset_of!(Body, is_loot), 0x7ac);
        assert_eq!(offset_of!(Body, is_blueprint), 0x7ad);
        assert_eq!(offset_of!(Body, unknown_07aeh), 0x7ae);
        assert_eq!(offset_of!(Body, is_wheel), 0x7af);
        assert_eq!(offset_of!(Body, is_leg), 0x7b0);
        assert_eq!(offset_of!(Body, unknown_07b1h), 0x7b1);
        assert_eq!(offset_of!(Body, mdl_fuel), 0x7f8);
        assert_eq!(offset_of!(Body, mdl_fss), 0x800);
        assert_eq!(offset_of!(Body, mdl_crew), 0x804);
        assert_eq!(offset_of!(Body, mdl_side), 0x808);
        assert_eq!(offset_of!(Body, unknown_080ch), 0x80c);
        assert_eq!(offset_of!(Body, mdl_pump), 0x838);
        assert_eq!(offset_of!(Body, mdl_turbine), 0x83c);
        assert_eq!(offset_of!(Body, unknown_0840h), 0x840);
        assert_eq!(offset_of!(Body, mdl_covered), 0x848);
        assert_eq!(offset_of!(Body, weapon_type), 0x84c);
        assert_eq!(offset_of!(Body, weapon_ammo), 0x850);
        assert_eq!(offset_of!(Body, weapon_ammo_extra), 0x854);
        assert_eq!(offset_of!(Body, weapon_ammo_amount), 0x858);
        assert_eq!(offset_of!(Body, weapon_ammo_extra_amount), 0x85c);
        assert_eq!(offset_of!(Body, unknown_0860h), 0x860);
        assert_eq!(offset_of!(Body, rescue_explosive), 0x877);
        assert_eq!(offset_of!(Body, rescue_flameable), 0x878);
        assert_eq!(offset_of!(Body, rescue_radioactive), 0x879);
        assert_eq!(offset_of!(Body, rescue_toxic), 0x87a);
        assert_eq!(offset_of!(Body, rescue_scare), 0x87b);
        assert_eq!(offset_of!(Body, unknown_087ch), 0x87c);
        assert_eq!(offset_of!(Body, is_crt_screen), 0x884);
        assert_eq!(offset_of!(Body, unknown_0885h), 0x885);
        assert_eq!(offset_of!(Body, repair_type), 0x888);
        assert_eq!(offset_of!(Body, repair_pos), 0x88c);
        assert_eq!(offset_of!(Body, craft_activated), 0x890);
        assert_eq!(offset_of!(Body, unknown_0891h), 0x891);
        assert_eq!(offset_of!(Body, mask), 0x8b8);
        assert_eq!(offset_of!(Body, sprites), 0x8c0);
        assert_eq!(offset_of!(Body, stage), 0x8d8);
        assert_eq!(offset_of!(Body, unknown_08dch), 0x8dc);
        assert_eq!(offset_of!(Body, mesh_render), 0x8ec);
        assert_eq!(offset_of!(Body, unknown_08edh), 0x8ed);
        assert_eq!(offset_of!(Body, mesh_color), 0x8f0);
        assert_eq!(offset_of!(Body, mask_mesh), 0x8f4);
        assert_eq!(offset_of!(Body, unknown_08f5h), 0x8f5);
        assert_eq!(offset_of!(Body, opacity), 0x8f8);
        assert_eq!(offset_of!(Body, unknown_08fch), 0x8fc);
        assert_eq!(offset_of!(Body, z), 0x90c);
        assert_eq!(offset_of!(Body, unknown_0910h), 0x910);
        assert_eq!(offset_of!(Body, scalemode_max), 0x914);
        assert_eq!(offset_of!(Body, scalemode_min), 0x918);
        assert_eq!(offset_of!(Body, gui), 0x91c);
        assert_eq!(offset_of!(Body, scalable), 0x91d);
        assert_eq!(offset_of!(Body, unknown_091eh), 0x91e);
        assert_eq!(offset_of!(Body, issuperlay), 0x91f);
        assert_eq!(offset_of!(Body, scale_x), 0x920);
        assert_eq!(offset_of!(Body, scale_y), 0x924);
        assert_eq!(offset_of!(Body, notaproximate), 0x928);
        assert_eq!(offset_of!(Body, renderlines), 0x929);
        assert_eq!(offset_of!(Body, unknown_092ah), 0x92a);
        assert_eq!(offset_of!(Body, light_pow), 0x92c);
        assert_eq!(offset_of!(Body, light_shift_x), 0x930);
        assert_eq!(offset_of!(Body, light_shift_y), 0x934);
        assert_eq!(offset_of!(Body, light_dir_x), 0x938);
        assert_eq!(offset_of!(Body, light_dir_y), 0x93c);
        assert_eq!(offset_of!(Body, light_directional), 0x940);
        assert_eq!(offset_of!(Body, shadow), 0x944);
        assert_eq!(offset_of!(Body, unknown_0948h), 0x948);
        assert_eq!(offset_of!(Body, floor_type), 0x9b0);
        assert_eq!(offset_of!(Body, floor), 0x9b8);
        assert_eq!(offset_of!(Body, tile_group), 0x9bc);
        assert_eq!(offset_of!(Body, tile_script), 0x9c0);
        assert_eq!(offset_of!(Body, unknown_09c4h), 0x9c4);
        assert_eq!(offset_of!(Body, light_color_r), 0xa00);
        assert_eq!(offset_of!(Body, light_color_g), 0xa04);
        assert_eq!(offset_of!(Body, light_color_b), 0xa08);
        assert_eq!(offset_of!(Body, unknown_0a0ch), 0xa0c);
        assert_eq!(offset_of!(Body, light_fade), 0xa10);
        assert_eq!(offset_of!(Body, ambient), 0xa14);
        assert_eq!(offset_of!(Body, tg_price), 0xa18);
        assert_eq!(offset_of!(Body, unknown_0a1ch), 0xa1c);
        assert_eq!(offset_of!(Body, aircraft_logic), 0xa20);
        assert_eq!(offset_of!(Body, ed_rotate), 0xa28);
        assert_eq!(offset_of!(Body, unknown_0a2ch), 0xa2c);
        assert_eq!(offset_of!(Body, help_index), 0xa34);
        assert_eq!(offset_of!(Body, focusable), 0xa38);
        assert_eq!(offset_of!(Body, dragable), 0xa39);
        assert_eq!(offset_of!(Body, slideable), 0xa3a);
        assert_eq!(offset_of!(Body, wheelable), 0xa3b);
        assert_eq!(offset_of!(Body, selectable), 0xa3c);
        assert_eq!(offset_of!(Body, deletable), 0xa3d);
        assert_eq!(offset_of!(Body, unknown_0a3eh), 0xa3e);
    }

    #[test]
    fn explicit_fields_cover_every_byte_without_implicit_padding() {
        let fields: &[(usize, usize)] = &[
            (offset_of!(Body, base), size_of::<Node>()),
            (offset_of!(Body, position_x), size_of::<f32>()),
            (offset_of!(Body, position_y), size_of::<f32>()),
            (offset_of!(Body, velocity_x), size_of::<f32>()),
            (offset_of!(Body, velocity_y), size_of::<f32>()),
            (offset_of!(Body, angle), size_of::<f32>()),
            (offset_of!(Body, ang_velocity), size_of::<f32>()),
            (offset_of!(Body, unknown_0140h), size_of::<[u8; 0x48]>()),
            (offset_of!(Body, part_index), size_of::<i32>()),
            (offset_of!(Body, unknown_018ch), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, oid), size_of::<EscadraString>()),
            (offset_of!(Body, b2bstate), size_of::<i32>()),
            (offset_of!(Body, unknown_01b4h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, mesh), size_of::<*mut ISerializable>()),
            (offset_of!(Body, meta), size_of::<u64>()),
            (offset_of!(Body, material), size_of::<u64>()),
            (offset_of!(Body, density), size_of::<f32>()),
            (offset_of!(Body, friction), size_of::<f32>()),
            (offset_of!(Body, mass), size_of::<f32>()),
            (offset_of!(Body, drag_lin), size_of::<f32>()),
            (offset_of!(Body, drag_ang), size_of::<f32>()),
            (offset_of!(Body, unknown_01e4h), size_of::<[u8; 0xc]>()),
            (offset_of!(Body, layer), size_of::<i32>()),
            (offset_of!(Body, unknown_01f4h), size_of::<[u8; 0x8]>()),
            (offset_of!(Body, bullet), size_of::<u8>()),
            (offset_of!(Body, unknown_01fdh), size_of::<[u8; 0xb]>()),
            (offset_of!(Body, is_physic_update), size_of::<u8>()),
            (offset_of!(Body, unknown_0209h), size_of::<[u8; 0x7]>()),
            (offset_of!(Body, center_x), size_of::<f32>()),
            (offset_of!(Body, center_y), size_of::<f32>()),
            (offset_of!(Body, unknown_0218h), size_of::<[u8; 0x34]>()),
            (offset_of!(Body, refuel_counter), size_of::<f32>()),
            (offset_of!(Body, aimer_creature_id), size_of::<i64>()),
            (offset_of!(Body, survivors_penalty), size_of::<u8>()),
            (offset_of!(Body, is_right_direction), size_of::<u8>()),
            (offset_of!(Body, unknown_025ah), size_of::<[u8; 0x16]>()),
            (
                offset_of!(Body, ship_convertion_error),
                size_of::<EscadraString>(),
            ),
            (offset_of!(Body, unknown_0290h), size_of::<[u8; 0x28]>()),
            (offset_of!(Body, collisions), size_of::<i32>()),
            (offset_of!(Body, unknown_02bch), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, ttl), size_of::<f32>()),
            (offset_of!(Body, ttl_desolvetime), size_of::<f32>()),
            (offset_of!(Body, health), size_of::<f32>()),
            (offset_of!(Body, health_max), size_of::<f32>()),
            (offset_of!(Body, health_lock), size_of::<u8>()),
            (offset_of!(Body, unknown_02d1h), size_of::<[u8; 0x3]>()),
            (offset_of!(Body, contamination), size_of::<f32>()),
            (offset_of!(Body, unknown_02d8h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, explosive_power), size_of::<f32>()),
            (offset_of!(Body, armor_piercing), size_of::<f32>()),
            (offset_of!(Body, explosive_type), size_of::<i32>()),
            (offset_of!(Body, explosive_temperature), size_of::<f32>()),
            (offset_of!(Body, unknown_02ech), size_of::<[u8; 0x48]>()),
            (offset_of!(Body, burn_hp), size_of::<f32>()),
            (
                offset_of!(Body, routs),
                size_of::<CVec<*mut ISerializable>>(),
            ),
            (
                offset_of!(Body, slots),
                size_of::<CVec<*mut ISerializable>>(),
            ),
            (offset_of!(Body, slot_type), size_of::<u64>()),
            (offset_of!(Body, spasmcode), size_of::<CVec<i32>>()),
            (offset_of!(Body, unknown_0388h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, collision_sensor), size_of::<f32>()),
            (offset_of!(Body, unknown_0390h), size_of::<[u8; 0x34]>()),
            (offset_of!(Body, logic), size_of::<i32>()),
            (offset_of!(Body, logic_active), size_of::<u8>()),
            (offset_of!(Body, unknown_03c9h), size_of::<[u8; 0x3]>()),
            (offset_of!(Body, logic_state), size_of::<u32>()),
            (offset_of!(Body, logic_f1), size_of::<f32>()),
            (offset_of!(Body, logic_f2), size_of::<f32>()),
            (offset_of!(Body, logic_f3), size_of::<f32>()),
            (offset_of!(Body, logic_f4), size_of::<f32>()),
            (offset_of!(Body, unknown_03e0h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, logic_i1), size_of::<i32>()),
            (offset_of!(Body, logic_i2), size_of::<i32>()),
            (offset_of!(Body, logic_i3), size_of::<i32>()),
            (offset_of!(Body, logic_i4), size_of::<i32>()),
            (offset_of!(Body, unknown_03f4h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, logic_s1), size_of::<EscadraString>()),
            (offset_of!(Body, unknown_0418h), size_of::<[u8; 0xc0]>()),
            (offset_of!(Body, logic_l1_saved), size_of::<Link>()),
            (offset_of!(Body, logic_editor), size_of::<CVec<i32>>()),
            (offset_of!(Body, sectors), size_of::<CVec<f32>>()),
            (offset_of!(Body, unknown_0520h), size_of::<[u8; 0x288]>()),
            (offset_of!(Body, count), size_of::<i32>()),
            (offset_of!(Body, is_loot), size_of::<u8>()),
            (offset_of!(Body, is_blueprint), size_of::<u8>()),
            (offset_of!(Body, unknown_07aeh), size_of::<[u8; 0x1]>()),
            (offset_of!(Body, is_wheel), size_of::<u8>()),
            (offset_of!(Body, is_leg), size_of::<u8>()),
            (offset_of!(Body, unknown_07b1h), size_of::<[u8; 0x47]>()),
            (offset_of!(Body, mdl_fuel), size_of::<f64>()),
            (offset_of!(Body, mdl_fss), size_of::<f32>()),
            (offset_of!(Body, mdl_crew), size_of::<i32>()),
            (offset_of!(Body, mdl_side), size_of::<i32>()),
            (offset_of!(Body, unknown_080ch), size_of::<[u8; 0x2c]>()),
            (offset_of!(Body, mdl_pump), size_of::<f32>()),
            (offset_of!(Body, mdl_turbine), size_of::<f32>()),
            (offset_of!(Body, unknown_0840h), size_of::<[u8; 0x8]>()),
            (offset_of!(Body, mdl_covered), size_of::<f32>()),
            (offset_of!(Body, weapon_type), size_of::<i32>()),
            (offset_of!(Body, weapon_ammo), size_of::<i32>()),
            (offset_of!(Body, weapon_ammo_extra), size_of::<i32>()),
            (offset_of!(Body, weapon_ammo_amount), size_of::<i32>()),
            (offset_of!(Body, weapon_ammo_extra_amount), size_of::<i32>()),
            (offset_of!(Body, unknown_0860h), size_of::<[u8; 0x17]>()),
            (offset_of!(Body, rescue_explosive), size_of::<u8>()),
            (offset_of!(Body, rescue_flameable), size_of::<u8>()),
            (offset_of!(Body, rescue_radioactive), size_of::<u8>()),
            (offset_of!(Body, rescue_toxic), size_of::<u8>()),
            (offset_of!(Body, rescue_scare), size_of::<u8>()),
            (offset_of!(Body, unknown_087ch), size_of::<[u8; 0x8]>()),
            (offset_of!(Body, is_crt_screen), size_of::<u8>()),
            (offset_of!(Body, unknown_0885h), size_of::<[u8; 0x3]>()),
            (offset_of!(Body, repair_type), size_of::<i32>()),
            (offset_of!(Body, repair_pos), size_of::<i32>()),
            (offset_of!(Body, craft_activated), size_of::<u8>()),
            (offset_of!(Body, unknown_0891h), size_of::<[u8; 0x27]>()),
            (offset_of!(Body, mask), size_of::<*mut ISerializable>()),
            (
                offset_of!(Body, sprites),
                size_of::<CVec<*mut ISerializable>>(),
            ),
            (offset_of!(Body, stage), size_of::<i32>()),
            (offset_of!(Body, unknown_08dch), size_of::<[u8; 0x10]>()),
            (offset_of!(Body, mesh_render), size_of::<u8>()),
            (offset_of!(Body, unknown_08edh), size_of::<[u8; 0x3]>()),
            (offset_of!(Body, mesh_color), size_of::<u32>()),
            (offset_of!(Body, mask_mesh), size_of::<u8>()),
            (offset_of!(Body, unknown_08f5h), size_of::<[u8; 0x3]>()),
            (offset_of!(Body, opacity), size_of::<f32>()),
            (offset_of!(Body, unknown_08fch), size_of::<[u8; 0x10]>()),
            (offset_of!(Body, z), size_of::<f32>()),
            (offset_of!(Body, unknown_0910h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, scalemode_max), size_of::<i32>()),
            (offset_of!(Body, scalemode_min), size_of::<i32>()),
            (offset_of!(Body, gui), size_of::<u8>()),
            (offset_of!(Body, scalable), size_of::<u8>()),
            (offset_of!(Body, unknown_091eh), size_of::<[u8; 0x1]>()),
            (offset_of!(Body, issuperlay), size_of::<u8>()),
            (offset_of!(Body, scale_x), size_of::<f32>()),
            (offset_of!(Body, scale_y), size_of::<f32>()),
            (offset_of!(Body, notaproximate), size_of::<u8>()),
            (offset_of!(Body, renderlines), size_of::<u8>()),
            (offset_of!(Body, unknown_092ah), size_of::<[u8; 0x2]>()),
            (offset_of!(Body, light_pow), size_of::<f32>()),
            (offset_of!(Body, light_shift_x), size_of::<f32>()),
            (offset_of!(Body, light_shift_y), size_of::<f32>()),
            (offset_of!(Body, light_dir_x), size_of::<f32>()),
            (offset_of!(Body, light_dir_y), size_of::<f32>()),
            (offset_of!(Body, light_directional), size_of::<f32>()),
            (offset_of!(Body, shadow), size_of::<f32>()),
            (offset_of!(Body, unknown_0948h), size_of::<[u8; 0x68]>()),
            (offset_of!(Body, floor_type), size_of::<u64>()),
            (offset_of!(Body, floor), size_of::<i32>()),
            (offset_of!(Body, tile_group), size_of::<i32>()),
            (offset_of!(Body, tile_script), size_of::<i32>()),
            (offset_of!(Body, unknown_09c4h), size_of::<[u8; 0x3c]>()),
            (offset_of!(Body, light_color_r), size_of::<f32>()),
            (offset_of!(Body, light_color_g), size_of::<f32>()),
            (offset_of!(Body, light_color_b), size_of::<f32>()),
            (offset_of!(Body, unknown_0a0ch), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, light_fade), size_of::<f32>()),
            (offset_of!(Body, ambient), size_of::<f32>()),
            (offset_of!(Body, tg_price), size_of::<f32>()),
            (offset_of!(Body, unknown_0a1ch), size_of::<[u8; 0x4]>()),
            (
                offset_of!(Body, aircraft_logic),
                size_of::<*mut ISerializable>(),
            ),
            (offset_of!(Body, ed_rotate), size_of::<f32>()),
            (offset_of!(Body, unknown_0a2ch), size_of::<[u8; 0x8]>()),
            (offset_of!(Body, help_index), size_of::<i32>()),
            (offset_of!(Body, focusable), size_of::<u8>()),
            (offset_of!(Body, dragable), size_of::<u8>()),
            (offset_of!(Body, slideable), size_of::<u8>()),
            (offset_of!(Body, wheelable), size_of::<u8>()),
            (offset_of!(Body, selectable), size_of::<u8>()),
            (offset_of!(Body, deletable), size_of::<u8>()),
            (offset_of!(Body, unknown_0a3eh), size_of::<[u8; 0x2]>()),
        ];
        let mut end = 0;
        for &(offset, width) in fields {
            assert_eq!(offset, end);
            end = offset + width;
        }
        assert_eq!(end, size_of::<Body>());
    }
}
