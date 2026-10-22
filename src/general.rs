//! Defines types that are very unlikely to change between game versions.
//!
//! # Shared native object layouts
//!
//! The object layouts in this module are shared by the inspected Steam 1.151 and 1.163 versions.
//! Rust-constructed values own their string fields.
//! Borrow game-owned instances through raw pointers or references; do not copy them into Rust ownership with `ptr::read` or destroy them with `drop_in_place`.
//! Rust field destruction is not the game's virtual destructor.
//! Raw pointer fields do not imply pointee ownership.
//!
//! RTTI establishes the primary, non-virtual chain Body -> [`Node`] -> [`ISerializable`] -> [`IObject`], with each base at offset zero.
//! Embedded `base` members model that chain; Rust traits and trait-object vtables do not model this C++ ABI.
//! The base spans describe prefixes within the inspected derived objects, not independently recovered standalone C++ allocation sizes.

pub mod escadra_string;
pub use escadra_string::*;

pub mod tll;
pub use tll::*;

pub mod cvec;
pub use cvec::*;

pub mod i_object;
pub use i_object::*;

pub mod i_serializable;
pub use i_serializable::*;

pub mod link;
pub use link::*;

pub mod node;
pub use node::*;

pub mod sentinel_list;
pub use sentinel_list::*;
