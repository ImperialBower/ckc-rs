pub struct Razz;

impl Razz {
    //                   AKQJT|98765432
    //                   bbbbb|bbbbbbbb
    pub const WHEEL: u32 = 0b1_0000_0000_1111;
}

#[cfg(test)]
#[allow(non_snake_case)]
mod games__razz {
    use super::*;
    use rstest::rstest;
    use crate::cards::binary_card::{BinaryCard, BC64};
    use crate::cards::five::Five;

    #[rstest]
    #[case("2D 3C 4D A♠ 5H", Razz::WHEEL)]
    pub fn razz(#[case] index: &str, #[case] expected: u32) {
        let five = BinaryCard::from_index(index);

        // let rank_bits = five.or_rank_bits();

        // assert_eq!(rank_bits, Razz::WHEEL)
    }
}