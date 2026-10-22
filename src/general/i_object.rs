//! Primary IObject base layout shared by Steam 1.151 and 1.163.

use super::EscadraString;
use core::ffi::c_void;

/// Primary `IObject` base view, occupying `0x30` bytes in the inspected hierarchy.
///
/// Contains the hierarchy's only primary vtable pointer.
/// Rust-constructed values own their initialized string field.
/// This does not implement native construction or virtual dispatch; borrow instances owned by the game.
#[repr(C)]
pub struct IObject {
    /// Native primary vtable pointer, shared by all embedded bases.
    pub vtable: *const c_void,
    /// Serialized as `m_classname`; owned when constructed in Rust.
    /// String access methods require UTF-8, which is not guaranteed for arbitrary native bytes.
    pub class_name: EscadraString,
    /// Serialized as `m_code`; enum/code values are not yet modeled.
    pub code: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, needs_drop, offset_of, size_of};

    #[test]
    fn primary_base_prefix_layout() {
        assert_eq!(size_of::<IObject>(), 0x30);
        assert_eq!(align_of::<IObject>(), 8);
        assert_eq!(offset_of!(IObject, vtable), 0);
        assert_eq!(offset_of!(IObject, class_name), 8);
        assert_eq!(offset_of!(IObject, code), 0x28);
        assert!(needs_drop::<IObject>());
    }

    #[test]
    fn constructed_object_owns_its_string_but_a_borrow_does_not_move_it() {
        let text = "A heap-allocated native class name".to_owned();
        let mut object = IObject {
            vtable: core::ptr::null(),
            class_name: EscadraString::new(),
            code: 0,
        };
        object.class_name.set_string(&text);
        let native_pointer = core::ptr::from_mut(&mut object);
        {
            let borrowed = unsafe { &*native_pointer };
            assert_eq!(borrowed.class_name.get_string(), text);
        }
        assert_eq!(object.class_name.get_string(), text);
        // Ordinary Rust destruction drops the owned string after the borrow ends.
        drop(object);
    }
}
