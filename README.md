# Highfleet-rs
Defines types in Highfleet.
Designed for creating Highfleet mods.

The library is split into three main componets.
General, 1.151, and 1.163.
Types in General are not expected to change between game releases.
The same types are in 1.151 and 1.163, but change depending on the game version.
Types are added on an as-needed basis.

Currently defines:
- EscadraStrings, custom string type used by the game
- Ammo, struct for ammo types
- TLL, "triply linked list"

Library includes extensive documentation (deny missing docs is enable) and tests.
