//! Native Body view for Steam 1.151, recovered from custom serialization and runtime accesses.

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
    /// Uninterpreted native state at `0x140..0x190`; not disposable padding.
    pub unknown_0140h: [u8; 0x50],
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
    /// Uninterpreted native state at `0x1F4..0x1F8`; not disposable padding.
    pub unknown_01f4h: [u8; 0x4],
    /// Serialized as `m_bullet`; native storage at `0x1F8`.
    /// Raw flag byte, not a Rust `bool`.
    pub bullet: u8,
    /// Uninterpreted native state at `0x1F9..0x204`; not disposable padding.
    pub unknown_01f9h: [u8; 0xb],
    /// Serialized as `m_isPhysicUpdate`; native storage at `0x204`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_physic_update: u8,
    /// Uninterpreted native state at `0x205..0x20C`; not disposable padding.
    pub unknown_0205h: [u8; 0x7],
    /// Serialized as `m_center.x`; native storage at `0x20C`.
    pub center_x: f32,
    /// Serialized as `m_center.y`; native storage at `0x210`.
    pub center_y: f32,
    /// Uninterpreted native state at `0x214..0x244`; not disposable padding.
    pub unknown_0214h: [u8; 0x30],
    /// Serialized as `refuelCounter`; native storage at `0x244`.
    pub refuel_counter: f32,
    /// Serialized as `aimerCreatureId`; native storage at `0x248`.
    pub aimer_creature_id: u64,
    /// Serialized as `survivorsPenalty`; native storage at `0x250`.
    /// Raw flag byte, not a Rust `bool`.
    pub survivors_penalty: u8,
    /// Serialized as `isRightDirection`; native storage at `0x251`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_right_direction: u8,
    /// Uninterpreted native state at `0x252..0x268`; not disposable padding.
    pub unknown_0252h: [u8; 0x16],
    /// Serialized as `shipConvertionError`; native storage at `0x268`.
    pub ship_convertion_error: EscadraString,
    /// Uninterpreted native state at `0x288..0x2A8`; not disposable padding.
    pub unknown_0288h: [u8; 0x20],
    /// Serialized as `m_collisions`; native storage at `0x2A8`.
    pub collisions: i32,
    /// Uninterpreted native state at `0x2AC..0x2B0`; not disposable padding.
    pub unknown_02ach: [u8; 0x4],
    /// Serialized as `m_ttl`; native storage at `0x2B0`.
    pub ttl: f32,
    /// Serialized as `m_ttl_desolvetime`; native storage at `0x2B4`.
    pub ttl_desolvetime: f32,
    /// Serialized as `m_health`; native storage at `0x2B8`.
    pub health: f32,
    /// Serialized as `m_health_max`; native storage at `0x2BC`.
    pub health_max: f32,
    /// Serialized as `m_health_lock`; native storage at `0x2C0`.
    /// Raw flag byte, not a Rust `bool`.
    pub health_lock: u8,
    /// Uninterpreted native state at `0x2C1..0x2C4`; not disposable padding.
    pub unknown_02c1h: [u8; 0x3],
    /// Serialized as `m_contamination`; native storage at `0x2C4`.
    pub contamination: f32,
    /// Uninterpreted native state at `0x2C8..0x2CC`; not disposable padding.
    pub unknown_02c8h: [u8; 0x4],
    /// Serialized as `m_explosive`; native storage at `0x2CC`.
    /// Explosive power, not final HP damage.
    pub explosive_power: f32,
    /// Serialized as `m_armorPiercing`; native storage at `0x2D0`.
    pub armor_piercing: f32,
    /// Serialized as `m_explosive_type`; native storage at `0x2D4`.
    pub explosive_type: i32,
    /// Serialized as `m_explosive_temperature`; native storage at `0x2D8`.
    pub explosive_temperature: f32,
    /// Uninterpreted native state at `0x2DC..0x324`; not disposable padding.
    pub unknown_02dch: [u8; 0x48],
    /// Serialized as `m_burn_hp`; native storage at `0x324`.
    pub burn_hp: f32,
    /// Serialized as `m_routs`; native storage at `0x328`.
    pub routs: CVec<*mut ISerializable>,
    /// Serialized as `m_slots`; native storage at `0x340`.
    pub slots: CVec<*mut ISerializable>,
    /// Serialized as `m_slot_type`; native storage at `0x358`.
    pub slot_type: u64,
    /// Serialized as `m_spasmcode`; native storage at `0x360`.
    pub spasmcode: CVec<i32>,
    /// Uninterpreted native state at `0x378..0x37C`; not disposable padding.
    pub unknown_0378h: [u8; 0x4],
    /// Serialized as `m_collision_sensor`; native storage at `0x37C`.
    pub collision_sensor: f32,
    /// Uninterpreted native state at `0x380..0x3B4`; not disposable padding.
    pub unknown_0380h: [u8; 0x34],
    /// Serialized as `m_logic`; native storage at `0x3B4`.
    pub logic: i32,
    /// Serialized as `m_logic_active`; native storage at `0x3B8`.
    /// Raw flag byte, not a Rust `bool`.
    pub logic_active: u8,
    /// Uninterpreted native state at `0x3B9..0x3BC`; not disposable padding.
    pub unknown_03b9h: [u8; 0x3],
    /// Runtime logic state-machine storage; original signedness is not established.
    pub logic_state: u32,
    /// Serialized as `m_logic_f1`; native storage at `0x3C0`.
    /// Logic-specific countdown; the shrapnel fuse for logic 150.
    pub logic_f1: f32,
    /// Serialized as `m_logic_f2`; native storage at `0x3C4`.
    pub logic_f2: f32,
    /// Serialized as `m_logic_f3`; native storage at `0x3C8`.
    pub logic_f3: f32,
    /// Serialized as `m_logic_f4`; native storage at `0x3CC`.
    pub logic_f4: f32,
    /// Uninterpreted native state at `0x3D0..0x3D4`; not disposable padding.
    pub unknown_03d0h: [u8; 0x4],
    /// Serialized as `m_logic_i1`; native storage at `0x3D4`.
    pub logic_i1: i32,
    /// Serialized as `m_logic_i2`; native storage at `0x3D8`.
    pub logic_i2: i32,
    /// Serialized as `m_logic_i3`; native storage at `0x3DC`.
    pub logic_i3: i32,
    /// Serialized as `m_logic_i4`; native storage at `0x3E0`.
    pub logic_i4: i32,
    /// Uninterpreted native state at `0x3E4..0x3E8`; not disposable padding.
    pub unknown_03e4h: [u8; 0x4],
    /// Serialized as `m_logic_s1`; native storage at `0x3E8`.
    pub logic_s1: EscadraString,
    /// Uninterpreted native state at `0x408..0x4C8`; not disposable padding.
    pub unknown_0408h: [u8; 0xc0],
    /// Native Link whose `saved_id` member serializes as `m_logic_l1_saved.id`.
    pub logic_l1_saved: Link,
    /// Serialized as `m_logic_editor`; native storage at `0x4E0`.
    pub logic_editor: CVec<i32>,
    /// Serialized as `m_sectors`; native storage at `0x4F8`.
    pub sectors: CVec<f32>,
    /// Uninterpreted native state at `0x510..0x798`; not disposable padding.
    pub unknown_0510h: [u8; 0x288],
    /// Serialized as `m_count`; native storage at `0x798`.
    pub count: i32,
    /// Serialized as `is_loot`; native storage at `0x79C`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_loot: u8,
    /// Serialized as `is_blueprint`; native storage at `0x79D`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_blueprint: u8,
    /// Uninterpreted native state at `0x79E..0x79F`; not disposable padding.
    pub unknown_079eh: [u8; 0x1],
    /// Serialized as `is_wheel`; native storage at `0x79F`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_wheel: u8,
    /// Serialized as `is_leg`; native storage at `0x7A0`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_leg: u8,
    /// Uninterpreted native state at `0x7A1..0x7E8`; not disposable padding.
    pub unknown_07a1h: [u8; 0x47],
    /// Serialized as `m_mdl_fuel`; native storage at `0x7E8`.
    pub mdl_fuel: f64,
    /// Serialized as `m_mdl_fss`; native storage at `0x7F0`.
    pub mdl_fss: f32,
    /// Serialized as `m_mdl_crew`; native storage at `0x7F4`.
    pub mdl_crew: i32,
    /// Serialized as `m_mdl_side`; native storage at `0x7F8`.
    pub mdl_side: i32,
    /// Uninterpreted native state at `0x7FC..0x828`; not disposable padding.
    pub unknown_07fch: [u8; 0x2c],
    /// Serialized as `m_mdl_pump`; native storage at `0x828`.
    pub mdl_pump: f32,
    /// Serialized as `m_mdl_turbine`; native storage at `0x82C`.
    pub mdl_turbine: f32,
    /// Uninterpreted native state at `0x830..0x834`; not disposable padding.
    pub unknown_0830h: [u8; 0x4],
    /// Serialized as `m_mdl_covered`; native storage at `0x834`.
    pub mdl_covered: f32,
    /// Serialized as `m_weapon_type`; native storage at `0x838`.
    pub weapon_type: i32,
    /// Serialized as `m_weapon_ammo`; native storage at `0x83C`.
    pub weapon_ammo: i32,
    /// Serialized as `m_weapon_ammo_extra`; native storage at `0x840`.
    pub weapon_ammo_extra: i32,
    /// Serialized as `m_weapon_ammo_amount`; native storage at `0x844`.
    pub weapon_ammo_amount: i32,
    /// Serialized as `m_weapon_ammo_extra_amount`; native storage at `0x848`.
    pub weapon_ammo_extra_amount: i32,
    /// Uninterpreted native state at `0x84C..0x862`; not disposable padding.
    pub unknown_084ch: [u8; 0x16],
    /// Serialized as `m_rescue_explosive`; native storage at `0x862`.
    /// Raw flag byte, not a Rust `bool`.
    pub rescue_explosive: u8,
    /// Serialized as `m_rescue_flameable`; native storage at `0x863`.
    /// Raw flag byte, not a Rust `bool`.
    pub rescue_flameable: u8,
    /// Serialized as `m_rescue_radioactive`; native storage at `0x864`.
    /// Raw flag byte, not a Rust `bool`.
    pub rescue_radioactive: u8,
    /// Serialized as `m_rescue_toxic`; native storage at `0x865`.
    /// Raw flag byte, not a Rust `bool`.
    pub rescue_toxic: u8,
    /// Serialized as `m_rescue_scare`; native storage at `0x866`.
    /// Raw flag byte, not a Rust `bool`.
    pub rescue_scare: u8,
    /// Uninterpreted native state at `0x867..0x870`; not disposable padding.
    pub unknown_0867h: [u8; 0x9],
    /// Serialized as `isCRTScreen`; native storage at `0x870`.
    /// Raw flag byte, not a Rust `bool`.
    pub is_crt_screen: u8,
    /// Uninterpreted native state at `0x871..0x874`; not disposable padding.
    pub unknown_0871h: [u8; 0x3],
    /// Serialized as `repairType`; native storage at `0x874`.
    pub repair_type: i32,
    /// Serialized as `repairPos`; native storage at `0x878`.
    pub repair_pos: i32,
    /// Serialized as `craftActivated`; native storage at `0x87C`.
    /// Raw flag byte, not a Rust `bool`.
    pub craft_activated: u8,
    /// Uninterpreted native state at `0x87D..0x880`; not disposable padding.
    pub unknown_087dh: [u8; 0x3],
    /// Serialized as `m_mask`; native storage at `0x880`.
    pub mask: *mut ISerializable,
    /// Serialized as `m_sprites`; native storage at `0x888`.
    pub sprites: CVec<*mut ISerializable>,
    /// Serialized as `m_stage`; native storage at `0x8A0`.
    pub stage: i32,
    /// Uninterpreted native state at `0x8A4..0x8B4`; not disposable padding.
    pub unknown_08a4h: [u8; 0x10],
    /// Serialized as `m_mesh_render`; native storage at `0x8B4`.
    /// Raw flag byte, not a Rust `bool`.
    pub mesh_render: u8,
    /// Uninterpreted native state at `0x8B5..0x8B8`; not disposable padding.
    pub unknown_08b5h: [u8; 0x3],
    /// Serialized as `m_mesh_color`; native storage at `0x8B8`.
    pub mesh_color: u32,
    /// Serialized as `m_mask_mesh`; native storage at `0x8BC`.
    /// Raw flag byte, not a Rust `bool`.
    pub mask_mesh: u8,
    /// Uninterpreted native state at `0x8BD..0x8C0`; not disposable padding.
    pub unknown_08bdh: [u8; 0x3],
    /// Serialized as `m_opacity`; native storage at `0x8C0`.
    pub opacity: f32,
    /// Uninterpreted native state at `0x8C4..0x8D4`; not disposable padding.
    pub unknown_08c4h: [u8; 0x10],
    /// Serialized as `m_z`; native storage at `0x8D4`.
    pub z: f32,
    /// Uninterpreted native state at `0x8D8..0x8DC`; not disposable padding.
    pub unknown_08d8h: [u8; 0x4],
    /// Serialized as `m_scalemode_max`; native storage at `0x8DC`.
    pub scalemode_max: i32,
    /// Serialized as `m_scalemode_min`; native storage at `0x8E0`.
    pub scalemode_min: i32,
    /// Serialized as `m_gui`; native storage at `0x8E4`.
    /// Raw flag byte, not a Rust `bool`.
    pub gui: u8,
    /// Serialized as `m_scalable`; native storage at `0x8E5`.
    /// Raw flag byte, not a Rust `bool`.
    pub scalable: u8,
    /// Uninterpreted native state at `0x8E6..0x8E7`; not disposable padding.
    pub unknown_08e6h: [u8; 0x1],
    /// Serialized as `m_issuperlay`; native storage at `0x8E7`.
    /// Raw flag byte, not a Rust `bool`.
    pub issuperlay: u8,
    /// Serialized as `m_scale.x`; native storage at `0x8E8`.
    pub scale_x: f32,
    /// Serialized as `m_scale.y`; native storage at `0x8EC`.
    pub scale_y: f32,
    /// Serialized as `m_notaproximate`; native storage at `0x8F0`.
    /// Raw flag byte, not a Rust `bool`.
    pub notaproximate: u8,
    /// Serialized as `m_renderlines`; native storage at `0x8F1`.
    /// Raw flag byte, not a Rust `bool`.
    pub renderlines: u8,
    /// Uninterpreted native state at `0x8F2..0x8F4`; not disposable padding.
    pub unknown_08f2h: [u8; 0x2],
    /// Serialized as `m_light_pow`; native storage at `0x8F4`.
    pub light_pow: f32,
    /// Serialized as `m_light_shift.x`; native storage at `0x8F8`.
    pub light_shift_x: f32,
    /// Serialized as `m_light_shift.y`; native storage at `0x8FC`.
    pub light_shift_y: f32,
    /// Serialized as `m_light_dir.x`; native storage at `0x900`.
    pub light_dir_x: f32,
    /// Serialized as `m_light_dir.y`; native storage at `0x904`.
    pub light_dir_y: f32,
    /// Serialized as `m_light_directional`; native storage at `0x908`.
    pub light_directional: f32,
    /// Serialized as `m_shadow`; native storage at `0x90C`.
    pub shadow: f32,
    /// Uninterpreted native state at `0x910..0x978`; not disposable padding.
    pub unknown_0910h: [u8; 0x68],
    /// Serialized as `m_floor_type`; native storage at `0x978`.
    pub floor_type: u64,
    /// Serialized as `m_floor`; native storage at `0x980`.
    pub floor: i32,
    /// Serialized as `m_tile_group`; native storage at `0x984`.
    pub tile_group: i32,
    /// Serialized as `m_tile_script`; native storage at `0x988`.
    pub tile_script: i32,
    /// Uninterpreted native state at `0x98C..0x9C8`; not disposable padding.
    pub unknown_098ch: [u8; 0x3c],
    /// Serialized as `m_light_color.r`; native storage at `0x9C8`.
    pub light_color_r: f32,
    /// Serialized as `m_light_color.g`; native storage at `0x9CC`.
    pub light_color_g: f32,
    /// Serialized as `m_light_color.b`; native storage at `0x9D0`.
    pub light_color_b: f32,
    /// Uninterpreted native state at `0x9D4..0x9D8`; not disposable padding.
    pub unknown_09d4h: [u8; 0x4],
    /// Serialized as `m_light_fade`; native storage at `0x9D8`.
    pub light_fade: f32,
    /// Serialized as `m_ambient`; native storage at `0x9DC`.
    pub ambient: f32,
    /// Serialized as `m_tg_price`; native storage at `0x9E0`.
    pub tg_price: f32,
    /// Uninterpreted native state at `0x9E4..0x9E8`; not disposable padding.
    pub unknown_09e4h: [u8; 0x4],
    /// Serialized as `aircraftLogic`; native storage at `0x9E8`.
    pub aircraft_logic: *mut ISerializable,
    /// Serialized as `m_ed_rotate`; native storage at `0x9F0`.
    pub ed_rotate: f32,
    /// Uninterpreted native state at `0x9F4..0x9FC`; not disposable padding.
    pub unknown_09f4h: [u8; 0x8],
    /// Serialized as `m_help_index`; native storage at `0x9FC`.
    pub help_index: i32,
    /// Serialized as `m_focusable`; native storage at `0xA00`.
    /// Raw flag byte, not a Rust `bool`.
    pub focusable: u8,
    /// Serialized as `m_dragable`; native storage at `0xA01`.
    /// Raw flag byte, not a Rust `bool`.
    pub dragable: u8,
    /// Serialized as `m_slideable`; native storage at `0xA02`.
    /// Raw flag byte, not a Rust `bool`.
    pub slideable: u8,
    /// Serialized as `m_wheelable`; native storage at `0xA03`.
    /// Raw flag byte, not a Rust `bool`.
    pub wheelable: u8,
    /// Serialized as `m_selectable`; native storage at `0xA04`.
    /// Raw flag byte, not a Rust `bool`.
    pub selectable: u8,
    /// Serialized as `m_deletable`; native storage at `0xA05`.
    /// Raw flag byte, not a Rust `bool`.
    pub deletable: u8,
    /// Uninterpreted native state at `0xA06..0xA08`.
    pub unknown_0a06h: [u8; 0x2],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, needs_drop, offset_of, size_of};

    #[test]
    fn native_layout() {
        assert_eq!(size_of::<Body>(), 0xa08);
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
        assert_eq!(offset_of!(Body, bullet), 0x1f8);
        assert_eq!(offset_of!(Body, unknown_01f9h), 0x1f9);
        assert_eq!(offset_of!(Body, is_physic_update), 0x204);
        assert_eq!(offset_of!(Body, unknown_0205h), 0x205);
        assert_eq!(offset_of!(Body, center_x), 0x20c);
        assert_eq!(offset_of!(Body, center_y), 0x210);
        assert_eq!(offset_of!(Body, unknown_0214h), 0x214);
        assert_eq!(offset_of!(Body, refuel_counter), 0x244);
        assert_eq!(offset_of!(Body, aimer_creature_id), 0x248);
        assert_eq!(offset_of!(Body, survivors_penalty), 0x250);
        assert_eq!(offset_of!(Body, is_right_direction), 0x251);
        assert_eq!(offset_of!(Body, unknown_0252h), 0x252);
        assert_eq!(offset_of!(Body, ship_convertion_error), 0x268);
        assert_eq!(offset_of!(Body, unknown_0288h), 0x288);
        assert_eq!(offset_of!(Body, collisions), 0x2a8);
        assert_eq!(offset_of!(Body, unknown_02ach), 0x2ac);
        assert_eq!(offset_of!(Body, ttl), 0x2b0);
        assert_eq!(offset_of!(Body, ttl_desolvetime), 0x2b4);
        assert_eq!(offset_of!(Body, health), 0x2b8);
        assert_eq!(offset_of!(Body, health_max), 0x2bc);
        assert_eq!(offset_of!(Body, health_lock), 0x2c0);
        assert_eq!(offset_of!(Body, unknown_02c1h), 0x2c1);
        assert_eq!(offset_of!(Body, contamination), 0x2c4);
        assert_eq!(offset_of!(Body, unknown_02c8h), 0x2c8);
        assert_eq!(offset_of!(Body, explosive_power), 0x2cc);
        assert_eq!(offset_of!(Body, armor_piercing), 0x2d0);
        assert_eq!(offset_of!(Body, explosive_type), 0x2d4);
        assert_eq!(offset_of!(Body, explosive_temperature), 0x2d8);
        assert_eq!(offset_of!(Body, unknown_02dch), 0x2dc);
        assert_eq!(offset_of!(Body, burn_hp), 0x324);
        assert_eq!(offset_of!(Body, routs), 0x328);
        assert_eq!(offset_of!(Body, slots), 0x340);
        assert_eq!(offset_of!(Body, slot_type), 0x358);
        assert_eq!(offset_of!(Body, spasmcode), 0x360);
        assert_eq!(offset_of!(Body, unknown_0378h), 0x378);
        assert_eq!(offset_of!(Body, collision_sensor), 0x37c);
        assert_eq!(offset_of!(Body, unknown_0380h), 0x380);
        assert_eq!(offset_of!(Body, logic), 0x3b4);
        assert_eq!(offset_of!(Body, logic_active), 0x3b8);
        assert_eq!(offset_of!(Body, unknown_03b9h), 0x3b9);
        assert_eq!(offset_of!(Body, logic_state), 0x3bc);
        assert_eq!(offset_of!(Body, logic_f1), 0x3c0);
        assert_eq!(offset_of!(Body, logic_f2), 0x3c4);
        assert_eq!(offset_of!(Body, logic_f3), 0x3c8);
        assert_eq!(offset_of!(Body, logic_f4), 0x3cc);
        assert_eq!(offset_of!(Body, unknown_03d0h), 0x3d0);
        assert_eq!(offset_of!(Body, logic_i1), 0x3d4);
        assert_eq!(offset_of!(Body, logic_i2), 0x3d8);
        assert_eq!(offset_of!(Body, logic_i3), 0x3dc);
        assert_eq!(offset_of!(Body, logic_i4), 0x3e0);
        assert_eq!(offset_of!(Body, unknown_03e4h), 0x3e4);
        assert_eq!(offset_of!(Body, logic_s1), 0x3e8);
        assert_eq!(offset_of!(Body, unknown_0408h), 0x408);
        assert_eq!(offset_of!(Body, logic_l1_saved), 0x4c8);
        assert_eq!(offset_of!(Body, logic_editor), 0x4e0);
        assert_eq!(offset_of!(Body, sectors), 0x4f8);
        assert_eq!(offset_of!(Body, unknown_0510h), 0x510);
        assert_eq!(offset_of!(Body, count), 0x798);
        assert_eq!(offset_of!(Body, is_loot), 0x79c);
        assert_eq!(offset_of!(Body, is_blueprint), 0x79d);
        assert_eq!(offset_of!(Body, unknown_079eh), 0x79e);
        assert_eq!(offset_of!(Body, is_wheel), 0x79f);
        assert_eq!(offset_of!(Body, is_leg), 0x7a0);
        assert_eq!(offset_of!(Body, unknown_07a1h), 0x7a1);
        assert_eq!(offset_of!(Body, mdl_fuel), 0x7e8);
        assert_eq!(offset_of!(Body, mdl_fss), 0x7f0);
        assert_eq!(offset_of!(Body, mdl_crew), 0x7f4);
        assert_eq!(offset_of!(Body, mdl_side), 0x7f8);
        assert_eq!(offset_of!(Body, unknown_07fch), 0x7fc);
        assert_eq!(offset_of!(Body, mdl_pump), 0x828);
        assert_eq!(offset_of!(Body, mdl_turbine), 0x82c);
        assert_eq!(offset_of!(Body, unknown_0830h), 0x830);
        assert_eq!(offset_of!(Body, mdl_covered), 0x834);
        assert_eq!(offset_of!(Body, weapon_type), 0x838);
        assert_eq!(offset_of!(Body, weapon_ammo), 0x83c);
        assert_eq!(offset_of!(Body, weapon_ammo_extra), 0x840);
        assert_eq!(offset_of!(Body, weapon_ammo_amount), 0x844);
        assert_eq!(offset_of!(Body, weapon_ammo_extra_amount), 0x848);
        assert_eq!(offset_of!(Body, unknown_084ch), 0x84c);
        assert_eq!(offset_of!(Body, rescue_explosive), 0x862);
        assert_eq!(offset_of!(Body, rescue_flameable), 0x863);
        assert_eq!(offset_of!(Body, rescue_radioactive), 0x864);
        assert_eq!(offset_of!(Body, rescue_toxic), 0x865);
        assert_eq!(offset_of!(Body, rescue_scare), 0x866);
        assert_eq!(offset_of!(Body, unknown_0867h), 0x867);
        assert_eq!(offset_of!(Body, is_crt_screen), 0x870);
        assert_eq!(offset_of!(Body, unknown_0871h), 0x871);
        assert_eq!(offset_of!(Body, repair_type), 0x874);
        assert_eq!(offset_of!(Body, repair_pos), 0x878);
        assert_eq!(offset_of!(Body, craft_activated), 0x87c);
        assert_eq!(offset_of!(Body, unknown_087dh), 0x87d);
        assert_eq!(offset_of!(Body, mask), 0x880);
        assert_eq!(offset_of!(Body, sprites), 0x888);
        assert_eq!(offset_of!(Body, stage), 0x8a0);
        assert_eq!(offset_of!(Body, unknown_08a4h), 0x8a4);
        assert_eq!(offset_of!(Body, mesh_render), 0x8b4);
        assert_eq!(offset_of!(Body, unknown_08b5h), 0x8b5);
        assert_eq!(offset_of!(Body, mesh_color), 0x8b8);
        assert_eq!(offset_of!(Body, mask_mesh), 0x8bc);
        assert_eq!(offset_of!(Body, unknown_08bdh), 0x8bd);
        assert_eq!(offset_of!(Body, opacity), 0x8c0);
        assert_eq!(offset_of!(Body, unknown_08c4h), 0x8c4);
        assert_eq!(offset_of!(Body, z), 0x8d4);
        assert_eq!(offset_of!(Body, unknown_08d8h), 0x8d8);
        assert_eq!(offset_of!(Body, scalemode_max), 0x8dc);
        assert_eq!(offset_of!(Body, scalemode_min), 0x8e0);
        assert_eq!(offset_of!(Body, gui), 0x8e4);
        assert_eq!(offset_of!(Body, scalable), 0x8e5);
        assert_eq!(offset_of!(Body, unknown_08e6h), 0x8e6);
        assert_eq!(offset_of!(Body, issuperlay), 0x8e7);
        assert_eq!(offset_of!(Body, scale_x), 0x8e8);
        assert_eq!(offset_of!(Body, scale_y), 0x8ec);
        assert_eq!(offset_of!(Body, notaproximate), 0x8f0);
        assert_eq!(offset_of!(Body, renderlines), 0x8f1);
        assert_eq!(offset_of!(Body, unknown_08f2h), 0x8f2);
        assert_eq!(offset_of!(Body, light_pow), 0x8f4);
        assert_eq!(offset_of!(Body, light_shift_x), 0x8f8);
        assert_eq!(offset_of!(Body, light_shift_y), 0x8fc);
        assert_eq!(offset_of!(Body, light_dir_x), 0x900);
        assert_eq!(offset_of!(Body, light_dir_y), 0x904);
        assert_eq!(offset_of!(Body, light_directional), 0x908);
        assert_eq!(offset_of!(Body, shadow), 0x90c);
        assert_eq!(offset_of!(Body, unknown_0910h), 0x910);
        assert_eq!(offset_of!(Body, floor_type), 0x978);
        assert_eq!(offset_of!(Body, floor), 0x980);
        assert_eq!(offset_of!(Body, tile_group), 0x984);
        assert_eq!(offset_of!(Body, tile_script), 0x988);
        assert_eq!(offset_of!(Body, unknown_098ch), 0x98c);
        assert_eq!(offset_of!(Body, light_color_r), 0x9c8);
        assert_eq!(offset_of!(Body, light_color_g), 0x9cc);
        assert_eq!(offset_of!(Body, light_color_b), 0x9d0);
        assert_eq!(offset_of!(Body, unknown_09d4h), 0x9d4);
        assert_eq!(offset_of!(Body, light_fade), 0x9d8);
        assert_eq!(offset_of!(Body, ambient), 0x9dc);
        assert_eq!(offset_of!(Body, tg_price), 0x9e0);
        assert_eq!(offset_of!(Body, unknown_09e4h), 0x9e4);
        assert_eq!(offset_of!(Body, aircraft_logic), 0x9e8);
        assert_eq!(offset_of!(Body, ed_rotate), 0x9f0);
        assert_eq!(offset_of!(Body, unknown_09f4h), 0x9f4);
        assert_eq!(offset_of!(Body, help_index), 0x9fc);
        assert_eq!(offset_of!(Body, focusable), 0xa00);
        assert_eq!(offset_of!(Body, dragable), 0xa01);
        assert_eq!(offset_of!(Body, slideable), 0xa02);
        assert_eq!(offset_of!(Body, wheelable), 0xa03);
        assert_eq!(offset_of!(Body, selectable), 0xa04);
        assert_eq!(offset_of!(Body, deletable), 0xa05);
        assert_eq!(offset_of!(Body, unknown_0a06h), 0xa06);
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
            (offset_of!(Body, unknown_0140h), size_of::<[u8; 0x50]>()),
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
            (offset_of!(Body, unknown_01f4h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, bullet), size_of::<u8>()),
            (offset_of!(Body, unknown_01f9h), size_of::<[u8; 0xb]>()),
            (offset_of!(Body, is_physic_update), size_of::<u8>()),
            (offset_of!(Body, unknown_0205h), size_of::<[u8; 0x7]>()),
            (offset_of!(Body, center_x), size_of::<f32>()),
            (offset_of!(Body, center_y), size_of::<f32>()),
            (offset_of!(Body, unknown_0214h), size_of::<[u8; 0x30]>()),
            (offset_of!(Body, refuel_counter), size_of::<f32>()),
            (offset_of!(Body, aimer_creature_id), size_of::<u64>()),
            (offset_of!(Body, survivors_penalty), size_of::<u8>()),
            (offset_of!(Body, is_right_direction), size_of::<u8>()),
            (offset_of!(Body, unknown_0252h), size_of::<[u8; 0x16]>()),
            (
                offset_of!(Body, ship_convertion_error),
                size_of::<EscadraString>(),
            ),
            (offset_of!(Body, unknown_0288h), size_of::<[u8; 0x20]>()),
            (offset_of!(Body, collisions), size_of::<i32>()),
            (offset_of!(Body, unknown_02ach), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, ttl), size_of::<f32>()),
            (offset_of!(Body, ttl_desolvetime), size_of::<f32>()),
            (offset_of!(Body, health), size_of::<f32>()),
            (offset_of!(Body, health_max), size_of::<f32>()),
            (offset_of!(Body, health_lock), size_of::<u8>()),
            (offset_of!(Body, unknown_02c1h), size_of::<[u8; 0x3]>()),
            (offset_of!(Body, contamination), size_of::<f32>()),
            (offset_of!(Body, unknown_02c8h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, explosive_power), size_of::<f32>()),
            (offset_of!(Body, armor_piercing), size_of::<f32>()),
            (offset_of!(Body, explosive_type), size_of::<i32>()),
            (offset_of!(Body, explosive_temperature), size_of::<f32>()),
            (offset_of!(Body, unknown_02dch), size_of::<[u8; 0x48]>()),
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
            (offset_of!(Body, unknown_0378h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, collision_sensor), size_of::<f32>()),
            (offset_of!(Body, unknown_0380h), size_of::<[u8; 0x34]>()),
            (offset_of!(Body, logic), size_of::<i32>()),
            (offset_of!(Body, logic_active), size_of::<u8>()),
            (offset_of!(Body, unknown_03b9h), size_of::<[u8; 0x3]>()),
            (offset_of!(Body, logic_state), size_of::<u32>()),
            (offset_of!(Body, logic_f1), size_of::<f32>()),
            (offset_of!(Body, logic_f2), size_of::<f32>()),
            (offset_of!(Body, logic_f3), size_of::<f32>()),
            (offset_of!(Body, logic_f4), size_of::<f32>()),
            (offset_of!(Body, unknown_03d0h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, logic_i1), size_of::<i32>()),
            (offset_of!(Body, logic_i2), size_of::<i32>()),
            (offset_of!(Body, logic_i3), size_of::<i32>()),
            (offset_of!(Body, logic_i4), size_of::<i32>()),
            (offset_of!(Body, unknown_03e4h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, logic_s1), size_of::<EscadraString>()),
            (offset_of!(Body, unknown_0408h), size_of::<[u8; 0xc0]>()),
            (offset_of!(Body, logic_l1_saved), size_of::<Link>()),
            (offset_of!(Body, logic_editor), size_of::<CVec<i32>>()),
            (offset_of!(Body, sectors), size_of::<CVec<f32>>()),
            (offset_of!(Body, unknown_0510h), size_of::<[u8; 0x288]>()),
            (offset_of!(Body, count), size_of::<i32>()),
            (offset_of!(Body, is_loot), size_of::<u8>()),
            (offset_of!(Body, is_blueprint), size_of::<u8>()),
            (offset_of!(Body, unknown_079eh), size_of::<[u8; 0x1]>()),
            (offset_of!(Body, is_wheel), size_of::<u8>()),
            (offset_of!(Body, is_leg), size_of::<u8>()),
            (offset_of!(Body, unknown_07a1h), size_of::<[u8; 0x47]>()),
            (offset_of!(Body, mdl_fuel), size_of::<f64>()),
            (offset_of!(Body, mdl_fss), size_of::<f32>()),
            (offset_of!(Body, mdl_crew), size_of::<i32>()),
            (offset_of!(Body, mdl_side), size_of::<i32>()),
            (offset_of!(Body, unknown_07fch), size_of::<[u8; 0x2c]>()),
            (offset_of!(Body, mdl_pump), size_of::<f32>()),
            (offset_of!(Body, mdl_turbine), size_of::<f32>()),
            (offset_of!(Body, unknown_0830h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, mdl_covered), size_of::<f32>()),
            (offset_of!(Body, weapon_type), size_of::<i32>()),
            (offset_of!(Body, weapon_ammo), size_of::<i32>()),
            (offset_of!(Body, weapon_ammo_extra), size_of::<i32>()),
            (offset_of!(Body, weapon_ammo_amount), size_of::<i32>()),
            (offset_of!(Body, weapon_ammo_extra_amount), size_of::<i32>()),
            (offset_of!(Body, unknown_084ch), size_of::<[u8; 0x16]>()),
            (offset_of!(Body, rescue_explosive), size_of::<u8>()),
            (offset_of!(Body, rescue_flameable), size_of::<u8>()),
            (offset_of!(Body, rescue_radioactive), size_of::<u8>()),
            (offset_of!(Body, rescue_toxic), size_of::<u8>()),
            (offset_of!(Body, rescue_scare), size_of::<u8>()),
            (offset_of!(Body, unknown_0867h), size_of::<[u8; 0x9]>()),
            (offset_of!(Body, is_crt_screen), size_of::<u8>()),
            (offset_of!(Body, unknown_0871h), size_of::<[u8; 0x3]>()),
            (offset_of!(Body, repair_type), size_of::<i32>()),
            (offset_of!(Body, repair_pos), size_of::<i32>()),
            (offset_of!(Body, craft_activated), size_of::<u8>()),
            (offset_of!(Body, unknown_087dh), size_of::<[u8; 0x3]>()),
            (offset_of!(Body, mask), size_of::<*mut ISerializable>()),
            (
                offset_of!(Body, sprites),
                size_of::<CVec<*mut ISerializable>>(),
            ),
            (offset_of!(Body, stage), size_of::<i32>()),
            (offset_of!(Body, unknown_08a4h), size_of::<[u8; 0x10]>()),
            (offset_of!(Body, mesh_render), size_of::<u8>()),
            (offset_of!(Body, unknown_08b5h), size_of::<[u8; 0x3]>()),
            (offset_of!(Body, mesh_color), size_of::<u32>()),
            (offset_of!(Body, mask_mesh), size_of::<u8>()),
            (offset_of!(Body, unknown_08bdh), size_of::<[u8; 0x3]>()),
            (offset_of!(Body, opacity), size_of::<f32>()),
            (offset_of!(Body, unknown_08c4h), size_of::<[u8; 0x10]>()),
            (offset_of!(Body, z), size_of::<f32>()),
            (offset_of!(Body, unknown_08d8h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, scalemode_max), size_of::<i32>()),
            (offset_of!(Body, scalemode_min), size_of::<i32>()),
            (offset_of!(Body, gui), size_of::<u8>()),
            (offset_of!(Body, scalable), size_of::<u8>()),
            (offset_of!(Body, unknown_08e6h), size_of::<[u8; 0x1]>()),
            (offset_of!(Body, issuperlay), size_of::<u8>()),
            (offset_of!(Body, scale_x), size_of::<f32>()),
            (offset_of!(Body, scale_y), size_of::<f32>()),
            (offset_of!(Body, notaproximate), size_of::<u8>()),
            (offset_of!(Body, renderlines), size_of::<u8>()),
            (offset_of!(Body, unknown_08f2h), size_of::<[u8; 0x2]>()),
            (offset_of!(Body, light_pow), size_of::<f32>()),
            (offset_of!(Body, light_shift_x), size_of::<f32>()),
            (offset_of!(Body, light_shift_y), size_of::<f32>()),
            (offset_of!(Body, light_dir_x), size_of::<f32>()),
            (offset_of!(Body, light_dir_y), size_of::<f32>()),
            (offset_of!(Body, light_directional), size_of::<f32>()),
            (offset_of!(Body, shadow), size_of::<f32>()),
            (offset_of!(Body, unknown_0910h), size_of::<[u8; 0x68]>()),
            (offset_of!(Body, floor_type), size_of::<u64>()),
            (offset_of!(Body, floor), size_of::<i32>()),
            (offset_of!(Body, tile_group), size_of::<i32>()),
            (offset_of!(Body, tile_script), size_of::<i32>()),
            (offset_of!(Body, unknown_098ch), size_of::<[u8; 0x3c]>()),
            (offset_of!(Body, light_color_r), size_of::<f32>()),
            (offset_of!(Body, light_color_g), size_of::<f32>()),
            (offset_of!(Body, light_color_b), size_of::<f32>()),
            (offset_of!(Body, unknown_09d4h), size_of::<[u8; 0x4]>()),
            (offset_of!(Body, light_fade), size_of::<f32>()),
            (offset_of!(Body, ambient), size_of::<f32>()),
            (offset_of!(Body, tg_price), size_of::<f32>()),
            (offset_of!(Body, unknown_09e4h), size_of::<[u8; 0x4]>()),
            (
                offset_of!(Body, aircraft_logic),
                size_of::<*mut ISerializable>(),
            ),
            (offset_of!(Body, ed_rotate), size_of::<f32>()),
            (offset_of!(Body, unknown_09f4h), size_of::<[u8; 0x8]>()),
            (offset_of!(Body, help_index), size_of::<i32>()),
            (offset_of!(Body, focusable), size_of::<u8>()),
            (offset_of!(Body, dragable), size_of::<u8>()),
            (offset_of!(Body, slideable), size_of::<u8>()),
            (offset_of!(Body, wheelable), size_of::<u8>()),
            (offset_of!(Body, selectable), size_of::<u8>()),
            (offset_of!(Body, deletable), size_of::<u8>()),
            (offset_of!(Body, unknown_0a06h), size_of::<[u8; 0x2]>()),
        ];
        let mut end = 0;
        for &(offset, width) in fields {
            assert_eq!(offset, end);
            end = offset + width;
        }
        assert_eq!(end, size_of::<Body>());
    }
}
