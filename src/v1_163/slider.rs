//! Native GUI slider for Steam 1.163.

use super::Gui;
use crate::general::EscadraString;

/// `Slider` live view with the allocation size `0xEA8`.
///
/// The factory at `0x1402024E0` allocates the class (`0x14000F`).
/// `0x14012B4A0` constructs its `Gui` base.
/// `0x140165CB0` registers the slider's own serialized properties.
///
/// Borrow live sliders through raw pointers.
/// Do not construct, move or drop a game-owned slider through this view.
#[repr(C)]
pub struct Slider {
    /// Non-virtual `Gui` base.
    pub base: Gui,
    /// Serialized as `m_render_value`; raw flag byte.
    pub render_value: u8,
    /// Serialized as `m_is_float`; raw flag byte.
    pub is_float: u8,
    /// Uninterpreted bytes at `0xE72..0xE74`.
    pub unknown_0e72h: [u8; 2],
    /// Serialized as `m_overmax`.
    pub overmax: f32,
    /// Serialized as `m_backname_bar`.
    pub backname_bar: EscadraString,
    /// Unmodeled runtime state at `0xE98..0xEA0`.
    pub unknown_0e98h: [u8; 8],
    /// Serialized as `m_slider_index`.
    pub slider_index: i32,
    /// Serialized as `m_slider_l`.
    pub slider_l: f32,
}

const _: () = {
    use core::mem::{align_of, offset_of, size_of};

    assert!(size_of::<Slider>() == 0xEA8);
    assert!(align_of::<Slider>() == 8);
    assert!(offset_of!(Slider, base) == 0);
    assert!(offset_of!(Slider, render_value) == 0xE70);
    assert!(offset_of!(Slider, is_float) == 0xE71);
    assert!(offset_of!(Slider, unknown_0e72h) == 0xE72);
    assert!(offset_of!(Slider, overmax) == 0xE74);
    assert!(offset_of!(Slider, backname_bar) == 0xE78);
    assert!(offset_of!(Slider, unknown_0e98h) == 0xE98);
    assert!(offset_of!(Slider, slider_index) == 0xEA0);
    assert!(offset_of!(Slider, slider_l) == 0xEA4);
};
