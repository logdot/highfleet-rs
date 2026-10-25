//! Native Profile live view for Steam 1.163, recovered from registered serialization properties.

use crate::general::{CVec, EscadraString, ISerializable, SentinelList};

/// Native Profile layout with the observed complete allocation size of `0x5E8`.
///
/// The constructor at `0x140289530` initializes the non-virtual ISerializable base at offset zero and registers properties through `0x140297A60`.
/// Its vtable uses shared `ISerializable::Serialize` / `Deserialize`, which dispatch typed codecs over this property list.
/// Fields follow the registered keys and backing addresses, not the stale saved Profile composite datatype.
/// Properties bound to application globals are not members of this layout.
/// Native keys do not establish enum variants or richer application semantics.
/// Flags are raw bytes, not Rust `bool`s.
///
/// This is a live view only.
/// Opaque regions include owning native state that Rust does not model.
/// Do not construct Profile values in Rust or use Rust drop as a substitute for native destruction.
/// Rust raw pointer elements do not model native ownership of their targets.
/// Borrow only a live object through a raw pointer or a short-lived reference.
#[repr(C)]
pub struct Profile {
    /// Non-virtual ISerializable base at offset zero.
    pub base: ISerializable,
    /// Uninterpreted native state at `0x048..0x0A8`.
    pub unknown_0048h: [u8; 0x60],
    /// Serialized as `nPlayers`; native signed integer at `0x0A8`.
    pub n_players: i32,
    /// Uninterpreted native state at `0x0AC..0x0B0`.
    pub unknown_00ach: [u8; 0x4],
    /// Serialized as `m_id` through the unsigned 64-bit codec at `0x0B0`.
    pub id: u64,
    /// Uninterpreted native state at `0x0B8..0x0D0`.
    pub unknown_00b8h: [u8; 0x18],
    /// Serialized as `m_name`; native storage at `0x0D0`.
    pub name: EscadraString,
    /// Serialized as `m_default`; raw flag byte at `0x0F0`.
    pub default: u8,
    /// Uninterpreted native state at `0x0F1..0x0F8`.
    pub unknown_00f1h: [u8; 0x7],
    /// Serialized as `m_level_file`; native storage at `0x0F8`.
    pub level_file: EscadraString,
    /// Serialized as `m_progress`; native storage at `0x118`.
    pub progress: f32,
    /// Uninterpreted native state at `0x11C..0x120`.
    pub unknown_011ch: [u8; 0x4],
    /// Serialized as `m_savename`; native storage at `0x120`.
    pub savename: EscadraString,
    /// Serialized as `m_savetime`; native storage at `0x140`.
    pub savetime: EscadraString,
    /// Serialized as `m_daytime`; native storage at `0x160`, distinct from `APP::DAY_TIME`.
    pub daytime: f32,
    /// Serialized as `m_easymode`; raw flag byte at `0x164`.
    pub easymode: u8,
    /// Serialized as `m_hardmode`; raw flag byte at `0x165`.
    pub hardmode: u8,
    /// Uninterpreted native state at `0x166..0x168`.
    pub unknown_0166h: [u8; 0x2],
    /// Serialized as `m_codename`; native storage at `0x168`.
    pub codename: EscadraString,
    /// Serialized as `m_countdown`; native storage at `0x188`.
    pub countdown: f32,
    /// Serialized as `m_days`; native storage at `0x18C`.
    pub days: f32,
    /// Serialized as `m_days_from_event`; native storage at `0x190`.
    pub days_from_event: f32,
    /// Uninterpreted native state at `0x194..0x198`.
    pub unknown_0194h: [u8; 0x4],
    /// Serialized as `m_schedule`; the codec appends 0x20-byte native strings at `0x198`.
    pub schedule: CVec<EscadraString>,
    /// Serialized as `m_objectives`; the same string-vector codec binds `0x1B0`.
    pub objectives: CVec<EscadraString>,
    /// Serialized as `m_enemy_value`; native storage at `0x1C8`.
    pub enemy_value: f32,
    /// Serialized as `m_scores`; native storage at `0x1CC`.
    pub scores: i32,
    /// Serialized as `m_scores_expedition`; native storage at `0x1D0`.
    pub scores_expedition: i32,
    /// Serialized as `m_scores_military`; native storage at `0x1D4`.
    pub scores_military: i32,
    /// Serialized as `m_scores_survivors`; native storage at `0x1D8`.
    pub scores_survivors: i32,
    /// Uninterpreted native state at `0x1DC..0x1F8`.
    pub unknown_01dch: [u8; 0x1c],
    /// Serialized as `m_escadras`; dynamic object-pointer vector at `0x1F8`.
    pub escadras: CVec<*mut ISerializable>,
    /// Serialized as `m_locations`; dynamic object-pointer vector at `0x210`.
    pub locations: CVec<*mut ISerializable>,
    /// Serialized as `m_missiles`; dynamic object-pointer vector at `0x228`.
    pub missiles: CVec<*mut ISerializable>,
    /// Serialized as `m_explosions`; dynamic object-pointer vector at `0x240`.
    pub explosions: CVec<*mut ISerializable>,
    /// Serialized as `m_designs`; dynamic object-pointer vector at `0x258`.
    pub designs: CVec<*mut ISerializable>,
    /// Serialized as `m_mapCrafts`; dynamic object-pointer vector at `0x270`.
    pub map_crafts: CVec<*mut ISerializable>,
    /// Uninterpreted native state at `0x288..0x2A0`.
    pub unknown_0288h: [u8; 0x18],
    /// Serialized as `m_mapmarks`; dynamically serialized object pointer at `0x2A0`.
    pub mapmarks: *mut ISerializable,
    /// Serialized as `m_cash`; native cash balance at `0x2A8`.
    pub cash: i32,
    /// Uninterpreted native state at `0x2AC..0x2B8`.
    pub unknown_02ach: [u8; 0xc],
    /// Serialized as `m_npc_index`; native storage at `0x2B8`.
    pub npc_index: i32,
    /// Uninterpreted native state at `0x2BC..0x2C0`.
    pub unknown_02bch: [u8; 0x4],
    /// Serialized as `m_npcs`; non-owning view of a native object-pointer list at `0x2C0`.
    pub npcs: SentinelList<*mut ISerializable>,
    /// Serialized as `m_items`; non-owning view of a native object-pointer list at `0x2D0`.
    pub items: SentinelList<*mut ISerializable>,
    /// Uninterpreted native state at `0x2E0..0x2E4`.
    pub unknown_02e0h: [u8; 0x4],
    /// Serialized as `m_reinforcement_state`; native storage at `0x2E4`.
    pub reinforcement_state: i32,
    /// Serialized as `m_endrace`; raw flag byte at `0x2E8`.
    pub endrace: u8,
    /// Serialized as `m_endwar`; raw flag byte at `0x2E9`.
    pub endwar: u8,
    /// Serialized as `m_endstrike`; raw flag byte at `0x2EA`.
    pub endstrike: u8,
    /// Uninterpreted native state at `0x2EB..0x2EC`.
    pub unknown_02ebh: [u8; 0x1],
    /// Serialized as `m_escalation`; native storage at `0x2EC`.
    pub escalation: i32,
    /// Serialized as `m_flagship_lost`; raw flag byte at `0x2F0`.
    pub flagship_lost: u8,
    /// Serialized as `m_burn_fuel`; raw flag byte at `0x2F1`.
    pub burn_fuel: u8,
    /// Uninterpreted native state at `0x2F2..0x2F4`.
    pub unknown_02f2h: [u8; 0x2],
    /// Serialized as `m_rams`; native storage at `0x2F4`.
    pub rams: i32,
    /// Serialized as `m_alert`; native storage at `0x2F8`.
    pub alert: f32,
    /// Serialized as `m_alert_hr`; native storage at `0x2FC`.
    pub alert_hr: f32,
    /// Uninterpreted native state at `0x300..0x304`.
    pub unknown_0300h: [u8; 0x4],
    /// Serialized as `m_radio_duration`; native storage at `0x304`.
    pub radio_duration: f32,
    /// Serialized as `m_radio_duration_base`; native storage at `0x308`.
    pub radio_duration_base: f32,
    /// Uninterpreted native state at `0x30C..0x310`.
    pub unknown_030ch: [u8; 0x4],
    /// Serialized individually as `m_enigma_set1` through `m_enigma_set4` at `0x310..0x320`.
    pub enigma_set: [i32; 4],
    /// Serialized individually as `m_enigma_key1` through `m_enigma_key12` at `0x320..0x350`.
    pub enigma_key: [i32; 12],
    /// Serialized individually as `m_enigma_got1` through `m_enigma_got12` at `0x350..0x35C`.
    /// Raw flag bytes, not Rust `bool`s.
    pub enigma_got: [u8; 12],
    /// Serialized as `m_enigma_timer`; native storage at `0x35C`.
    pub enigma_timer: i32,
    /// Serialized as `m_tads_record`; native storage at `0x360`.
    pub tads_record: f32,
    /// Serialized as `m_intel_data`; native storage at `0x364`.
    pub intel_data: i32,
    /// Serialized as `m_ant`; raw flag byte at `0x368`.
    pub ant: u8,
    /// Uninterpreted native state at `0x369..0x36C`.
    pub unknown_0369h: [u8; 0x3],
    /// Serialized as `m_char_fear`; native storage at `0x36C`.
    pub char_fear: i32,
    /// Serialized as `m_char_gerat`; native storage at `0x370`.
    pub char_gerat: i32,
    /// Serialized as `m_char_romani`; native storage at `0x374`.
    pub char_romani: i32,
    /// Serialized as `m_char_faith`; native storage at `0x378`.
    pub char_faith: i32,
    /// Serialized as `m_char_right`; native storage at `0x37C`.
    pub char_right: i32,
    /// Serialized as `m_char_force`; native storage at `0x380`.
    pub char_force: i32,
    /// Serialized as `m_char_wealth`; native storage at `0x384`.
    pub char_wealth: i32,
    /// Serialized as `m_char_kindness`; native storage at `0x388`.
    pub char_kindness: i32,
    /// Serialized as `m_char_fear_val`; native storage at `0x38C`.
    pub char_fear_val: f32,
    /// Serialized as `m_char_gerat_val`; native storage at `0x390`.
    pub char_gerat_val: f32,
    /// Serialized as `m_char_romani_val`; native storage at `0x394`.
    pub char_romani_val: f32,
    /// Serialized as `m_char_faith_val`; native storage at `0x398`.
    pub char_faith_val: f32,
    /// Serialized as `m_char_right_val`; native storage at `0x39C`.
    pub char_right_val: f32,
    /// Serialized as `m_char_force_val`; native storage at `0x3A0`.
    pub char_force_val: f32,
    /// Serialized as `m_char_wealth_val`; native storage at `0x3A4`.
    pub char_wealth_val: f32,
    /// Serialized as `m_char_kindness_val`; native storage at `0x3A8`.
    pub char_kindness_val: f32,
    /// Uninterpreted native state at `0x3AC..0x3B0`.
    pub unknown_03ach: [u8; 0x4],
    /// Serialized as `nextCreatureId`; native signed 64-bit storage at `0x3B0`.
    pub next_creature_id: i64,
    /// Serialized as `nextHistoryDesign`; native signed 32-bit storage at `0x3B8`.
    pub next_history_design: i32,
    /// Uninterpreted native state at `0x3BC..0x3C0`.
    pub unknown_03bch: [u8; 0x4],
    /// Serialized as `vecLastPlayerPositionsKnown`; dynamic object-pointer vector at `0x3C0`.
    pub last_player_positions_known: CVec<*mut ISerializable>,
    /// Uninterpreted native state at `0x3D8..0x3F0`.
    pub unknown_03d8h: [u8; 0x18],
    /// Opaque region at `0x3F0..0x430` containing the custom `m_trigger` container.
    /// Its codec serializes string/float entries; the container's internal layout is not modeled.
    pub trigger: [u8; 0x40],
    /// Opaque region at `0x430..0x470` containing the custom `m_unlocks` container.
    /// Its codec serializes string/float entries; the container's internal layout is not modeled.
    pub unlocks: [u8; 0x40],
    /// Opaque region at `0x470..0x4B0` containing the custom `m_strigger` container.
    /// Its codec serializes string/string entries; the container's internal layout is not modeled.
    pub strigger: [u8; 0x40],
    /// Opaque region at `0x4B0..0x4F0` containing the custom `m_events` container.
    /// Its codec serializes string/float entries; the container's internal layout is not modeled.
    pub events: [u8; 0x40],
    /// Serialized as `m_arguments`; signed integer vector at `0x4F0`.
    pub arguments: CVec<i32>,
    /// Serialized as `m_demo_state`; native storage at `0x508`.
    pub demo_state: i32,
    /// Uninterpreted native state at `0x50C..0x510`.
    pub unknown_050ch: [u8; 0x4],
    /// Serialized as `m_demo_loot`; native string at `0x510`.
    pub demo_loot: EscadraString,
    /// Uninterpreted native state at `0x530..0x578`.
    pub unknown_0530h: [u8; 0x48],
    /// Serialized as `mapSkyOffset.x`; native storage at `0x578`.
    pub map_sky_offset_x: f32,
    /// Serialized as `mapSkyOffset.y`; native storage at `0x57C`.
    pub map_sky_offset_y: f32,
    /// Serialized as `mapSkySteps`; native storage at `0x580`.
    pub map_sky_steps: i32,
    /// Serialized as `mapSkySeed`; native unsigned integer at `0x584`.
    pub map_sky_seed: u32,
    /// Uninterpreted native state at `0x588..0x5A0`.
    pub unknown_0588h: [u8; 0x18],
    /// Serialized as `gameVersion`; native float at `0x5A0`.
    pub game_version: f32,
    /// Uninterpreted native state at `0x5A4..0x5C0`.
    pub unknown_05a4h: [u8; 0x1c],
    /// Opaque native vector header at `0x5C0..0x5D8`, serialized as `vecFreeCaptains`.
    /// Its codec walks inline 0x50-byte objects, not object pointers; element layout is not modeled.
    pub free_captains: [u8; 0x18],
    /// Serialized as `skirmishLevel`; native storage at `0x5D8`.
    pub skirmish_level: i32,
    /// Serialized as `nextSkirmishEscadraLevel`; native storage at `0x5DC`.
    pub next_skirmish_escadra_level: i32,
    /// Serialized as `totalCaptains`; native storage at `0x5E0`.
    pub total_captains: i32,
    /// Uninterpreted native state at `0x5E4..0x5E8`.
    pub unknown_05e4h: [u8; 0x4],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, needs_drop, offset_of, size_of};

    // Expected native offsets and widths, including opaque regions.
    const FIELDS: &[(usize, usize, usize)] = &[
        (offset_of!(Profile, base), 0, size_of::<ISerializable>()),
        (
            offset_of!(Profile, unknown_0048h),
            0x48,
            size_of::<[u8; 0x60]>(),
        ),
        (offset_of!(Profile, n_players), 0xa8, size_of::<i32>()),
        (
            offset_of!(Profile, unknown_00ach),
            0xac,
            size_of::<[u8; 0x4]>(),
        ),
        (offset_of!(Profile, id), 0xb0, size_of::<u64>()),
        (
            offset_of!(Profile, unknown_00b8h),
            0xb8,
            size_of::<[u8; 0x18]>(),
        ),
        (offset_of!(Profile, name), 0xd0, size_of::<EscadraString>()),
        (offset_of!(Profile, default), 0xf0, size_of::<u8>()),
        (
            offset_of!(Profile, unknown_00f1h),
            0xf1,
            size_of::<[u8; 0x7]>(),
        ),
        (
            offset_of!(Profile, level_file),
            0xf8,
            size_of::<EscadraString>(),
        ),
        (offset_of!(Profile, progress), 0x118, size_of::<f32>()),
        (
            offset_of!(Profile, unknown_011ch),
            0x11c,
            size_of::<[u8; 0x4]>(),
        ),
        (
            offset_of!(Profile, savename),
            0x120,
            size_of::<EscadraString>(),
        ),
        (
            offset_of!(Profile, savetime),
            0x140,
            size_of::<EscadraString>(),
        ),
        (offset_of!(Profile, daytime), 0x160, size_of::<f32>()),
        (offset_of!(Profile, easymode), 0x164, size_of::<u8>()),
        (offset_of!(Profile, hardmode), 0x165, size_of::<u8>()),
        (
            offset_of!(Profile, unknown_0166h),
            0x166,
            size_of::<[u8; 0x2]>(),
        ),
        (
            offset_of!(Profile, codename),
            0x168,
            size_of::<EscadraString>(),
        ),
        (offset_of!(Profile, countdown), 0x188, size_of::<f32>()),
        (offset_of!(Profile, days), 0x18c, size_of::<f32>()),
        (
            offset_of!(Profile, days_from_event),
            0x190,
            size_of::<f32>(),
        ),
        (
            offset_of!(Profile, unknown_0194h),
            0x194,
            size_of::<[u8; 0x4]>(),
        ),
        (
            offset_of!(Profile, schedule),
            0x198,
            size_of::<CVec<EscadraString>>(),
        ),
        (
            offset_of!(Profile, objectives),
            0x1b0,
            size_of::<CVec<EscadraString>>(),
        ),
        (offset_of!(Profile, enemy_value), 0x1c8, size_of::<f32>()),
        (offset_of!(Profile, scores), 0x1cc, size_of::<i32>()),
        (
            offset_of!(Profile, scores_expedition),
            0x1d0,
            size_of::<i32>(),
        ),
        (
            offset_of!(Profile, scores_military),
            0x1d4,
            size_of::<i32>(),
        ),
        (
            offset_of!(Profile, scores_survivors),
            0x1d8,
            size_of::<i32>(),
        ),
        (
            offset_of!(Profile, unknown_01dch),
            0x1dc,
            size_of::<[u8; 0x1c]>(),
        ),
        (
            offset_of!(Profile, escadras),
            0x1f8,
            size_of::<CVec<*mut ISerializable>>(),
        ),
        (
            offset_of!(Profile, locations),
            0x210,
            size_of::<CVec<*mut ISerializable>>(),
        ),
        (
            offset_of!(Profile, missiles),
            0x228,
            size_of::<CVec<*mut ISerializable>>(),
        ),
        (
            offset_of!(Profile, explosions),
            0x240,
            size_of::<CVec<*mut ISerializable>>(),
        ),
        (
            offset_of!(Profile, designs),
            0x258,
            size_of::<CVec<*mut ISerializable>>(),
        ),
        (
            offset_of!(Profile, map_crafts),
            0x270,
            size_of::<CVec<*mut ISerializable>>(),
        ),
        (
            offset_of!(Profile, unknown_0288h),
            0x288,
            size_of::<[u8; 0x18]>(),
        ),
        (
            offset_of!(Profile, mapmarks),
            0x2a0,
            size_of::<*mut ISerializable>(),
        ),
        (offset_of!(Profile, cash), 0x2a8, size_of::<i32>()),
        (
            offset_of!(Profile, unknown_02ach),
            0x2ac,
            size_of::<[u8; 0xc]>(),
        ),
        (offset_of!(Profile, npc_index), 0x2b8, size_of::<i32>()),
        (
            offset_of!(Profile, unknown_02bch),
            0x2bc,
            size_of::<[u8; 0x4]>(),
        ),
        (
            offset_of!(Profile, npcs),
            0x2c0,
            size_of::<SentinelList<*mut ISerializable>>(),
        ),
        (
            offset_of!(Profile, items),
            0x2d0,
            size_of::<SentinelList<*mut ISerializable>>(),
        ),
        (
            offset_of!(Profile, unknown_02e0h),
            0x2e0,
            size_of::<[u8; 0x4]>(),
        ),
        (
            offset_of!(Profile, reinforcement_state),
            0x2e4,
            size_of::<i32>(),
        ),
        (offset_of!(Profile, endrace), 0x2e8, size_of::<u8>()),
        (offset_of!(Profile, endwar), 0x2e9, size_of::<u8>()),
        (offset_of!(Profile, endstrike), 0x2ea, size_of::<u8>()),
        (
            offset_of!(Profile, unknown_02ebh),
            0x2eb,
            size_of::<[u8; 0x1]>(),
        ),
        (offset_of!(Profile, escalation), 0x2ec, size_of::<i32>()),
        (offset_of!(Profile, flagship_lost), 0x2f0, size_of::<u8>()),
        (offset_of!(Profile, burn_fuel), 0x2f1, size_of::<u8>()),
        (
            offset_of!(Profile, unknown_02f2h),
            0x2f2,
            size_of::<[u8; 0x2]>(),
        ),
        (offset_of!(Profile, rams), 0x2f4, size_of::<i32>()),
        (offset_of!(Profile, alert), 0x2f8, size_of::<f32>()),
        (offset_of!(Profile, alert_hr), 0x2fc, size_of::<f32>()),
        (
            offset_of!(Profile, unknown_0300h),
            0x300,
            size_of::<[u8; 0x4]>(),
        ),
        (offset_of!(Profile, radio_duration), 0x304, size_of::<f32>()),
        (
            offset_of!(Profile, radio_duration_base),
            0x308,
            size_of::<f32>(),
        ),
        (
            offset_of!(Profile, unknown_030ch),
            0x30c,
            size_of::<[u8; 0x4]>(),
        ),
        (
            offset_of!(Profile, enigma_set),
            0x310,
            size_of::<[i32; 4]>(),
        ),
        (
            offset_of!(Profile, enigma_key),
            0x320,
            size_of::<[i32; 12]>(),
        ),
        (
            offset_of!(Profile, enigma_got),
            0x350,
            size_of::<[u8; 12]>(),
        ),
        (offset_of!(Profile, enigma_timer), 0x35c, size_of::<i32>()),
        (offset_of!(Profile, tads_record), 0x360, size_of::<f32>()),
        (offset_of!(Profile, intel_data), 0x364, size_of::<i32>()),
        (offset_of!(Profile, ant), 0x368, size_of::<u8>()),
        (
            offset_of!(Profile, unknown_0369h),
            0x369,
            size_of::<[u8; 0x3]>(),
        ),
        (offset_of!(Profile, char_fear), 0x36c, size_of::<i32>()),
        (offset_of!(Profile, char_gerat), 0x370, size_of::<i32>()),
        (offset_of!(Profile, char_romani), 0x374, size_of::<i32>()),
        (offset_of!(Profile, char_faith), 0x378, size_of::<i32>()),
        (offset_of!(Profile, char_right), 0x37c, size_of::<i32>()),
        (offset_of!(Profile, char_force), 0x380, size_of::<i32>()),
        (offset_of!(Profile, char_wealth), 0x384, size_of::<i32>()),
        (offset_of!(Profile, char_kindness), 0x388, size_of::<i32>()),
        (offset_of!(Profile, char_fear_val), 0x38c, size_of::<f32>()),
        (offset_of!(Profile, char_gerat_val), 0x390, size_of::<f32>()),
        (
            offset_of!(Profile, char_romani_val),
            0x394,
            size_of::<f32>(),
        ),
        (offset_of!(Profile, char_faith_val), 0x398, size_of::<f32>()),
        (offset_of!(Profile, char_right_val), 0x39c, size_of::<f32>()),
        (offset_of!(Profile, char_force_val), 0x3a0, size_of::<f32>()),
        (
            offset_of!(Profile, char_wealth_val),
            0x3a4,
            size_of::<f32>(),
        ),
        (
            offset_of!(Profile, char_kindness_val),
            0x3a8,
            size_of::<f32>(),
        ),
        (
            offset_of!(Profile, unknown_03ach),
            0x3ac,
            size_of::<[u8; 0x4]>(),
        ),
        (
            offset_of!(Profile, next_creature_id),
            0x3b0,
            size_of::<i64>(),
        ),
        (
            offset_of!(Profile, next_history_design),
            0x3b8,
            size_of::<i32>(),
        ),
        (
            offset_of!(Profile, unknown_03bch),
            0x3bc,
            size_of::<[u8; 0x4]>(),
        ),
        (
            offset_of!(Profile, last_player_positions_known),
            0x3c0,
            size_of::<CVec<*mut ISerializable>>(),
        ),
        (
            offset_of!(Profile, unknown_03d8h),
            0x3d8,
            size_of::<[u8; 0x18]>(),
        ),
        (offset_of!(Profile, trigger), 0x3f0, size_of::<[u8; 0x40]>()),
        (offset_of!(Profile, unlocks), 0x430, size_of::<[u8; 0x40]>()),
        (
            offset_of!(Profile, strigger),
            0x470,
            size_of::<[u8; 0x40]>(),
        ),
        (offset_of!(Profile, events), 0x4b0, size_of::<[u8; 0x40]>()),
        (
            offset_of!(Profile, arguments),
            0x4f0,
            size_of::<CVec<i32>>(),
        ),
        (offset_of!(Profile, demo_state), 0x508, size_of::<i32>()),
        (
            offset_of!(Profile, unknown_050ch),
            0x50c,
            size_of::<[u8; 0x4]>(),
        ),
        (
            offset_of!(Profile, demo_loot),
            0x510,
            size_of::<EscadraString>(),
        ),
        (
            offset_of!(Profile, unknown_0530h),
            0x530,
            size_of::<[u8; 0x48]>(),
        ),
        (
            offset_of!(Profile, map_sky_offset_x),
            0x578,
            size_of::<f32>(),
        ),
        (
            offset_of!(Profile, map_sky_offset_y),
            0x57c,
            size_of::<f32>(),
        ),
        (offset_of!(Profile, map_sky_steps), 0x580, size_of::<i32>()),
        (offset_of!(Profile, map_sky_seed), 0x584, size_of::<u32>()),
        (
            offset_of!(Profile, unknown_0588h),
            0x588,
            size_of::<[u8; 0x18]>(),
        ),
        (offset_of!(Profile, game_version), 0x5a0, size_of::<f32>()),
        (
            offset_of!(Profile, unknown_05a4h),
            0x5a4,
            size_of::<[u8; 0x1c]>(),
        ),
        (
            offset_of!(Profile, free_captains),
            0x5c0,
            size_of::<[u8; 0x18]>(),
        ),
        (offset_of!(Profile, skirmish_level), 0x5d8, size_of::<i32>()),
        (
            offset_of!(Profile, next_skirmish_escadra_level),
            0x5dc,
            size_of::<i32>(),
        ),
        (offset_of!(Profile, total_captains), 0x5e0, size_of::<i32>()),
        (
            offset_of!(Profile, unknown_05e4h),
            0x5e4,
            size_of::<[u8; 0x4]>(),
        ),
    ];

    #[test]
    fn native_layout() {
        assert_eq!(size_of::<Profile>(), 0x5e8);
        assert_eq!(align_of::<Profile>(), 8);
        assert!(needs_drop::<Profile>());
        for &(offset, expected, _) in FIELDS {
            assert_eq!(offset, expected);
        }
    }

    #[test]
    fn explicit_fields_cover_every_byte_without_implicit_padding() {
        let mut end = 0;
        for &(offset, _, width) in FIELDS {
            assert_eq!(offset, end);
            end = offset + width;
        }
        assert_eq!(end, size_of::<Profile>());
    }
}
