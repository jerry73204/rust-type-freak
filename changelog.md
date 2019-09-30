# Change Log

## v0.2.0
### Added
- New `functional` module that provides trait-level functional programming capability. Now it includes `Functor`, `Compose`, `FMap` and `Applicative`.
- New `KVLIst` trait that wraps around `TList`, which is a typed list with extra values.
- More comprehensive docs. The crate level doc describes brief usage and naming conventions.
- New `Imply` and `NotImply` boolean operators.
- New `LZip` `LUnzip` and `LSplit` operators for `TList`.
- Now `TList` and `Maybe` supports mapping, filtering and scanning functional interface.

### Changed
- Rewrite most type operators into types with `Functor` for better functional programming experience.
- Fix wrong implementation in `LInsertAt`.
- Many bug fixed and code refactoring.

## v0.1.2
### Added
- Trait-level `TList` with insertion, removal and indexing.

- `Boolean` trait and trait-level boolean algebra.
- `Maybe` trait that is analogous to `Option`.
- `IfSame`, `IfLess` primitives for static guard and assertion.
