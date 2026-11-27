//! Native mesh vertex storage for Steam 1.163.

use crate::general::ISerializable;

/// `Mesh` live view with the allocation size `0x58`.
///
/// The factory at `0x1402024E0` allocates this class (`0x40000003`).
/// `0x14027E9E0` constructs it.
/// The serializer at `0x14027EFE0` reads one pair of floats per vertex, with an eight-byte stride.
///
/// Borrow live meshes through raw pointers.
/// The vertex buffer belongs to the game and may be null.
/// Do not adopt its storage or drop the game-owned object.
#[repr(C)]
pub struct Mesh {
    /// Non-virtual `ISerializable` base.
    pub base: ISerializable,
    /// Pointer to the native array of consecutive x/y vertex pairs.
    pub vertices: *mut [f32; 2],
    /// Signed vertex count, serialized as `m_size`.
    pub vertex_count: i32,
    /// Uninterpreted bytes at `0x54..0x58`.
    pub unknown_0054h: [u8; 4],
}

const _: () = {
    use core::mem::{align_of, offset_of, size_of};

    assert!(size_of::<Mesh>() == 0x58);
    assert!(align_of::<Mesh>() == 8);
    assert!(offset_of!(Mesh, base) == 0);
    assert!(offset_of!(Mesh, vertices) == 0x48);
    assert!(offset_of!(Mesh, vertex_count) == 0x50);
    assert!(offset_of!(Mesh, unknown_0054h) == 0x54);
};
