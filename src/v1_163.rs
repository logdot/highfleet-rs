//! Defines the types needed for interfacing with version 1.163 of Highfleet.

pub use crate::general::*;

pub mod aircraft_logic;
pub use aircraft_logic::*;

pub mod ammo;
pub use ammo::*;

pub mod body;
pub use body::*;

pub mod mesh;
pub use mesh::*;

pub mod creature;
pub use creature::*;

pub mod escadra;
pub use escadra::*;

pub mod gui;
pub use gui::*;

pub mod location;
pub use location::*;

pub mod mark;
pub use mark::*;

pub mod plane;
pub use plane::*;

pub mod npc;
pub use npc::*;

pub mod profile;
pub use profile::*;

pub mod slider;
pub use slider::*;

pub mod part_stats;
pub use part_stats::*;
