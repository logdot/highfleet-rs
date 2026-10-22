//! Native Node prefix layout shared by Steam 1.151 and 1.163.

use super::{EscadraString, ISerializable, Link, SentinelList};

/// `Node` prefix view, spanning `0x128` bytes within the inspected Body objects.
///
/// Unknown regions include runtime-only state; they are not free mod storage.
/// Rust-owned values drop initialized string fields, but do not perform native link registration or virtual destruction.
/// Pointer-based containers retain their existing cleanup behavior; this type does not recursively destroy their targets.
#[repr(C)]
pub struct Node {
    /// Non-virtual `ISerializable` base at offset zero.
    pub base: ISerializable,
    /// Serialized as `m_state`.
    pub state: i32,
    /// Uninterpreted state at offsets `0x4C..0x50`.
    pub unknown_004ch: [u8; 4],
    /// Native registry of Links tracking this Node.
    pub registered_links: SentinelList<*mut Link>,
    /// Uninterpreted state at offsets `0x60..0x78`.
    pub unknown_0060h: [u8; 0x18],
    /// Serialized as `m_id`.
    pub id: i64,
    /// Master link; its identifier serializes as `m_master_id`.
    pub master: Link,
    /// Native Link whose application-level role has not been established.
    pub unnamed_link: Link,
    /// Owner link; its identifier serializes as `m_owner_id`.
    pub owner: Link,
    /// Serialized as `m_name`.
    pub name: EscadraString,
    /// Serialized as `nodeType`; enum values are not yet modeled.
    pub node_type: i32,
    /// Uninterpreted state at offsets `0xEC..0xF0`.
    pub unknown_00ech: [u8; 4],
    /// Serialized as `m_desowning_time`.
    pub desowning_time: f32,
    /// Uninterpreted state at offsets `0xF4..0x100`.
    pub unknown_00f4h: [u8; 0xc],
    /// Serialized as `m_children`; elements are Node pointers.
    pub children: SentinelList<*mut Node>,
    /// Serialized as `m_joints`; concrete serializable classes remain unknown.
    pub joints: SentinelList<*mut ISerializable>,
    /// Uninterpreted state at offsets `0x120..0x128`.
    pub unknown_0120h: [u8; 8],
}

#[cfg(test)]
mod tests {
    use super::Node;
    use core::mem::{align_of, needs_drop, offset_of, size_of};

    #[test]
    fn node_prefix_layout() {
        assert_eq!(size_of::<Node>(), 0x128);
        assert_eq!(align_of::<Node>(), 8);
        assert_eq!(offset_of!(Node, base), 0);
        assert_eq!(offset_of!(Node, state), 0x48);
        assert_eq!(offset_of!(Node, unknown_004ch), 0x4c);
        assert_eq!(offset_of!(Node, registered_links), 0x50);
        assert_eq!(offset_of!(Node, unknown_0060h), 0x60);
        assert_eq!(offset_of!(Node, id), 0x78);
        assert_eq!(offset_of!(Node, master), 0x80);
        assert_eq!(offset_of!(Node, unnamed_link), 0x98);
        assert_eq!(offset_of!(Node, owner), 0xb0);
        assert_eq!(offset_of!(Node, name), 0xc8);
        assert_eq!(offset_of!(Node, node_type), 0xe8);
        assert_eq!(offset_of!(Node, unknown_00ech), 0xec);
        assert_eq!(offset_of!(Node, desowning_time), 0xf0);
        assert_eq!(offset_of!(Node, unknown_00f4h), 0xf4);
        assert_eq!(offset_of!(Node, children), 0x100);
        assert_eq!(offset_of!(Node, joints), 0x110);
        assert_eq!(offset_of!(Node, unknown_0120h), 0x120);
        assert!(needs_drop::<Node>());
    }
}
