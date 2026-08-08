//! Cactus Kev lookup tables. See LICENSE in this directory for the MIT notice
//! covering the generated table data (Copyright (c) 2015 Vladislav Supalov).
//!
//! The tables are `pub(crate)` behind `#[inline]` accessors so the table *shape*
//! is not public API — a deck with a different rank count needs differently
//! dimensioned tables, and that must not be a breaking change here.

pub(crate) mod flushes;
pub(crate) mod products;
pub(crate) mod unique5;
pub(crate) mod values;

#[inline]
pub(crate) fn flush_rank(i: usize) -> u16 {
    flushes::FLUSHES[i]
}

#[inline]
pub(crate) fn unique_rank(i: usize) -> u16 {
    unique5::UNIQUE_5[i]
}

#[inline]
pub(crate) fn value_at(i: usize) -> u16 {
    values::VALUES[i]
}

#[inline]
pub(crate) fn product_at(i: usize) -> u32 {
    products::PRODUCTS[i]
}
