//! ISerializable base-prefix layout shared by Steam 1.151 and 1.163.

use super::{IObject, SentinelList};
use core::ffi::c_void;

/// `ISerializable` prefix view, spanning `0x48` bytes within Node and Body.
#[repr(C)]
pub struct ISerializable {
    /// Non-virtual `IObject` base at offset zero.
    pub base: IObject,
    /// Property-list header; concrete property pointee types are not modeled.
    pub properties: SentinelList<*mut c_void>,
    /// Native serialization-control byte; not a Rust `bool` validity constraint.
    pub serialization_enabled: u8,
    /// Uninterpreted state at offsets `0x41..0x48`, not disposable padding.
    pub unknown_0041h: [u8; 7],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, needs_drop, offset_of, size_of};

    #[test]
    fn serializable_base_prefix_layout() {
        assert_eq!(size_of::<ISerializable>(), 0x48);
        assert_eq!(align_of::<ISerializable>(), 8);
        assert_eq!(offset_of!(ISerializable, base), 0);
        assert_eq!(offset_of!(ISerializable, properties), 0x30);
        assert_eq!(offset_of!(ISerializable, serialization_enabled), 0x40);
        assert_eq!(offset_of!(ISerializable, unknown_0041h), 0x41);
        assert!(needs_drop::<ISerializable>());
    }
}
