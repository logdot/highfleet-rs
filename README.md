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
- `CVec<T>`, the game's native-layout three-pointer vector (std::vector<T>)
- `TllContainer<T, U>` and `Tll<T, U>`, the game's native-layout red-black map (std::map<K, V>)

Library includes extensive documentation (deny missing docs is enable) and tests.

## Native container ownership

See the [native vector module documentation](src/general/cvec.rs) for `CVec` access, layout, cleanup, and ownership-transfer requirements.

The generic tree owns its libc-allocated nodes and initialized payloads, including any vectors stored in those payloads.
See the [native tree module documentation](src/general/tll.rs) for the usage example, layout, cloning, cleanup, and ownership-transfer requirements.

Game-owned objects and containers must remain borrowed through pointers or references, not copied into Rust ownership with `ptr::read` or destroyed with `drop_in_place`.
Rust-owned `Body` values must initialize every string and vector field before use or drop, even when constructing opaque state through `MaybeUninit`.
Rust field destruction is not the game's virtual destructor, and non-owning sentinel-list headers do not free their nodes.
