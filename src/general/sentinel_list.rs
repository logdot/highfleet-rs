//! Native circular sentinel-list layouts shared by Steam 1.151 and 1.163.

/// Non-owning circular sentinel-list header, not Rust's `LinkedList` layout.
#[repr(C)]
pub struct SentinelList<T> {
    /// Heap sentinel node; links form a circular list when initialized by the game.
    pub head: *mut SentinelListNode<T>,
    /// Number of elements, excluding the sentinel.
    pub length: u64,
}

/// Native list node.
/// The inspected lists carry pointer-sized values.
#[repr(C)]
pub struct SentinelListNode<T> {
    /// Next node in the circular list.
    pub next: *mut SentinelListNode<T>,
    /// Previous node in the circular list.
    pub previous: *mut SentinelListNode<T>,
    /// Stored element; a sentinel's value must not be assumed initialized.
    pub value: T,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::general::Node;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn native_header_and_node_layouts() {
        type List = SentinelList<*mut Node>;
        type ListNode = SentinelListNode<*mut Node>;
        assert_eq!(size_of::<List>(), 0x10);
        assert_eq!(align_of::<List>(), 8);
        assert_eq!(offset_of!(List, head), 0);
        assert_eq!(offset_of!(List, length), 8);
        assert_eq!(size_of::<ListNode>(), 0x18);
        assert_eq!(offset_of!(ListNode, next), 0);
        assert_eq!(offset_of!(ListNode, previous), 8);
        assert_eq!(offset_of!(ListNode, value), 0x10);
    }
}
