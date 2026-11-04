//! Native aircraft definition payloads for Steam 1.163.
//!
//! These structures are table values, not creatures or bodies.
//! `PLANE_TLL` at `0x143A13C50` maps `CRAFT_*` names to [`Plane`] values, and
//! `PLANE_LOADOUTS` at `0x143A15A60` maps `LOADOUT_*` names to [`Loadout`] values.
//! Both tables are filled by `LoadPlaneLoadouts` at `0x1401AF830` before the main loop.
//!
//! [`Plane::loadouts`] holds non-owning pointers into `PLANE_LOADOUTS`, and
//! [`Loadout::parts`] is an inline native vector inside a table value.
//! These are borrowed live views only: do not construct them in Rust, copy a
//! game-owned value with `ptr::read`, or run Rust drop on native table storage.
//! A matching layout does not establish allocator compatibility or ownership transfer.
//! Raw pointer elements do not own their targets.
//! Access requires live, aligned table entries of this game version. Prefer raw field
//! pointers; do not form references whose lifetime or exclusivity the game cannot uphold.

use std::mem::{align_of, offset_of, size_of};

use crate::general::{CVec, EscadraString};

/// Value stored by `PLANE_TLL`, keyed by a `CRAFT_*` name.
///
/// The 1.163 value is `0x20` bytes: a 32-bit cached loadout-weight sum, four
/// uninterpreted bytes, and a vector of loadout pointers.
/// `AddPlane` at `0x140196660` allocates the containing `0x60`-byte map node and
/// initializes the vector at node offsets `0x48`..`0x58`.
/// The cache is rebuilt from the loadouts by `TryLaunchAiAircraftGroup` at `0x1400EF910`
/// when it is zero; a nonzero cache is used directly.
/// The game can therefore write the cache during gameplay; do not hold shared references
/// across those writes or assume it is automatically invalidated after table edits.
///
/// This is a live view of game-owned table storage.
/// Rust drop is not the native table destructor, and vector elements do not own their targets.
#[repr(C)]
pub struct Plane {
    /// Cached sum of the loadouts' launch weights at `0x00`.
    ///
    /// `TryLaunchAiAircraftGroup` reads this word with `MOV R8D, dword ptr [RSI+0x40]` at
    /// `0x1400EFB03` and treats zero as "recompute". Its loop loads each
    /// `dword ptr [loadout+0x58]` and adds it back in the sequence
    /// `0x1400EFB23`..`0x1400EFB2F`.
    /// Native 32-bit storage; the observed accesses do not establish signedness.
    pub cached_launch_weight_sum: u32,
    /// Uninterpreted bytes at `0x04..0x08`.
    ///
    /// `AddPlane` at `0x140196660` copies only the dword at payload `+0x00` and moves the
    /// vector; initialization of these four bytes is not established.
    /// Do not treat them as disposable padding.
    pub unknown_0004h: [u8; 4],
    /// Non-owning pointers to `Loadout` values stored in `PLANE_LOADOUTS` at `0x08`.
    ///
    /// Each element points at a value inside a map node; the vector does not own them.
    pub loadouts: CVec<*const Loadout>,
}

/// Value stored by `PLANE_LOADOUTS`, keyed by a `LOADOUT_*` name.
///
/// The 1.163 value is `0x60` bytes. The map node also stores a duplicate key before
/// this value, so the loadout value's own `oid` starts at value offset zero.
/// `AddToLoadouts` at `0x140196440` allocates the containing `0xA0`-byte map node and
/// initializes the strings at node `0x40`/`0x60`, the parts vector at `0x80`, the
/// weight dword at `0x98`, and the flag byte at `0x9C`.
///
/// This is a live view of game-owned table storage.
/// Do not construct, copy, or drop a game-owned loadout through this type.
#[repr(C)]
pub struct Loadout {
    /// Object identifier string at `0x00`; duplicates the map key.
    pub oid: EscadraString,
    /// Icon or sprite name at `0x20`, for example `LOADOUT_GUN37`.
    pub icon: EscadraString,
    /// Inline part entries at `0x40`, with a native element stride of `0x28`.
    ///
    /// The vector storage and its string elements belong to the native table.
    pub parts: CVec<Munition>,
    /// Weight used by the launch-group selection at `0x58`.
    ///
    /// `TryLaunchAiAircraftGroup` reads it with `MOV R8D, dword ptr [RAX+0x58]` and
    /// subtracts weights while choosing a loadout. This is a raw 32-bit word; the native
    /// declaration's signedness is not independently established.
    /// The constructor at `0x1401A4710` accepts a float but converts it with
    /// `CVTTSS2SI` at `0x1401A4794` before storing the integer at `0x58`.
    pub launch_loadout_weight: u32,
    /// Raw `has_gun37mm` flag byte at `0x5C`, not a Rust `bool`.
    pub has_gun37mm: u8,
    /// Uninterpreted native state at `0x5D..0x60`; not established as disposable padding.
    pub unknown_005dh: [u8; 3],
}

/// One inline part entry in [`Loadout::parts`], with a native stride of `0x28`.
///
/// This is borrowed table storage: do not move it out of the native vector or drop it.
#[repr(C)]
pub struct Munition {
    /// Item name string at `0x00`, for example `ITEM_FAB100`.
    pub name: EscadraString,
    /// Part count at `0x20`.
    ///
    /// Native 32-bit storage used with signed arithmetic. `LoadAmmoForAircraft` at
    /// `0x14002E930` copies the count and passes it to `Aircraft_AddAmmoToList120` at
    /// `0x140007AA0`, which tests it with `TEST R14D,R14D` / `JLE` at
    /// `0x140007AEC`..`0x140007AEF` and bounds its append loop with
    /// `CMP EBP,R14D` / `JL` at `0x140007B44`..`0x140007B47`.
    /// This establishes signed use, not the original C++ declaration.
    pub count: i32,
    /// Uninterpreted native state at `0x24..0x28`; not established as disposable padding.
    pub unknown_0024h: [u8; 4],
}

// Compile-time layout contract for the Steam 1.163 definition tables.
const _: () = {
    assert!(size_of::<Plane>() == 0x20);
    assert!(align_of::<Plane>() == 8);
    assert!(offset_of!(Plane, cached_launch_weight_sum) == 0x00);
    assert!(offset_of!(Plane, unknown_0004h) == 0x04);
    assert!(offset_of!(Plane, loadouts) == 0x08);

    assert!(size_of::<Loadout>() == 0x60);
    assert!(align_of::<Loadout>() == 8);
    assert!(offset_of!(Loadout, oid) == 0x00);
    assert!(offset_of!(Loadout, icon) == 0x20);
    assert!(offset_of!(Loadout, parts) == 0x40);
    assert!(offset_of!(Loadout, launch_loadout_weight) == 0x58);
    assert!(offset_of!(Loadout, has_gun37mm) == 0x5c);
    assert!(offset_of!(Loadout, unknown_005dh) == 0x5d);

    assert!(size_of::<Munition>() == 0x28);
    assert!(align_of::<Munition>() == 8);
    assert!(offset_of!(Munition, name) == 0x00);
    assert!(offset_of!(Munition, count) == 0x20);
    assert!(offset_of!(Munition, unknown_0024h) == 0x24);
};
