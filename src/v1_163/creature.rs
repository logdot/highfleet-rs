//! Native Creature (ship) live view for Steam 1.163.

use super::Body;
use crate::general::{CVec, Link};

/// Native Creature layout for Steam 1.163, with the copy routine's complete allocation size of `0x1A50`.
///
/// `CopyObject` at `0x1401BFD00` loads the immediate `0x1A50` at `0x1401BFDE1`, allocates it, and calls the copy
/// constructor at `0x14005C7A0` at `0x1401BFDFA`.
/// That copy constructor is not the allocating factory: it calls `Body::Body` (`0x140015ED0`) at `0x14005C7BF` and
/// then installs the Creature vtable at `0x140354C40`.
/// `Creature_InitBaseFields` at `0x140072B90` writes the class code `0x2F`, the `mode` dword at `0xB48`, the
/// `order_role` dword at `0xD70`, and sets the `ai_flag` byte at `0xBD4` to `1`; it initializes fields through `0x1A48`.
///
/// No Creature serializer was recovered; fields follow native accesses and consumer widths.
/// Mode, driver state and order role are exposed as raw native integers, with no enum variants asserted.
/// The `side` field is exposed as its native float, without asserting side values or variants.
/// The exact application effect of the trigger bytes is consumer-derived, not proven by a serializer.
///
/// This is a live view only.
/// The embedded `Body` contains native strings and vectors.
/// Each `CVec<Link>` owns its vector allocation, while the `Link` targets are non-owning; the elements are `Link`
/// records, not owning pointers.
/// Do not construct a Creature in Rust, and do not use Rust drop as a substitute for the native virtual destructor.
/// Never use `ptr::read` or `read_unaligned` to create an owning copy of a game-owned Creature.
/// Change Links through the game's link operations so registration remains intact.
/// Borrow only a live, aligned object of this version through a raw pointer or a short-lived reference.
/// Do not create a borrow that outlives the native object or conflicts with the game's access.
/// Unknown regions are native state, not mod storage.
#[repr(C)]
pub struct Creature {
    /// Non-virtual `Body` base at offset zero.
    /// `0x14005C7A0` calls `Body::Body` and then installs the Creature table.
    pub base: Body,
    /// Uninterpreted native state at `0x0A40..0x0AD4`; not disposable padding.
    pub unknown_0a40h: [u8; 0x94],
    /// Native side float at `0x0AD4`.
    /// Exposed as its native representation; no side values or variants are asserted here.
    pub side: f32,
    /// Uninterpreted native state at `0x0AD8..0x0AF8`; not disposable padding.
    pub unknown_0ad8h: [u8; 0x20],
    /// Native `Link` at `0x0AF8`, constructed by the native Link constructor and tracking the selected ship target.
    /// `Link::target` at `+8` is the node at `0x0B00`, which may be a ship or one of its parts.
    pub target: Link,
    /// Uninterpreted native state at `0x0B10..0x0B28`; not disposable padding.
    pub unknown_0b10h: [u8; 0x18],
    /// Native retarget countdown in seconds at `0x0B28`; the target chooser resets it to `3.0` after a pick.
    pub retarget_timer: f32,
    /// Uninterpreted native state at `0x0B2C..0x0B48`; not disposable padding.
    pub unknown_0b2ch: [u8; 0x1c],
    /// Native mode integer at `0x0B48`, written by `Creature_InitBaseFields`.
    /// Consumers branch on it; the variant values are not modeled and no meaning is asserted here.
    pub mode: i32,
    /// Uninterpreted native state at `0x0B4C..0x0BBC`; not disposable padding.
    pub unknown_0b4ch: [u8; 0x70],
    /// Raw trigger byte at `0x0BBC`.
    /// `CreatureUpdate` (`0x14007EB50`) fires the group-0 gun list (`0x13A8`) while it is non-zero.
    /// Raw flag byte, not a Rust `bool`.
    pub trigger_gun_group_0: u8,
    /// Raw trigger byte at `0x0BBD`.
    /// `CreatureUpdate` fires the group-1 gun list (`0x13C0`) while it is non-zero.
    /// Raw flag byte, not a Rust `bool`.
    pub trigger_gun_group_1: u8,
    /// Raw trigger byte at `0x0BBE`.
    /// The native per-creature update calls `0x14002E7D0` on the first non-null entry of the list at `0x1438`, then
    /// clears this byte. The exact ordnance effect is not established.
    pub trigger_bomb: u8,
    /// Uninterpreted native state at `0x0BBF`; a separate native flag byte, not disposable padding.
    pub unknown_0bbfh: [u8; 0x1],
    /// Raw trigger byte at `0x0BC0`.
    /// `CreatureUpdate` calls the flare release on every launcher of the list at `0x1408` while it is non-zero.
    /// It is not cleared by that path.
    pub trigger_flare: u8,
    /// Uninterpreted native state at `0x0BC1..0x0BD4`; not disposable padding.
    pub unknown_0bc1h: [u8; 0x13],
    /// Raw AI flag byte at `0x0BD4`.
    /// `Creature_InitBaseFields` sets it to `1` and the player-control setter clears it to `0`.
    /// Raw flag byte, not a Rust `bool`.
    pub ai_flag: u8,
    /// Uninterpreted native state at `0x0BD5..0x0BDC`; not disposable padding.
    pub unknown_0bd5h: [u8; 0x7],
    /// Native driver state integer at `0x0BDC`, switched by `aiShipDriver` (`0x1400739C0`).
    /// States are not modeled as an enum.
    pub driver_state: i32,
    /// Uninterpreted native state at `0x0BE0..0x0BF0`; not disposable padding.
    pub unknown_0be0h: [u8; 0x10],
    /// Native AI waypoint x at `0x0BF0`, written by the AI steering path.
    pub waypoint_x: f32,
    /// Native AI waypoint y at `0x0BF4`, written by the AI steering path.
    pub waypoint_y: f32,
    /// Uninterpreted native state at `0x0BF8..0x0D70`; not disposable padding.
    pub unknown_0bf8h: [u8; 0x178],
    /// Native order-role integer at `0x0D70`, written by `Creature_InitBaseFields`.
    /// The target chooser and the battle manager test it; role values are not modeled as an enum.
    pub order_role: i32,
    /// Uninterpreted native state at `0x0D74..0x1330`; not disposable padding.
    pub unknown_0d74h: [u8; 0x5bc],
    /// Native `CVec<Link>` header at `0x1330` holding the ship's part list.
    /// Each `Link::target` at element `+8` points at a part node.
    pub parts: CVec<Link>,
    /// Uninterpreted native state at `0x1348..0x1360`; not disposable padding.
    pub unknown_1348h: [u8; 0x18],
    /// Native `CVec<Link>` header at `0x1360` holding the dependency-driven ammo-consumer list.
    /// `Creature_AccumulatePartStats` (`0x140071380`) appends a part's `Link` when its stats `+0xE8` (ammobox) is zero
    /// and `+0xEC` (ammobox_need) is non-zero; this is not an exclusive gun list.
    pub ammo_consumers: CVec<Link>,
    /// Uninterpreted native state at `0x1378..0x13A8`; not disposable padding.
    pub unknown_1378h: [u8; 0x30],
    /// Native `CVec<Link>` header at `0x13A8` holding the group-0 gun list.
    /// `CreatureUpdate` and the AI weapon controller walk it.
    pub gun_group_0: CVec<Link>,
    /// Native `CVec<Link>` header at `0x13C0` holding the group-1 gun list.
    pub gun_group_1: CVec<Link>,
    /// Native `CVec<Link>` header at `0x13D8` holding the anti-ship launch list.
    /// The native guided-missile launch path uses this list.
    pub anti_ship_launchers: CVec<Link>,
    /// Uninterpreted native state at `0x13F0..0x1408`; not disposable padding.
    pub unknown_13f0h: [u8; 0x18],
    /// Native `CVec<Link>` header at `0x1408` holding the flare launcher list.
    pub flare_launchers: CVec<Link>,
    /// Uninterpreted native state at `0x1420..0x1438`; not disposable padding.
    pub unknown_1420h: [u8; 0x18],
    /// Native `CVec<Link>` header at `0x1438` holding the bomb list.
    pub bombs: CVec<Link>,
    /// Uninterpreted native state at `0x1450..0x14C8`; not disposable padding.
    pub unknown_1450h: [u8; 0x78],
    /// Native `CVec<Link>` header at `0x14C8` holding the carrier's plane parts.
    pub planes: CVec<Link>,
    /// Uninterpreted native state at `0x14E0..0x15B8`; not disposable padding.
    pub unknown_14e0h: [u8; 0xd8],
    /// Native total mass float at `0x15B8`, summed by `CalculateStats` from the ship's parts.
    pub mass: f32,
    /// Uninterpreted native state at `0x15BC..0x1638`; not disposable padding.
    pub unknown_15bch: [u8; 0x7c],
    /// Native current fuel double at `0x1638`, summed by the stats pass from the parts' fuel.
    pub fuel: f64,
    /// Native fuel capacity double at `0x1640`.
    pub fuel_capacity: f64,
    /// Uninterpreted native state at `0x1648..0x1664`; not disposable padding.
    pub unknown_1648h: [u8; 0x1c],
    /// Native engine-bank thrust sum at `0x1664`, accumulated from engine statistics.
    pub thrust_group_0: f32,
    /// Native engine-bank thrust sum at `0x1668`, accumulated from engine statistics.
    pub thrust_group_1: f32,
    /// Uninterpreted native state at `0x166C..0x1838`; not disposable padding.
    pub unknown_166ch: [u8; 0x1cc],
    /// Native group-0 weapon reach float at `0x1838`, read by the AI range gate and stand-off calculation.
    pub weapon_reach_group_0: f32,
    /// Native group-1 weapon reach float at `0x183C`, read by the AI range gate and stand-off calculation.
    pub weapon_reach_group_1: f32,
    /// Native bounding-box centre x at `0x1840`, written by `CalculateStats`.
    /// Computed as the midpoint of the parts' min/max world x, from their `Body` half extents at `+0x218`/`+0x21C`.
    /// This is not a mass-weighted centre of mass and is unrelated to `Body` `+0x210`/`+0x214`.
    pub bounds_center_x: f32,
    /// Native bounding-box centre y at `0x1844`, from the same `CalculateStats` min/max calculation.
    pub bounds_center_y: f32,
    /// Uninterpreted native state at `0x1848..0x1850`; not disposable padding.
    pub unknown_1848h: [u8; 0x8],
    /// Native bounding radius float at `0x1850`, written by `CalculateStats`.
    pub bounding_radius: f32,
    /// Uninterpreted native state at `0x1854..0x1A50`; not disposable padding.
    pub unknown_1854h: [u8; 0x1fc],
}

