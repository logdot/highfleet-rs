//! v1.163

use serde::{Deserialize, Serialize};

use crate::general::escadra_string::EscadraString;

/// Represents an Ammo object in Highfleet
#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Ammo {
    /// What reticle to use when firing the ammo.
    ///
    /// The vanilla ammos use one of four values:
    /// - 1: Standard reticle used by most ammos.
    /// - 2: Used by aircraft bombs.
    /// - 3: Used mostly by rockets.
    /// - 4: Used by aircraft ammos.
    pub reticle: i32,
    /// Unused padding bytes?
    /// Not always set to 0.
    pub padding_4h: u32,
    /// The internal name for the item within Highfleet.
    pub item_name: EscadraString,
    /// The text that displays the shell's kind in the shop.
    ///
    /// For example: "Incendiary".
    pub shell_kind: EscadraString,
    /// The internal text to determine the shell's kind.
    ///
    /// For example: "@INCENDIARY".
    pub shell_kind2: EscadraString,
    /// The text to display for the ammo's milimeter in the shop.
    ///
    /// For example: "57mm".
    pub milimeterage: EscadraString,
    /// The image to use for the ammo in the magazine.
    /// These are defined in the .res files inside of the Tex folder.
    ///
    /// The standard ammo images are defined in the midground2.res file.
    /// But you can use the images from the other .res files as well.
    ///
    /// Note that this doesn't have to be an actual image, it can be an animation.
    /// When setting it to an animation include the full name. E.g. "animation_name_01"
    pub magazine_image: EscadraString,
    /// What sign to use for the reticle?
    ///
    /// In vanilla it's one of these:
    /// - "sign_ammo_unset" for the standard rounds.
    /// - "sign_ammo_inc_small" for small incendiary rounds.
    /// - "sign_ammo_ap" for armour piercing rounds.
    /// - "sign_ammo_proxy" for proxy rounds.
    /// - "sign_ammo_inc" for standard incendiary rounds.
    /// - "sign_ammo_guided" for lazer guided rounds.
    /// - "sign_ammo_craft" for rounds (bombs, or rockets) used by aircraft.
    pub sign_ammo: EscadraString,
    /// How tall the bullet is in the magazine.
    ///
    /// In the vanilla game it ranges from 16 to 38.
    pub bullet_height: f32,
    /// Unused padding bytes by the game.
    ///
    /// Ammo Extended hijacks this value to determine shell behaviour.
    /// It must be a vanilla value.
    pub padding_cch: u32,
    /// The sound set to play when a shell is loaded into the magazine.
    ///
    /// This has to be a sound set, otherwise nothing will play.
    /// A sound set is a variable length set of sound files defined in sound.res file.
    ///
    /// For example:
    /// - crowd_01
    /// - crowd_02
    /// - crowd_03
    ///
    /// Would be one sound set.
    ///
    /// Unlike the magazine_image, do not use the full name.
    /// Use the name of the sound set.
    /// Using the crowd sound set as an example, the name would be "crowd"
    ///
    /// A sound set is valid even when it includes only one sound.
    ///
    /// In vanilla it is one of these two values:
    /// - "shell_in_small"
    /// - "shell_in_med"
    pub shell_in: EscadraString,
    /// The sound set to play when firing the gun.
    ///
    /// See `shell_in` for more information.
    ///
    /// In vanilla it would be one of these values:
    /// - "shell_out_tiny"
    /// - "shell_out_small"
    /// - "shell_out_small2"
    /// - "shell_out_med"
    /// - "shell_out_big"
    /// - "shell_out_big3"
    pub shell_out: EscadraString,
    /// The sound set to play when an enemy is firing the gun.
    ///
    /// See `shell_in` for more information.
    ///
    /// In vanilla it would be one of these values:
    /// - "shell_out_enemy_tiny"
    /// - "shell_out_enemy_med"
    /// - "shell_out_enemy_big"
    pub shell_enemy: EscadraString,
    /// The sound set to play when the gun is fired from far away.
    ///
    /// See `shell_in` for more information.
    ///
    /// In vanilla it would be one of these values:
    /// - "shell_out_tiny_far"
    /// - "shell_out_small_far"
    /// - "shell_out_med_far"
    /// - "shell_out_big_far"
    pub shell_far: EscadraString,
    /// Determines if the shell behaves like HE, AP, INC, or LG?
    ///
    /// In vanilla it is one of these values:
    /// - 100: The default
    /// - 130: Rocket and Incendiary?
    /// - 140: Laser Guided
    /// - 160: Proxy
    pub caliber: i32,
    /// The index of the ammo.
    /// A weapon's m_weapon_caliber should match with an ammo index.
    pub index: i32,
    /// The speed of the shell.
    pub speed: f32,
    /// The drag the shell experiences?
    ///
    /// A value between 0 and 1.
    /// In the vanilla game it's either set to 0 or to 0.0007
    pub ap_drag: f32,
    /// The shell's explosive power.
    /// Higher is better.
    pub explosive_power: f32,
    /// The shell's penetrative power.
    /// Higher is better.
    pub penetrative_power: f32,
    /// The shell's incendiary power.
    /// Higher is better.
    ///
    /// By default it is 100.0, with incendiary rounds having it set to 1000.0
    pub incendiary_power: f32,
    /// Determines how long a shell will last in the air.
    ///
    /// In vanilla ranges from 30 to 1.
    #[serde(alias = "unknown_16ch")]
    pub ttl: f32,
    /// The price of the ammo inside of city shops.
    pub shop_price: i32,
    /// Determines how rare the ammo is in the shop.
    ///
    /// Percentage value between 0 and 1.
    /// Non special ammos have it set to 0.0.
    #[serde(alias = "unknown_174h")]
    pub shop_rarity: f32,
    /// On average, how much of the ammo is available in the shop.
    /// Ranges from 0.0 to 500.0 in vanilla.
    ///
    /// Non special ammos have it set to 0.0.
    #[serde(alias = "unknown_178h")]
    pub shop_ammount: f32,
    /// How long it takes from "pulling the trigger" to the bullet being fired.
    /// Standard guns are unaffected by this value.
    /// Affects plane payload release.
    ///
    /// Value between 0 and 1.
    /// By default it is 0.5.
    ///
    /// The only exceptions are:
    /// - The NAR122 where it's 0.2
    /// - The 37MM aircraft rounds where it's 0.05
    /// - The 57MM aircraft rounds where it's 0.2
    #[serde(alias = "unknown_17ch")]
    pub fire_delay: f32,
    /// Value with unknown purpose.
    /// By default it is 10.
    ///
    /// The only exception are:
    /// - The 37MM aircraft rounds where it's 20.
    /// - The 57MM aircraft rounds where it's 7.
    pub unknown_180h: i32,
    /// Unused padding bytes
    pub padding_184h: u32,
}
