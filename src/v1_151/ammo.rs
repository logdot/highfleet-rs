//! v1.151

use serde::{Deserialize, Serialize};

use crate::general::escadra_string::EscadraString;

/// Represents an Ammo object in Highfleet
#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Ammo {
    /// Selects the aiming reticle and affects firing AI.
    ///
    /// Vanilla values include:
    /// - 1: Standard reticle used by most ammos.
    /// - 2: Used by aircraft bombs.
    /// - 3: Used mostly by rockets.
    /// - 4: Used by aircraft ammos.
    /// - 5: K13.
    /// - 6: Harpoon.
    /// - 7: KAZ.
    pub reticle: i32,
    /// Unused padding bytes?
    /// Not always set to 0.
    pub padding_4h: u32,
    /// The internal name for the item within Highfleet.
    pub item_name: EscadraString,
    /// The text that displays the shell's kind in the shop.
    ///
    /// For example: "Incendiary".
    pub display_name: EscadraString,
    /// Cosmetic display text used in the prebattle screen.
    ///
    /// For example: "@INCENDIARY".
    pub display_name2: EscadraString,
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
    /// The ammo icon used in the prebattle screen.
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
    /// Native padding bytes, unused by the game.
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
    /// Selects shell behaviour, independently of the weapon's caliber identifier.
    ///
    /// Known values:
    /// - 100: Default shell logic.
    /// - 130: Rocket and incendiary logic.
    /// - 135: Harpoon logic.
    /// - 140: Laser-guided logic.
    /// - 150: AP airburst logic with fixed damage and fuze behaviour.
    /// - 160: Proximity-fuzed logic.
    ///
    /// The behaviour of 110 and 120 is unknown.
    /// Value 132 is used by shipped 300mm ammo.
    pub logic: i32,
    /// The ammo's caliber identifier, matched by a weapon's `m_weapon_caliber`.
    pub caliber: i32,
    /// The shell's speed.
    /// Negative values are possible.
    ///
    /// At the maximum vanilla framerate, effective speed is capped at 2000.
    pub speed: f32,
    /// The shell's drag, which interacts with its mass.
    /// Negative values are possible.
    ///
    /// In vanilla it is set to either 0 or 0.0007.
    pub ap_drag: f32,
    /// The shell's explosive power.
    ///
    /// Increasing this also increases weight and affects kinetic/contact damage.
    /// It may also affect the hitbox, but that effect is uncertain.
    pub explosive_power: f32,
    /// The shell's penetrative power.
    /// Higher values increase penetration.
    pub penetrative_power: f32,
    /// The shell's incendiary power.
    /// Higher is better.
    ///
    /// The default is 100.0; incendiary rounds use 1000.0.
    pub incendiary_power: f32,
    /// The price of the ammo inside of city shops.
    pub shop_price: i32,
    /// Availability probability for initial city shop stock.
    ///
    /// Vanilla values are between 0 and 1; non-special ammos use 0.0.
    pub shop_rarity: f32,
    /// Base initial city shop stock quantity.
    ///
    /// Stock is sampled from the base amount to twice the base, then converted to an integer.
    /// Vanilla values range from 0.0 to 500.0; non-special ammos use 0.0.
    pub shop_ammount: f32,
    /// Delay between plane payload releases; larger values make release slower.
    /// Standard guns are unaffected by this value.
    ///
    /// Vanilla values are between 0 and 1.
    /// By default it is 0.5.
    ///
    /// The only exceptions are:
    /// - The NAR122 where it's 0.2
    /// - The 37MM aircraft rounds where it's 0.1
    /// - The 57MM aircraft rounds where it's 0.2
    pub fire_delay: f32,
    /// Number of payload releases per aircraft burst/strafe.
    /// For aircraft guns, initializes both magazine capacity and loaded rounds.
    /// By default it is 10.
    ///
    /// The only exception is the 57MM aircraft rounds where it's 7.
    pub salvo_size: i32,
    /// Contribution to map-aircraft weapon-strength scoring, not projectile damage.
    /// Carried-count scaling depends on the weapon category.
    ///
    /// For example, the 37MM aircraft gun contributes 1.0 and the 57MM gun contributes 2.0.
    pub aircraft_strength_contribution: f32,
    /// Native padding bytes.
    pub padding_164h: u32,
}

#[cfg(test)]
mod tests {
    use super::Ammo;
    use std::mem::{offset_of, size_of};

    #[test]
    fn native_layout() {
        assert_eq!(size_of::<Ammo>(), 0x168);
        assert_eq!(offset_of!(Ammo, display_name), 0x28);
        assert_eq!(offset_of!(Ammo, display_name2), 0x48);
        assert_eq!(offset_of!(Ammo, padding_cch), 0xcc);
        assert_eq!(offset_of!(Ammo, logic), 0x130);
        assert_eq!(offset_of!(Ammo, caliber), 0x134);
        assert_eq!(offset_of!(Ammo, speed), 0x138);
        assert_eq!(offset_of!(Ammo, ap_drag), 0x13c);
        assert_eq!(offset_of!(Ammo, explosive_power), 0x140);
        assert_eq!(offset_of!(Ammo, penetrative_power), 0x144);
        assert_eq!(offset_of!(Ammo, incendiary_power), 0x148);
        assert_eq!(offset_of!(Ammo, shop_price), 0x14c);
        assert_eq!(offset_of!(Ammo, shop_rarity), 0x150);
        assert_eq!(offset_of!(Ammo, shop_ammount), 0x154);
        assert_eq!(offset_of!(Ammo, fire_delay), 0x158);
        assert_eq!(offset_of!(Ammo, salvo_size), 0x15c);
        assert_eq!(offset_of!(Ammo, aircraft_strength_contribution), 0x160);
        assert_eq!(offset_of!(Ammo, padding_164h), 0x164);
    }
}
