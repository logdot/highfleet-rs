//! Native Gui live view for Steam 1.163.

use super::Body;
use crate::general::EscadraString;

/// Native Gui layout with the observed complete allocation size of `0xE70`.
///
/// The non-virtual Body base occupies offset zero.
/// Field offsets are established by GUI property initialization.
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
    /// Native `m_guid` identifier used by click dispatch, at `0xA40`.
    pub identifier: EscadraString,
    /// Uninterpreted native state at `0xA60..0xA8C`.
    pub unknown_0a60h: [u8; 0x2c],
    /// Native text color at `0xA8C`.
    pub text_color: u32,
    /// Uninterpreted native state at `0xA90..0xC10`.
    pub unknown_0a90h: [u8; 0x180],
    /// Ammo identifier used by native shop routing, at `0xC10`.
    pub item_key: EscadraString,
    /// Uninterpreted native state at `0xC30..0xCA8`.
    pub unknown_0c30h: [u8; 0x78],
    /// Native control text at `0xCA8`.
    pub text: EscadraString,
    /// Uninterpreted native state at `0xCC8..0xD88`.
    pub unknown_0cc8h: [u8; 0xc0],
    /// Native font identifier at `0xD88`.
    pub font: EscadraString,
    /// Native font scale at `0xDA8`.
    pub font_scale: f32,
    /// Uninterpreted native state at `0xDAC..0xE70`.
    pub unknown_0dach: [u8; 0xc4],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, needs_drop, offset_of, size_of};

    #[test]
    fn native_layout() {
        assert_eq!(size_of::<Gui>(), 0xe70);
        assert_eq!(align_of::<Gui>(), 8);
        assert!(needs_drop::<Gui>());
        assert_eq!(offset_of!(Gui, base), 0);
        assert_eq!(offset_of!(Gui, identifier), 0xa40);
        assert_eq!(offset_of!(Gui, unknown_0a60h), 0xa60);
        assert_eq!(offset_of!(Gui, text_color), 0xa8c);
        assert_eq!(offset_of!(Gui, unknown_0a90h), 0xa90);
        assert_eq!(offset_of!(Gui, item_key), 0xc10);
        assert_eq!(offset_of!(Gui, unknown_0c30h), 0xc30);
        assert_eq!(offset_of!(Gui, text), 0xca8);
        assert_eq!(offset_of!(Gui, unknown_0cc8h), 0xcc8);
        assert_eq!(offset_of!(Gui, font), 0xd88);
        assert_eq!(offset_of!(Gui, font_scale), 0xda8);
        assert_eq!(offset_of!(Gui, unknown_0dach), 0xdac);
    }

    #[test]
    fn explicit_fields_cover_every_byte_without_implicit_padding() {
        let fields: &[(usize, usize)] = &[
            (offset_of!(Gui, base), size_of::<Body>()),
            (offset_of!(Gui, identifier), size_of::<EscadraString>()),
            (offset_of!(Gui, unknown_0a60h), size_of::<[u8; 0x2c]>()),
            (offset_of!(Gui, text_color), size_of::<u32>()),
            (offset_of!(Gui, unknown_0a90h), size_of::<[u8; 0x180]>()),
            (offset_of!(Gui, item_key), size_of::<EscadraString>()),
            (offset_of!(Gui, unknown_0c30h), size_of::<[u8; 0x78]>()),
            (offset_of!(Gui, text), size_of::<EscadraString>()),
            (offset_of!(Gui, unknown_0cc8h), size_of::<[u8; 0xc0]>()),
            (offset_of!(Gui, font), size_of::<EscadraString>()),
            (offset_of!(Gui, font_scale), size_of::<f32>()),
            (offset_of!(Gui, unknown_0dach), size_of::<[u8; 0xc4]>()),
        ];
        let mut end = 0;
        for &(offset, width) in fields {
            assert_eq!(offset, end);
            end = offset + width;
        }
        assert_eq!(end, size_of::<Gui>());
    }
}
