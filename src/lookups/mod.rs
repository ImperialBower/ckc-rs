/// This code was taken from [Vladislav Supalov's](https://github.com/vsupalov)
/// [pokereval-rs](https://github.com/vsupalov/pokereval-rs) library, which in
/// turn was based on Cactus Kev's (aka [Kevin Suffecool](https://suffe.cool/))
/// [Poker Hand Evaluator](https://suffe.cool/poker/evaluator.html) code in C.
///
/// ```txt
/// Copyright (c) 2015 Vladislav Supalov
///
/// Permission is hereby granted, free of charge, to any person obtaining a copy
/// of this software and associated documentation files (the "Software"), to deal
/// in the Software without restriction, including without limitation the rights
/// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
/// copies of the Software, and to permit persons to whom the Software is
/// furnished to do so, subject to the following conditions:
///
/// The above copyright notice and this permission notice shall be included in
/// all copies or substantial portions of the Software.
///
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
/// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
/// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
/// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
/// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
/// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
/// THE SOFTWARE.
/// ```
///
///
/// this is a table lookup for all "flush" hands (e.g.  both
/// flushes and straight-flushes.  entries containing a zero
/// mean that combination is not possible with a five-card
/// flush hand.
pub mod flushes;
pub mod products;
pub mod unique5;
pub mod values;

/// Proof that the tables above are derived, not magic.
///
/// `examples/gen_lookups.rs` rebuilds all four arrays from the rules of poker.
/// It is pulled in here rather than duplicated so there is exactly one copy of
/// the derivation, and these tests fail the moment it drifts from the committed
/// files. See that file for the full explanation of how the tables work.
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../examples/gen_lookups.rs"]
mod gen;

#[cfg(test)]
mod lookups_tests {
    use super::*;

    #[test]
    fn generated_flushes_match() {
        assert_eq!(super::gen::build().flushes, flushes::FLUSHES.to_vec());
    }

    #[test]
    fn generated_unique5_match() {
        assert_eq!(super::gen::build().unique5, unique5::UNIQUE_5.to_vec());
    }

    #[test]
    fn generated_products_match() {
        assert_eq!(super::gen::build().products, products::PRODUCTS.to_vec());
    }

    #[test]
    fn generated_values_match() {
        assert_eq!(super::gen::build().values, values::VALUES.to_vec());
    }
}
