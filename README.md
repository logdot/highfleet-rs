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
- `TllContainer<T, U>` and `Tll<T, U>`, the game's native-layout red-black map (std::map<K, V>)

Library includes extensive documentation (deny missing docs is enable) and tests.

## Native tree ownership

The generic tree owns its libc-allocated nodes and initialized payloads; game-owned trees must remain borrowed.
See the [native tree module documentation](src/general/tll.rs) for the usage example, layout, cloning, cleanup, and ownership-transfer requirements.
