# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-07-28

A clean break from the 0.1.x line. The evaluation kernel that pkcore folded in
and improved (see pkcore EPIC-80) moves back down into this crate, which is now
a `no_std`, zero-dependency Cactus Kev kernel intended to serve pkcore,
cardpack.rs, fudd, and pokerhand as the single copy of the evaluator.

### Added

- `standard52` namespace holding everything French-deck-specific: `Card` (a
  `u32` newtype in the Cactus Kev bit layout), `CardNumber`, `Rank`, `Suit`,
  `SuitShift`, `Five`/`Six`/`Seven`, `HandRank`/`HandRankName`/`HandRankClass`,
  `HandRanker`, `HandValidator`, and `Five::eval`. Deliberately no
  root glob re-export, so a future deck family is an addition rather than a
  breaking reshuffle.
- `CkcError` (implements `core::error::Error`).
- `no_std` support: builds for `thumbv7em-none-eabi` and
  `wasm32-unknown-unknown` with `--no-default-features --features standard52`.
- Feature gates: `standard52`, `std`, `alloc`, and an optional `serde`
  (which implies `alloc`; cards serialize as strings such as `"A♠"`).
- Exhaustive verification: every one of the 2,598,960 five-card hands
  evaluates bit-identically to the frozen 0.1.18 release (`golden_oracle`),
  plus invalid-hand semantics tests and lookup-table identity hashes.

### Changed

- **`HandRanker::hand_rank_value` now validates by default**, returning
  `NO_HAND_RANK_VALUE` for hands that fail `HandValidator::is_valid()`
  (duplicates, blanks, or corrupt bit patterns). In 0.1.x validation was
  opt-in via `hand_rank_value_validated`. No result changes for any valid
  hand — proven exhaustively.
- The lookup tables are now `pub(crate)` behind `#[inline]` accessors instead
  of public constants, keeping their shape out of the API.
- Edition 2024, MSRV 1.85.
- Zero default dependencies: `strum` is gone (replaced by `const ALL` arrays
  with `iter()` on each enum) and `serde` is opt-in. The default dependency
  tree is exactly one crate — this one.

### Fixed

- Two latent panics on the public unguarded surface, both invisible to
  valid-hand testing: an off-by-one in `Five::unique_rank`'s bounds guard
  (`index == 7937` passed and indexed out of bounds), and a `usize` underflow
  in `Five::find_in_products` when the key sorted below every table entry
  (e.g. an all-blank hand). The binary search is now half-open.

### Known caveats

- `Five::find_in_products` returns `0` both as its not-found sentinel and as
  the legitimate index of the quad-deuces product, so `not_unique()` on
  garbage input reports rank 166. Unreachable through the validated
  evaluator path; documented on the method. Callers reaching it directly
  must validate first.

### Removed

- The entire 0.1.x public surface: `CKCNumber`, `PokerCard`, `Shifty`,
  `cards::{two,three,four,five,six,seven}`, `deck`, `parse`, and the
  unvalidated `hand_rank_value` semantics. Consumers pinned to `0.1.x` are
  unaffected until they migrate.

## [0.1.18] - 2025-06-23

Final release of the 0.1.x line, and the frozen differential oracle for the
0.2.0 rewrite. See the [git history](https://github.com/ImperialBower/ckc-rs)
for changes prior to 0.2.0.

[0.2.0]: https://github.com/ImperialBower/ckc-rs/compare/v0.1.18...v0.2.0
[0.1.18]: https://github.com/ImperialBower/ckc-rs/releases/tag/v0.1.18
