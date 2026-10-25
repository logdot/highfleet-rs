//! Native Gui live view for Steam 1.163.

use super::Body;
use crate::general::{CVec, EscadraString, ISerializable, Link};

/// Native Gui layout with the observed complete allocation size of `0xE70`.
///
/// The non-virtual Body base occupies offset zero.
/// Serialized fields follow `initializeGuiProperties` (`0x14015FA60` in 1.163):
/// each property binds its key to native storage through a typed property codec.
/// GUI's vtable uses shared `ISerializable::Serialize` (`0x140190820`) and
/// `Deserialize` (`0x14018FA40`), which traverse these registered properties.
/// Names identify serializer keys, not inferred enums or application-level meanings.
/// Native flags are raw bytes rather than Rust `bool`s.
///
/// This is a live view only.
/// Opaque regions include owning native state that Rust does not model.
/// Do not construct Gui values in Rust or use Rust drop as a substitute for native destruction.
/// In particular, never move or drop a game-owned object through this view.
/// Borrow it through a raw pointer or a short-lived reference.
#[repr(C)]
pub struct Gui {
    /// Non-virtual Body base at offset zero.
    pub base: Body,
    /// Serialized as `m_guid`; click-dispatch identifier at `0xA40`.
    pub identifier: EscadraString,
    /// Serialized as `m_value_str`; native storage at `0xA60`.
    pub value_str: EscadraString,
    /// Serialized as `m_value`; native storage at `0xA80`.
    pub value: f32,
    /// Serialized as `m_min`; native storage at `0xA84`.
    pub min: f32,
    /// Serialized as `m_max`; native storage at `0xA88`.
    pub max: f32,
    /// Serialized through the signed integer codec as `m_fontcolor`; packed color bits at `0xA8C`.
    pub text_color: u32,
    /// Serialized as `m_border_width`; native storage at `0xA90`.
    pub border_width: f32,
    /// Serialized as `m_border_width_text`; native storage at `0xA94`.
    pub border_width_text: f32,
    /// Uninterpreted native state at `0xA98..0xAB8`.
    pub unknown_0a98h: [u8; 0x20],
    /// Serialized as `m_backname`; native storage at `0xAB8`.
    pub backname: EscadraString,
    /// Serialized as `m_backname_focus`; native storage at `0xAD8`.
    pub backname_focus: EscadraString,
    /// Serialized as both `m_backname_use` and `m_backname_checked` at `0xAF8`.
    /// Both property registrations bind this same address, not the string at `0xB18`.
    pub backname_use: EscadraString,
    /// Uninterpreted native state at `0xB18..0xB38`.
    pub unknown_0b18h: [u8; 0x20],
    /// Serialized as `m_forename`; native storage at `0xB38`.
    pub forename: EscadraString,
    /// Serialized as `m_bagetname`; native storage at `0xB58`.
    pub bagetname: EscadraString,
    /// Uninterpreted native state at `0xB78..0xBB0`.
    pub unknown_0b78h: [u8; 0x38],
    /// Serialized as `m_action`; native storage at `0xBB0`.
    pub action: i32,
    /// Uninterpreted native state at `0xBB4..0xBB8`.
    pub unknown_0bb4h: [u8; 0x4],
    /// Native Link at `0xBB8`; `m_gui_l1` serializes its `saved_id` at `0xBC8`.
    pub gui_l1: Link,
    /// Native Link at `0xBD0`; `m_gui_l2` serializes its `saved_id` at `0xBE0`.
    pub gui_l2: Link,
    /// Native Link at `0xBE8`; `m_gui_l3` serializes its `saved_id` at `0xBF8`.
    pub gui_l3: Link,
    /// Serialized as `m_gui_i1`; native storage at `0xC00`.
    pub gui_i1: i32,
    /// Serialized as `m_gui_i2`; native storage at `0xC04`.
    pub gui_i2: i32,
    /// Serialized as `m_gui_i3`; native storage at `0xC08`.
    pub gui_i3: i32,
    /// Uninterpreted native state at `0xC0C..0xC10`.
    pub unknown_0c0ch: [u8; 0x4],
    /// Serialized as `m_gui_s1`; ammo identifier used by native shop routing at `0xC10`.
    pub gui_s1: EscadraString,
    /// Serialized as `m_gui_s2`; native storage at `0xC30`.
    pub gui_s2: EscadraString,
    /// Serialized as `m_gui_s3`; native storage at `0xC50`.
    pub gui_s3: EscadraString,
    /// Serialized as `m_gui_timestamp`; native storage at `0xC70`.
    pub gui_timestamp: f64,
    /// Uninterpreted native state at `0xC78..0xC80`.
    pub unknown_0c78h: [u8; 0x8],
    /// Serialized as `m_checklist`; vector of dynamically serialized object pointers at `0xC80`.
    /// The codec establishes the pointer element type, not a concrete derived class.
    /// Rust raw pointer elements do not model native ownership of the pointed-to objects.
    pub checklist: CVec<*mut ISerializable>,
    /// Uninterpreted native state at `0xC98..0xC9C`.
    pub unknown_0c98h: [u8; 0x4],
    /// Serialized as `m_texted_max`; native storage at `0xC9C`.
    pub texted_max: i32,
    /// Serialized as `m_clickable`; raw flag byte at `0xCA0`.
    pub clickable: u8,
    /// Serialized as `m_checkable`; raw flag byte at `0xCA1`.
    pub checkable: u8,
    /// Serialized as `m_textable`; raw flag byte at `0xCA2`.
    pub textable: u8,
    /// Serialized as `m_dropable`; raw flag byte at `0xCA3`.
    pub dropable: u8,
    /// Uninterpreted native state at `0xCA4..0xCA8`.
    pub unknown_0ca4h: [u8; 0x4],
    /// Serialized as `m_text`; native control text at `0xCA8`.
    pub text: EscadraString,
    /// Uninterpreted native state at `0xCC8..0xCE8`.
    pub unknown_0cc8h: [u8; 0x20],
    /// Serialized as `m_note`; native storage at `0xCE8`.
    pub note: EscadraString,
    /// Serialized as `m_str`; native storage at `0xD08`.
    pub str: EscadraString,
    /// Uninterpreted native state at `0xD28..0xD78`.
    pub unknown_0d28h: [u8; 0x50],
    /// Serialized as `m_text_counter`; native storage at `0xD78`.
    pub text_counter: i32,
    /// Uninterpreted native state at `0xD7C..0xD88`.
    pub unknown_0d7ch: [u8; 0xc],
    /// Serialized as `m_fontname`; native font identifier at `0xD88`.
    pub font: EscadraString,
    /// Serialized as `m_fontscale`; native storage at `0xDA8`.
    pub font_scale: f32,
    /// Serialized as `m_fontblend`; native storage at `0xDAC`.
    pub font_blend: i32,
    /// Uninterpreted native state at `0xDB0..0xDB4`.
    pub unknown_0db0h: [u8; 0x4],
    /// Serialized as `m_align`; native integer at `0xDB4`, not an inferred enum.
    pub align: i32,
    /// Serialized as `m_centered`; native integer (not a flag byte) at `0xDB8`.
    pub centered: i32,
    /// Serialized as `m_text_shift.x`; native storage at `0xDBC`.
    pub text_shift_x: f32,
    /// Serialized as `m_text_shift.y`; native storage at `0xDC0`.
    pub text_shift_y: f32,
    /// Uninterpreted native state at `0xDC4..0xE04`.
    pub unknown_0dc4h: [u8; 0x40],
    /// Serialized as `rotateText`; raw flag byte at `0xE04`.
    pub rotate_text: u8,
    /// Uninterpreted native state at `0xE05..0xE08`.
    pub unknown_0e05h: [u8; 0x3],
    /// Serialized as `m_alarm`; native storage at `0xE08`.
    pub alarm: f32,
    /// Serialized as `m_alarm_pos.x`; native storage at `0xE0C`.
    pub alarm_pos_x: f32,
    /// Serialized as `m_alarm_pos.y`; native storage at `0xE10`.
    pub alarm_pos_y: f32,
    /// Uninterpreted native state at `0xE14..0xE38`.
    pub unknown_0e14h: [u8; 0x24],
    /// Serialized as `m_alarm_backsound`; raw flag byte at `0xE38`.
    pub alarm_backsound: u8,
    /// Uninterpreted native state at `0xE39..0xE3C`.
    pub unknown_0e39h: [u8; 0x3],
    /// Serialized as `m_alarm_volume`; native storage at `0xE3C`.
    pub alarm_volume: f32,
    /// Serialized as `m_alarm_speed`; native storage at `0xE40`.
    pub alarm_speed: f32,
    /// Uninterpreted native state at `0xE44..0xE50`.
    pub unknown_0e44h: [u8; 0xc],
    /// Serialized as `m_alarm_color` through the unsigned integer codec at `0xE50`.
    pub alarm_color: u32,
    /// Serialized as `m_alarm_scale`; native storage at `0xE54`.
    pub alarm_scale: f32,
    /// Serialized individually as `radiogramData.codes[0]` through `[3]` at `0xE58..0xE68`.
    pub radiogram_codes: [i32; 4],
    /// Serialized as `radiogramData.language`; native integer at `0xE68`.
    pub radiogram_language: i32,
    /// Uninterpreted native state at `0xE6C..0xE70`.
    pub unknown_0e6ch: [u8; 0x4],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, needs_drop, offset_of, size_of};

    // Expected native offsets and widths, including all opaque regions.
    const FIELDS: &[(usize, usize, usize)] = &[
        (offset_of!(Gui, base), 0, size_of::<Body>()),
        (
            offset_of!(Gui, identifier),
            0xa40,
            size_of::<EscadraString>(),
        ),
        (
            offset_of!(Gui, value_str),
            0xa60,
            size_of::<EscadraString>(),
        ),
        (offset_of!(Gui, value), 0xa80, size_of::<f32>()),
        (offset_of!(Gui, min), 0xa84, size_of::<f32>()),
        (offset_of!(Gui, max), 0xa88, size_of::<f32>()),
        (offset_of!(Gui, text_color), 0xa8c, size_of::<u32>()),
        (offset_of!(Gui, border_width), 0xa90, size_of::<f32>()),
        (offset_of!(Gui, border_width_text), 0xa94, size_of::<f32>()),
        (
            offset_of!(Gui, unknown_0a98h),
            0xa98,
            size_of::<[u8; 0x20]>(),
        ),
        (offset_of!(Gui, backname), 0xab8, size_of::<EscadraString>()),
        (
            offset_of!(Gui, backname_focus),
            0xad8,
            size_of::<EscadraString>(),
        ),
        (
            offset_of!(Gui, backname_use),
            0xaf8,
            size_of::<EscadraString>(),
        ),
        (
            offset_of!(Gui, unknown_0b18h),
            0xb18,
            size_of::<[u8; 0x20]>(),
        ),
        (offset_of!(Gui, forename), 0xb38, size_of::<EscadraString>()),
        (
            offset_of!(Gui, bagetname),
            0xb58,
            size_of::<EscadraString>(),
        ),
        (
            offset_of!(Gui, unknown_0b78h),
            0xb78,
            size_of::<[u8; 0x38]>(),
        ),
        (offset_of!(Gui, action), 0xbb0, size_of::<i32>()),
        (
            offset_of!(Gui, unknown_0bb4h),
            0xbb4,
            size_of::<[u8; 0x4]>(),
        ),
        (offset_of!(Gui, gui_l1), 0xbb8, size_of::<Link>()),
        (offset_of!(Gui, gui_l2), 0xbd0, size_of::<Link>()),
        (offset_of!(Gui, gui_l3), 0xbe8, size_of::<Link>()),
        (offset_of!(Gui, gui_i1), 0xc00, size_of::<i32>()),
        (offset_of!(Gui, gui_i2), 0xc04, size_of::<i32>()),
        (offset_of!(Gui, gui_i3), 0xc08, size_of::<i32>()),
        (
            offset_of!(Gui, unknown_0c0ch),
            0xc0c,
            size_of::<[u8; 0x4]>(),
        ),
        (offset_of!(Gui, gui_s1), 0xc10, size_of::<EscadraString>()),
        (offset_of!(Gui, gui_s2), 0xc30, size_of::<EscadraString>()),
        (offset_of!(Gui, gui_s3), 0xc50, size_of::<EscadraString>()),
        (offset_of!(Gui, gui_timestamp), 0xc70, size_of::<f64>()),
        (
            offset_of!(Gui, unknown_0c78h),
            0xc78,
            size_of::<[u8; 0x8]>(),
        ),
        (
            offset_of!(Gui, checklist),
            0xc80,
            size_of::<CVec<*mut ISerializable>>(),
        ),
        (
            offset_of!(Gui, unknown_0c98h),
            0xc98,
            size_of::<[u8; 0x4]>(),
        ),
        (offset_of!(Gui, texted_max), 0xc9c, size_of::<i32>()),
        (offset_of!(Gui, clickable), 0xca0, size_of::<u8>()),
        (offset_of!(Gui, checkable), 0xca1, size_of::<u8>()),
        (offset_of!(Gui, textable), 0xca2, size_of::<u8>()),
        (offset_of!(Gui, dropable), 0xca3, size_of::<u8>()),
        (
            offset_of!(Gui, unknown_0ca4h),
            0xca4,
            size_of::<[u8; 0x4]>(),
        ),
        (offset_of!(Gui, text), 0xca8, size_of::<EscadraString>()),
        (
            offset_of!(Gui, unknown_0cc8h),
            0xcc8,
            size_of::<[u8; 0x20]>(),
        ),
        (offset_of!(Gui, note), 0xce8, size_of::<EscadraString>()),
        (offset_of!(Gui, str), 0xd08, size_of::<EscadraString>()),
        (
            offset_of!(Gui, unknown_0d28h),
            0xd28,
            size_of::<[u8; 0x50]>(),
        ),
        (offset_of!(Gui, text_counter), 0xd78, size_of::<i32>()),
        (
            offset_of!(Gui, unknown_0d7ch),
            0xd7c,
            size_of::<[u8; 0xc]>(),
        ),
        (offset_of!(Gui, font), 0xd88, size_of::<EscadraString>()),
        (offset_of!(Gui, font_scale), 0xda8, size_of::<f32>()),
        (offset_of!(Gui, font_blend), 0xdac, size_of::<i32>()),
        (
            offset_of!(Gui, unknown_0db0h),
            0xdb0,
            size_of::<[u8; 0x4]>(),
        ),
        (offset_of!(Gui, align), 0xdb4, size_of::<i32>()),
        (offset_of!(Gui, centered), 0xdb8, size_of::<i32>()),
        (offset_of!(Gui, text_shift_x), 0xdbc, size_of::<f32>()),
        (offset_of!(Gui, text_shift_y), 0xdc0, size_of::<f32>()),
        (
            offset_of!(Gui, unknown_0dc4h),
            0xdc4,
            size_of::<[u8; 0x40]>(),
        ),
        (offset_of!(Gui, rotate_text), 0xe04, size_of::<u8>()),
        (
            offset_of!(Gui, unknown_0e05h),
            0xe05,
            size_of::<[u8; 0x3]>(),
        ),
        (offset_of!(Gui, alarm), 0xe08, size_of::<f32>()),
        (offset_of!(Gui, alarm_pos_x), 0xe0c, size_of::<f32>()),
        (offset_of!(Gui, alarm_pos_y), 0xe10, size_of::<f32>()),
        (
            offset_of!(Gui, unknown_0e14h),
            0xe14,
            size_of::<[u8; 0x24]>(),
        ),
        (offset_of!(Gui, alarm_backsound), 0xe38, size_of::<u8>()),
        (
            offset_of!(Gui, unknown_0e39h),
            0xe39,
            size_of::<[u8; 0x3]>(),
        ),
        (offset_of!(Gui, alarm_volume), 0xe3c, size_of::<f32>()),
        (offset_of!(Gui, alarm_speed), 0xe40, size_of::<f32>()),
        (
            offset_of!(Gui, unknown_0e44h),
            0xe44,
            size_of::<[u8; 0xc]>(),
        ),
        (offset_of!(Gui, alarm_color), 0xe50, size_of::<u32>()),
        (offset_of!(Gui, alarm_scale), 0xe54, size_of::<f32>()),
        (
            offset_of!(Gui, radiogram_codes),
            0xe58,
            size_of::<[i32; 4]>(),
        ),
        (offset_of!(Gui, radiogram_language), 0xe68, size_of::<i32>()),
        (
            offset_of!(Gui, unknown_0e6ch),
            0xe6c,
            size_of::<[u8; 0x4]>(),
        ),
    ];

    #[test]
    fn native_layout() {
        assert_eq!(size_of::<Gui>(), 0xe70);
        assert_eq!(align_of::<Gui>(), 8);
        assert!(needs_drop::<Gui>());
        for &(offset, expected, _) in FIELDS {
            assert_eq!(offset, expected);
        }
        assert_eq!(offset_of!(Gui, gui_l1) + offset_of!(Link, saved_id), 0xbc8);
        assert_eq!(offset_of!(Gui, gui_l2) + offset_of!(Link, saved_id), 0xbe0);
        assert_eq!(offset_of!(Gui, gui_l3) + offset_of!(Link, saved_id), 0xbf8);
    }

    #[test]
    fn explicit_fields_cover_every_byte_without_implicit_padding() {
        let mut end = 0;
        for &(offset, _, width) in FIELDS {
            assert_eq!(offset, end);
            end = offset + width;
        }
        assert_eq!(end, size_of::<Gui>());
    }
}
