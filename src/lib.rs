#![no_std]
#![allow(
    // Cactus Kev card constants and lookup tables are bit-pattern layouts where
    // digit separators would obscure 32-bit structure. These are byte-identical
    // to pkcore originals and must not be edited.
    clippy::unreadable_literal,
)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod error;
pub mod prelude;

#[cfg(feature = "standard52")]
pub mod standard52;

pub use error::CkcError;