const _: () = {
    use core::mem::{align_of, offset_of, size_of};

    assert!(size_of::<Body>() == 0xA40);
    assert!(size_of::<Link>() == 0x18);
    assert!(size_of::<CVec<Link>>() == 0x18);

    assert!(offset_of!(Creature, base) == 0x0);
    assert!(offset_of!(Creature, unknown_0a40h) == 0xA40);
    assert!(offset_of!(Creature, side) == 0xAD4);
    assert!(offset_of!(Creature, unknown_0ad8h) == 0xAD8);
    assert!(offset_of!(Creature, target) == 0xAF8);
    assert!(offset_of!(Creature, unknown_0b10h) == 0xB10);
    assert!(offset_of!(Creature, retarget_timer) == 0xB28);
    assert!(offset_of!(Creature, unknown_0b2ch) == 0xB2C);
    assert!(offset_of!(Creature, mode) == 0xB48);
    assert!(offset_of!(Creature, unknown_0b4ch) == 0xB4C);
    assert!(offset_of!(Creature, trigger_gun_group_0) == 0xBBC);
    assert!(offset_of!(Creature, trigger_gun_group_1) == 0xBBD);
    assert!(offset_of!(Creature, trigger_bomb) == 0xBBE);
    assert!(offset_of!(Creature, unknown_0bbfh) == 0xBBF);
    assert!(offset_of!(Creature, trigger_flare) == 0xBC0);
    assert!(offset_of!(Creature, unknown_0bc1h) == 0xBC1);
    assert!(offset_of!(Creature, ai_flag) == 0xBD4);
    assert!(offset_of!(Creature, unknown_0bd5h) == 0xBD5);
    assert!(offset_of!(Creature, driver_state) == 0xBDC);
    assert!(offset_of!(Creature, unknown_0be0h) == 0xBE0);
    assert!(offset_of!(Creature, waypoint_x) == 0xBF0);
    assert!(offset_of!(Creature, waypoint_y) == 0xBF4);
    assert!(offset_of!(Creature, unknown_0bf8h) == 0xBF8);
    assert!(offset_of!(Creature, order_role) == 0xD70);
    assert!(offset_of!(Creature, unknown_0d74h) == 0xD74);
    assert!(offset_of!(Creature, parts) == 0x1330);
    assert!(offset_of!(Creature, unknown_1348h) == 0x1348);
    assert!(offset_of!(Creature, ammo_consumers) == 0x1360);
    assert!(offset_of!(Creature, unknown_1378h) == 0x1378);
    assert!(offset_of!(Creature, gun_group_0) == 0x13A8);
    assert!(offset_of!(Creature, gun_group_1) == 0x13C0);
    assert!(offset_of!(Creature, anti_ship_launchers) == 0x13D8);
    assert!(offset_of!(Creature, unknown_13f0h) == 0x13F0);
    assert!(offset_of!(Creature, flare_launchers) == 0x1408);
    assert!(offset_of!(Creature, unknown_1420h) == 0x1420);
    assert!(offset_of!(Creature, bombs) == 0x1438);
    assert!(offset_of!(Creature, unknown_1450h) == 0x1450);
    assert!(offset_of!(Creature, planes) == 0x14C8);
    assert!(offset_of!(Creature, unknown_14e0h) == 0x14E0);
    assert!(offset_of!(Creature, mass) == 0x15B8);
    assert!(offset_of!(Creature, unknown_15bch) == 0x15BC);
    assert!(offset_of!(Creature, fuel) == 0x1638);
    assert!(offset_of!(Creature, fuel_capacity) == 0x1640);
    assert!(offset_of!(Creature, unknown_1648h) == 0x1648);
    assert!(offset_of!(Creature, thrust_group_0) == 0x1664);
    assert!(offset_of!(Creature, thrust_group_1) == 0x1668);
    assert!(offset_of!(Creature, unknown_166ch) == 0x166C);
    assert!(offset_of!(Creature, weapon_reach_group_0) == 0x1838);
    assert!(offset_of!(Creature, weapon_reach_group_1) == 0x183C);
    assert!(offset_of!(Creature, bounds_center_x) == 0x1840);
    assert!(offset_of!(Creature, bounds_center_y) == 0x1844);
    assert!(offset_of!(Creature, unknown_1848h) == 0x1848);
    assert!(offset_of!(Creature, bounding_radius) == 0x1850);
    assert!(offset_of!(Creature, unknown_1854h) == 0x1854);

    assert!(size_of::<Creature>() == 0x1A50);
    assert!(align_of::<Creature>() == 8);
};
