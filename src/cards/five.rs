use crate::cards::HandValidator;
use crate::hand_rank::{HandRank, HandRankValue};
use crate::{CKCNumber, CardNumber, HandError, PokerCard};
use core::slice::Iter;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct Five([CKCNumber; 5]);

impl Five {
    pub const POSSIBLE_COMBINATIONS: usize = 7937;
    /// The number of leading and trailing zeroes from the `Five.or_rank_bits()` of a straight
    /// if it's not a wheel (5♥ 4♥ 3♥ 2♠ A♠).
    pub const STRAIGHT_PADDING: u32 = 27;
    pub const WHEEL_OR_BITS: u32 = 4111;

    //region accessors

    #[must_use]
    pub fn second(&self) -> CKCNumber {
        self.0[1]
    }

    #[must_use]
    pub fn third(&self) -> CKCNumber {
        self.0[2]
    }

    #[must_use]
    pub fn forth(&self) -> CKCNumber {
        self.0[3]
    }

    #[must_use]
    pub fn fifth(&self) -> CKCNumber {
        self.0[4]
    }

    pub fn set_first(&mut self, card_number: CKCNumber) {
        self.0[0] = card_number;
    }

    pub fn set_second(&mut self, card_number: CKCNumber) {
        self.0[1] = card_number;
    }

    pub fn set_third(&mut self, card_number: CKCNumber) {
        self.0[2] = card_number;
    }

    pub fn set_forth(&mut self, card_number: CKCNumber) {
        self.0[3] = card_number;
    }

    pub fn set_fifth(&mut self, card_number: CKCNumber) {
        self.0[4] = card_number;
    }

    #[must_use]
    pub fn to_arr(&self) -> [CKCNumber; 5] {
        self.0
    }

    //endregion

    fn from_index(index: &str) -> Option<[CKCNumber; 5]> {
        let mut esses = index.split_whitespace();

        let first = CKCNumber::from_index(esses.next()?);
        let second = CKCNumber::from_index(esses.next()?);
        let third = CKCNumber::from_index(esses.next()?);
        let forth = CKCNumber::from_index(esses.next()?);
        let fifth = CKCNumber::from_index(esses.next()?);
        let hand: [CKCNumber; 5] = [first, second, third, forth, fifth];
        Some(hand)
    }

    //region evaluate

    #[must_use]
    pub fn hand_rank(&self) -> HandRank {
        HandRank::from(self.hand_rank_value())
    }

    #[must_use]
    pub fn hand_rank_validated(&self) -> HandRank {
        HandRank::from(self.hand_rank_value_validated())
    }

    #[must_use]
    pub fn hand_rank_value(&self) -> HandRankValue {
        let i = self.or_rank_bits() as usize;

        if self.is_flush() {
            return crate::lookups::FLUSHES[i];
        }

        // Continue to evaluate if it's not a flush and the cards aren't
        // unique (straight or high card).
        let unique = Five::unique(i);
        match unique {
            0 => self.not_unique(),
            _ => unique,
        }
    }

    #[must_use]
    pub fn hand_rank_value_validated(&self) -> HandRankValue {
        if !self.is_valid() {
            return crate::hand_rank::NO_HAND_RANK_VALUE;
        }
        self.hand_rank_value()
    }

    #[must_use]
    #[allow(clippy::comparison_chain)]
    pub fn find_in_products(key: usize) -> usize {
        let mut low = 0;
        let mut high = 4887;
        let mut mid;

        while low <= high {
            mid = (high + low) >> 1; // divide by two

            let product = crate::lookups::PRODUCTS[mid] as usize;
            if key < product {
                high = mid - 1;
            } else if key > product {
                low = mid + 1;
            } else {
                return mid;
            }
        }
        0
    }

    fn not_unique(&self) -> HandRankValue {
        crate::lookups::VALUES[Five::find_in_products(self.multiply_primes())]
    }

    #[allow(clippy::cast_possible_truncation)]
    fn unique(index: usize) -> HandRankValue {
        if index > Five::POSSIBLE_COMBINATIONS {
            return CardNumber::BLANK as HandRankValue;
        }
        crate::lookups::UNIQUE_5[index]
    }

    //endregion

    //region bitwise

    #[must_use]
    pub fn and_bits(&self) -> u32 {
        self.first() & self.second() & self.third() & self.forth() & self.fifth()
    }

    #[must_use]
    pub fn is_flush(&self) -> bool {
        (self.and_bits() & CardNumber::SUIT_FILTER) != 0
    }

    /// WRITE: Unit testing uncovering how the padding method doesn't work for wheels.
    #[must_use]
    pub fn is_straight(&self) -> bool {
        let rank_bits = self.or_rank_bits();
        ((rank_bits.trailing_zeros() + rank_bits.leading_zeros()) == Five::STRAIGHT_PADDING)
            || rank_bits == Five::WHEEL_OR_BITS
    }

    #[must_use]
    pub fn is_straight_flush(&self) -> bool {
        self.is_straight() && self.is_flush()
    }

    #[must_use]
    pub fn is_wheel(&self) -> bool {
        self.or_rank_bits() == Five::WHEEL_OR_BITS
    }

    #[must_use]
    pub fn multiply_primes(&self) -> usize {
        (self.first().get_rank_prime()
            * self.second().get_rank_prime()
            * self.third().get_rank_prime()
            * self.forth().get_rank_prime()
            * self.fifth().get_rank_prime()) as usize
    }

    #[must_use]
    pub fn or_bits(&self) -> u32 {
        self.first() | self.second() | self.third() | self.forth() | self.fifth()
    }

    #[must_use]
    pub fn or_rank_bits(&self) -> u32 {
        self.or_bits() >> CardNumber::RANK_FLAG_SHIFT
    }

    //endregion bitwise
}

impl From<[CKCNumber; 5]> for Five {
    fn from(array: [CKCNumber; 5]) -> Self {
        Five(array)
    }
}

impl HandValidator for Five {
    // TODO: macro?
    fn are_unique(&self) -> bool {
        !(1..5).any(|i| self.0[i..].contains(&self.0[i - 1]))
    }

    fn first(&self) -> CKCNumber {
        self.0[0]
    }

    fn sort(&self) -> Five {
        let mut array = *self;
        array.sort_in_place();
        array
    }

    fn sort_in_place(&mut self) {
        self.0.sort_unstable();
        self.0.reverse();
    }

    fn iter(&self) -> Iter<'_, CKCNumber> {
        self.0.iter()
    }
}

impl TryFrom<&'static str> for Five {
    type Error = HandError;

    fn try_from(index: &'static str) -> Result<Self, Self::Error> {
        match Five::from_index(index) {
            None => Err(HandError::InvalidIndex),
            Some(five) => Ok(Five::from(five)),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cards__five_tests {
    use super::*;
    use crate::hand_rank::{HandRankClass, HandRankName};
    use crate::CardNumber;
    use alloc::format;
    use rstest::rstest;

    #[rustfmt::skip]
    #[rstest]
    #[case("A♠ K♠ Q♠ J♠ T♠", 1, HandRankName::StraightFlush, HandRankClass::RoyalFlush)]
    #[case("K♥ Q♥ J♥ T♥ 9♥", 2, HandRankName::StraightFlush, HandRankClass::KingHighStraightFlush)]
    #[case("Q♦ J♦ T♦ 9♦ 8♦", 3, HandRankName::StraightFlush, HandRankClass::QueenHighStraightFlush)]
    #[case("J♣ T♣ 9♣ 8♣ 7♣", 4, HandRankName::StraightFlush, HandRankClass::JackHighStraightFlush)]
    #[case("T♤ 9♤ 8♤ 7♤ 6♤", 5, HandRankName::StraightFlush, HandRankClass::TenHighStraightFlush)]
    #[case("9♡ 8♡ 7♡ 6♡ 5♡", 6, HandRankName::StraightFlush, HandRankClass::NineHighStraightFlush)]
    #[case("8♧ 7♧ 6♧ 5♧ 4♧", 7, HandRankName::StraightFlush, HandRankClass::EightHighStraightFlush)]
    #[case("7S 6S 5S 4S 3S", 8, HandRankName::StraightFlush, HandRankClass::SevenHighStraightFlush)]
    #[case("6H 5H 4H 3H 2H", 9, HandRankName::StraightFlush, HandRankClass::SixHighStraightFlush)]
    #[case("5D 4D 3D 2D AD", 10, HandRankName::StraightFlush, HandRankClass::FiveHighStraightFlush)]
    #[case("AS AH AD AC KS", 11, HandRankName::FourOfAKind, HandRankClass::FourAces)]
    #[case("AS AH AD AC QS", 12, HandRankName::FourOfAKind, HandRankClass::FourAces)]
    #[case("AS AH AD AC JS", 13, HandRankName::FourOfAKind, HandRankClass::FourAces)]
    #[case("AS AH AD AC TD", 14, HandRankName::FourOfAKind, HandRankClass::FourAces)]
    #[case("AS AH AD AC TC", 14, HandRankName::FourOfAKind, HandRankClass::FourAces)]
    #[case("AS AH AD AC 2S", 22, HandRankName::FourOfAKind, HandRankClass::FourAces)]
    #[case("KS KH KD KC AS", 23, HandRankName::FourOfAKind, HandRankClass::FourKings)]
    #[case("KS KH KD KC 2S", 34, HandRankName::FourOfAKind, HandRankClass::FourKings)]
    #[case("QS QH QD QC AS", 35, HandRankName::FourOfAKind, HandRankClass::FourQueens)]
    #[case("QS QH QD QC 2C", 46, HandRankName::FourOfAKind, HandRankClass::FourQueens)]
    #[case("JS JH JD JC AC", 47, HandRankName::FourOfAKind, HandRankClass::FourJacks)]
    #[case("JS JH JD JC 2C", 58, HandRankName::FourOfAKind, HandRankClass::FourJacks)]
    #[case("TS TH TD TC AS", 59, HandRankName::FourOfAKind, HandRankClass::FourTens)]
    #[case("TS TH TD TC 2C", 70, HandRankName::FourOfAKind, HandRankClass::FourTens)]
    #[case("9S 9H 9D 9C AH", 71, HandRankName::FourOfAKind, HandRankClass::FourNines)]
    #[case("9S 9H 9D 9C 2D", 82, HandRankName::FourOfAKind, HandRankClass::FourNines)]
    #[case("8S 8H 8D 8C AD", 83, HandRankName::FourOfAKind, HandRankClass::FourEights)]
    #[case("8S 8H 8D 8C 2D", 94, HandRankName::FourOfAKind, HandRankClass::FourEights)]
    #[case("7S 7H 7D 7C AD", 95, HandRankName::FourOfAKind, HandRankClass::FourSevens)]
    #[case("7S 7H 7D 7C 2D", 106, HandRankName::FourOfAKind, HandRankClass::FourSevens)]
    #[case("6S 6H 6D 6C AD", 107, HandRankName::FourOfAKind, HandRankClass::FourSixes)]
    #[case("6S 6H 6D 6C 2D", 118, HandRankName::FourOfAKind, HandRankClass::FourSixes)]
    #[case("5S 5H 5D 5C AD", 119, HandRankName::FourOfAKind, HandRankClass::FourFives)]
    #[case("5S 5H 5D 5C 2D", 130, HandRankName::FourOfAKind, HandRankClass::FourFives)]
    #[case("4S 4H 4D 4C AD", 131, HandRankName::FourOfAKind, HandRankClass::FourFours)]
    #[case("4S 4H 4D 4C 2D", 142, HandRankName::FourOfAKind, HandRankClass::FourFours)]
    #[case("3S 3H 3D 3C AD", 143, HandRankName::FourOfAKind, HandRankClass::FourTreys)]
    #[case("3S 3H 3D 3C 2D", 154, HandRankName::FourOfAKind, HandRankClass::FourTreys)]
    #[case("2S 2H 2D 2C AC", 155, HandRankName::FourOfAKind, HandRankClass::FourDeuces)]
    #[case("2S 2H 2D 2C 3D", 166, HandRankName::FourOfAKind, HandRankClass::FourDeuces)]
    #[case("AS AH AD KC KD", 167, HandRankName::FullHouse, HandRankClass::AcesOverKings)]
    #[case("AS AH AD QC QD", 168, HandRankName::FullHouse, HandRankClass::AcesOverQueens)]
    #[case("AS AH AD JD JC", 169, HandRankName::FullHouse, HandRankClass::AcesOverJacks)]
    #[case("AS AH AD TD TC", 170, HandRankName::FullHouse, HandRankClass::AcesOverTens)]
    #[case("AS AH AD 9S 9D", 171, HandRankName::FullHouse, HandRankClass::AcesOverNines)]
    #[case("AS AH AD 8S 8D", 172, HandRankName::FullHouse, HandRankClass::AcesOverEights)]
    #[case("AS AH AD 7S 7D", 173, HandRankName::FullHouse, HandRankClass::AcesOverSevens)]
    #[case("AS AH AD 6S 6D", 174, HandRankName::FullHouse, HandRankClass::AcesOverSixes)]
    #[case("AS AH AD 5S 5D", 175, HandRankName::FullHouse, HandRankClass::AcesOverFives)]
    #[case("AS AH AD 4S 4D", 176, HandRankName::FullHouse, HandRankClass::AcesOverFours)]
    #[case("AS AH AD 3D 3c", 177, HandRankName::FullHouse, HandRankClass::AcesOverTreys)]
    #[case("AS AH AD 2H 2D", 178, HandRankName::FullHouse, HandRankClass::AcesOverDeuces)]
    #[case("AS AH KD KH KC", 179, HandRankName::FullHouse, HandRankClass::KingsOverAces)]
    #[case("QS KH QD KC KD", 180, HandRankName::FullHouse, HandRankClass::KingsOverQueens)]
    #[case("KS KH KD JH JD", 181, HandRankName::FullHouse, HandRankClass::KingsOverJacks)]
    #[case("KS KH KD TH TD", 182, HandRankName::FullHouse, HandRankClass::KingsOverTens)]
    #[case("KS KH KD 9H 9D", 183, HandRankName::FullHouse, HandRankClass::KingsOverNines)]
    #[case("KS KH 8D 8H KD", 184, HandRankName::FullHouse, HandRankClass::KingsOverEights)]
    #[case("KS KH KD 7H 7D", 185, HandRankName::FullHouse, HandRankClass::KingsOverSevens)]
    #[case("KS KH KD 6H 6D", 186, HandRankName::FullHouse, HandRankClass::KingsOverSixes)]
    #[case("KS KH KD 5H 5D", 187, HandRankName::FullHouse, HandRankClass::KingsOverFives)]
    #[case("4S 4H KD KH KC", 188, HandRankName::FullHouse, HandRankClass::KingsOverFours)]
    #[case("3S KH KD 3H KC", 189, HandRankName::FullHouse, HandRankClass::KingsOverTreys)]
    #[case("KS KH KD 2H 2D", 190, HandRankName::FullHouse, HandRankClass::KingsOverDeuces)]
    #[case("QS QH QD AH AD", 191, HandRankName::FullHouse, HandRankClass::QueensOverAces)]
    #[case("QS QH QD KH KD", 192, HandRankName::FullHouse, HandRankClass::QueensOverKings)]
    #[case("QS QH QD JH JD", 193, HandRankName::FullHouse, HandRankClass::QueensOverJacks)]
    #[case("QS QH QD TH TD", 194, HandRankName::FullHouse, HandRankClass::QueensOverTens)]
    #[case("QS QH QD 9H 9D", 195, HandRankName::FullHouse, HandRankClass::QueensOverNines)]
    #[case("QS QH QD 8H 8D", 196, HandRankName::FullHouse, HandRankClass::QueensOverEights)]
    #[case("QS QH QD 7H 7D", 197, HandRankName::FullHouse, HandRankClass::QueensOverSevens)]
    #[case("QS QH QD 6H 6D", 198, HandRankName::FullHouse, HandRankClass::QueensOverSixes)]
    #[case("QS QH QD 5H 5D", 199, HandRankName::FullHouse, HandRankClass::QueensOverFives)]
    #[case("QS QH QD 4S 4D", 200, HandRankName::FullHouse, HandRankClass::QueensOverFours)]
    #[case("QS QH QD 3H 3D", 201, HandRankName::FullHouse, HandRankClass::QueensOverTreys)]
    #[case("QS QH QD 2H 2D", 202, HandRankName::FullHouse, HandRankClass::QueensOverDeuces)]
    #[case("JS JH JD AH AD", 203, HandRankName::FullHouse, HandRankClass::JacksOverAces)]
    #[case("JS JH JD KH KD", 204, HandRankName::FullHouse, HandRankClass::JacksOverKings)]
    #[case("JS JH JD QH QD", 205, HandRankName::FullHouse, HandRankClass::JacksOverQueens)]
    #[case("JS JH JD TH TD", 206, HandRankName::FullHouse, HandRankClass::JacksOverTens)]
    #[case("JS JH JD 9H 9D", 207, HandRankName::FullHouse, HandRankClass::JacksOverNines)]
    #[case("JS JH JD 8H 8D", 208, HandRankName::FullHouse, HandRankClass::JacksOverEights)]
    #[case("JS JH JD 7H 7D", 209, HandRankName::FullHouse, HandRankClass::JacksOverSevens)]
    #[case("JS JH JD 6H 6D", 210, HandRankName::FullHouse, HandRankClass::JacksOverSixes)]
    #[case("JS JH JD 5H 5D", 211, HandRankName::FullHouse, HandRankClass::JacksOverFives)]
    #[case("JS JH JD 4H 4D", 212, HandRankName::FullHouse, HandRankClass::JacksOverFours)]
    #[case("JS JH JD 3H 3D", 213, HandRankName::FullHouse, HandRankClass::JacksOverTreys)]
    #[case("JS JH JD 2H 2D", 214, HandRankName::FullHouse, HandRankClass::JacksOverDeuces)]
    #[case("TS TH TD AH AD", 215, HandRankName::FullHouse, HandRankClass::TensOverAces)]
    #[case("TS TH TD KH KD", 216, HandRankName::FullHouse, HandRankClass::TensOverKings)]
    #[case("TS TH TD QH QD", 217, HandRankName::FullHouse, HandRankClass::TensOverQueens)]
    #[case("TS TH TD JH JD", 218, HandRankName::FullHouse, HandRankClass::TensOverJacks)]
    #[case("TS TH TD 9H 9D", 219, HandRankName::FullHouse, HandRankClass::TensOverNines)]
    #[case("TS TH TD 8H 8D", 220, HandRankName::FullHouse, HandRankClass::TensOverEights)]
    #[case("TS TH TD 7H 7D", 221, HandRankName::FullHouse, HandRankClass::TensOverSevens)]
    #[case("TS TH TD 6S 6D", 222, HandRankName::FullHouse, HandRankClass::TensOverSixes)]
    #[case("TS TH TD 5H 5D", 223, HandRankName::FullHouse, HandRankClass::TensOverFives)]
    #[case("TS TH TD 4H 4D", 224, HandRankName::FullHouse, HandRankClass::TensOverFours)]
    #[case("TS TH TD 3H 3D", 225, HandRankName::FullHouse, HandRankClass::TensOverTreys)]
    #[case("TS TH TD 2H 2D", 226, HandRankName::FullHouse, HandRankClass::TensOverDeuces)]
    #[case("9S 9H 9D AH AD", 227, HandRankName::FullHouse, HandRankClass::NinesOverAces)]
    #[case("9S 9H 9D KH KD", 228, HandRankName::FullHouse, HandRankClass::NinesOverKings)]
    #[case("9S 9H 9D QH QD", 229, HandRankName::FullHouse, HandRankClass::NinesOverQueens)]
    #[case("9S 9H 9D JH JD", 230, HandRankName::FullHouse, HandRankClass::NinesOverJacks)]
    #[case("9S 9H 9D TH TD", 231, HandRankName::FullHouse, HandRankClass::NinesOverTens)]
    #[case("9S 9H 9D 8H 8D", 232, HandRankName::FullHouse, HandRankClass::NinesOverEights)]
    #[case("9S 9H 9D 7H 7D", 233, HandRankName::FullHouse, HandRankClass::NinesOverSevens)]
    #[case("9S 9H 9D 6S 6D", 234, HandRankName::FullHouse, HandRankClass::NinesOverSixes)]
    #[case("9S 9H 9D 5H 5D", 235, HandRankName::FullHouse, HandRankClass::NinesOverFives)]
    #[case("9S 9H 9D 4H 4D", 236, HandRankName::FullHouse, HandRankClass::NinesOverFours)]
    #[case("9S 9H 9D 3H 3D", 237, HandRankName::FullHouse, HandRankClass::NinesOverTreys)]
    #[case("9S 9H 9D 2H 2D", 238, HandRankName::FullHouse, HandRankClass::NinesOverDeuces)]
    #[case("8S 8H 8D AH AD", 239, HandRankName::FullHouse, HandRankClass::EightsOverAces)]
    #[case("8S 8H 8D KH KD", 240, HandRankName::FullHouse, HandRankClass::EightsOverKings)]
    #[case("8S 8H 8D QH QD", 241, HandRankName::FullHouse, HandRankClass::EightsOverQueens)]
    #[case("8S 8H 8D JH JD", 242, HandRankName::FullHouse, HandRankClass::EightsOverJacks)]
    #[case("8S 8H 8D TH TD", 243, HandRankName::FullHouse, HandRankClass::EightsOverTens)]
    #[case("8S 8H 8D 9H 9D", 244, HandRankName::FullHouse, HandRankClass::EightsOverNines)]
    #[case("8S 8H 8D 7H 7D", 245, HandRankName::FullHouse, HandRankClass::EightsOverSevens)]
    #[case("8S 8H 8D 6S 6D", 246, HandRankName::FullHouse, HandRankClass::EightsOverSixes)]
    #[case("8S 8H 8D 5H 5D", 247, HandRankName::FullHouse, HandRankClass::EightsOverFives)]
    #[case("8S 8H 8D 4H 4D", 248, HandRankName::FullHouse, HandRankClass::EightsOverFours)]
    #[case("8S 8H 8D 3H 3D", 249, HandRankName::FullHouse, HandRankClass::EightsOverTreys)]
    #[case("8S 8H 8D 2H 2D", 250, HandRankName::FullHouse, HandRankClass::EightsOverDeuces)]
    #[case("7S 7H 7D AH AD", 251, HandRankName::FullHouse, HandRankClass::SevensOverAces)]
    #[case("7S 7H 7D KH KD", 252, HandRankName::FullHouse, HandRankClass::SevensOverKings)]
    #[case("7S 7H 7D QH QD", 253, HandRankName::FullHouse, HandRankClass::SevensOverQueens)]
    #[case("7S 7H 7D JH JD", 254, HandRankName::FullHouse, HandRankClass::SevensOverJacks)]
    #[case("7S 7H 7D TH TD", 255, HandRankName::FullHouse, HandRankClass::SevensOverTens)]
    #[case("7S 7H 7D 9H 9D", 256, HandRankName::FullHouse, HandRankClass::SevensOverNines)]
    #[case("7S 7H 7D 8H 8D", 257, HandRankName::FullHouse, HandRankClass::SevensOverEights)]
    #[case("7S 7H 7D 6S 6D", 258, HandRankName::FullHouse, HandRankClass::SevensOverSixes)]
    #[case("7S 7H 7D 5H 5D", 259, HandRankName::FullHouse, HandRankClass::SevensOverFives)]
    #[case("7S 7H 7D 4H 4D", 260, HandRankName::FullHouse, HandRankClass::SevensOverFours)]
    #[case("7S 7H 7D 3H 3D", 261, HandRankName::FullHouse, HandRankClass::SevensOverTreys)]
    #[case("7S 7H 7D 2H 2D", 262, HandRankName::FullHouse, HandRankClass::SevensOverDeuces)]
    #[case("6S 6H 6D AH AD", 263, HandRankName::FullHouse, HandRankClass::SixesOverAces)]
    #[case("6S 6H 6D KH KD", 264, HandRankName::FullHouse, HandRankClass::SixesOverKings)]
    #[case("6S 6H 6D QH QD", 265, HandRankName::FullHouse, HandRankClass::SixesOverQueens)]
    #[case("6S 6H 6D JH JD", 266, HandRankName::FullHouse, HandRankClass::SixesOverJacks)]
    #[case("6S 6H 6D TH TD", 267, HandRankName::FullHouse, HandRankClass::SixesOverTens)]
    #[case("6S 6H 6D 9H 9D", 268, HandRankName::FullHouse, HandRankClass::SixesOverNines)]
    #[case("6S 6H 6D 8H 8D", 269, HandRankName::FullHouse, HandRankClass::SixesOverEights)]
    #[case("6S 6H 6D 7S 7D", 270, HandRankName::FullHouse, HandRankClass::SixesOverSevens)]
    #[case("6S 6H 6D 5H 5D", 271, HandRankName::FullHouse, HandRankClass::SixesOverFives)]
    #[case("6S 6H 6D 4H 4D", 272, HandRankName::FullHouse, HandRankClass::SixesOverFours)]
    #[case("6S 6H 6D 3H 3D", 273, HandRankName::FullHouse, HandRankClass::SixesOverTreys)]
    #[case("6S 6H 6D 2H 2D", 274, HandRankName::FullHouse, HandRankClass::SixesOverDeuces)]
    #[case("5S 5H 5D AH AD", 275, HandRankName::FullHouse, HandRankClass::FivesOverAces)]
    #[case("5S 5H 5D KH KD", 276, HandRankName::FullHouse, HandRankClass::FivesOverKings)]
    #[case("5S 5H 5D QH QD", 277, HandRankName::FullHouse, HandRankClass::FivesOverQueens)]
    #[case("5S 5H 5D JH JD", 278, HandRankName::FullHouse, HandRankClass::FivesOverJacks)]
    #[case("5S 5H 5D TH TD", 279, HandRankName::FullHouse, HandRankClass::FivesOverTens)]
    #[case("5S 5H 5D 9H 9D", 280, HandRankName::FullHouse, HandRankClass::FivesOverNines)]
    #[case("5S 5H 5D 8H 8D", 281, HandRankName::FullHouse, HandRankClass::FivesOverEights)]
    #[case("5S 5H 5D 7S 7D", 282, HandRankName::FullHouse, HandRankClass::FivesOverSevens)]
    #[case("5S 5H 5D 6H 6D", 283, HandRankName::FullHouse, HandRankClass::FivesOverSixes)]
    #[case("5S 5H 5D 4H 4D", 284, HandRankName::FullHouse, HandRankClass::FivesOverFours)]
    #[case("5S 5H 5D 3H 3D", 285, HandRankName::FullHouse, HandRankClass::FivesOverTreys)]
    #[case("5S 5H 5D 2H 2D", 286, HandRankName::FullHouse, HandRankClass::FivesOverDeuces)]
    #[case("4S 4H 4D AH AD", 287, HandRankName::FullHouse, HandRankClass::FoursOverAces)]
    #[case("4S 4H 4D KH KD", 288, HandRankName::FullHouse, HandRankClass::FoursOverKings)]
    #[case("4S 4H 4D QH QD", 289, HandRankName::FullHouse, HandRankClass::FoursOverQueens)]
    #[case("4S 4H 4D JH JD", 290, HandRankName::FullHouse, HandRankClass::FoursOverJacks)]
    #[case("4S 4H 4D TH TD", 291, HandRankName::FullHouse, HandRankClass::FoursOverTens)]
    #[case("4S 4H 4D 9H 9D", 292, HandRankName::FullHouse, HandRankClass::FoursOverNines)]
    #[case("4S 4H 4D 8H 8D", 293, HandRankName::FullHouse, HandRankClass::FoursOverEights)]
    #[case("4S 4H 4D 7S 7D", 294, HandRankName::FullHouse, HandRankClass::FoursOverSevens)]
    #[case("4S 4H 4D 6H 6D", 295, HandRankName::FullHouse, HandRankClass::FoursOverSixes)]
    #[case("4S 4H 4D 5H 5D", 296, HandRankName::FullHouse, HandRankClass::FoursOverFives)]
    #[case("4S 4H 4D 3H 3D", 297, HandRankName::FullHouse, HandRankClass::FoursOverTreys)]
    #[case("4S 4H 4D 2H 2D", 298, HandRankName::FullHouse, HandRankClass::FoursOverDeuces)]
    #[case("3S 3H 3D AH AD", 299, HandRankName::FullHouse, HandRankClass::TreysOverAces)]
    #[case("3S 3H 3D KH KD", 300, HandRankName::FullHouse, HandRankClass::TreysOverKings)]
    #[case("3S 3H 3D QH QD", 301, HandRankName::FullHouse, HandRankClass::TreysOverQueens)]
    #[case("3S 3H 3D JH JD", 302, HandRankName::FullHouse, HandRankClass::TreysOverJacks)]
    #[case("3S 3H 3D TH TD", 303, HandRankName::FullHouse, HandRankClass::TreysOverTens)]
    #[case("3S 3H 3D 9H 9D", 304, HandRankName::FullHouse, HandRankClass::TreysOverNines)]
    #[case("3S 3H 3D 8H 8D", 305, HandRankName::FullHouse, HandRankClass::TreysOverEights)]
    #[case("3S 3H 3D 7S 7D", 306, HandRankName::FullHouse, HandRankClass::TreysOverSevens)]
    #[case("3S 3H 3D 6H 6D", 307, HandRankName::FullHouse, HandRankClass::TreysOverSixes)]
    #[case("3S 3H 3D 5H 5D", 308, HandRankName::FullHouse, HandRankClass::TreysOverFives)]
    #[case("3S 3H 3D 4H 4D", 309, HandRankName::FullHouse, HandRankClass::TreysOverFours)]
    #[case("3S 3H 3D 2H 2D", 310, HandRankName::FullHouse, HandRankClass::TreysOverDeuces)]
    #[case("2S 2H 2D AH AD", 311, HandRankName::FullHouse, HandRankClass::DeucesOverAces)]
    #[case("2S 2H 2D KH KD", 312, HandRankName::FullHouse, HandRankClass::DeucesOverKings)]
    #[case("2S 2H 2D QH QD", 313, HandRankName::FullHouse, HandRankClass::DeucesOverQueens)]
    #[case("2S 2H 2D JH JD", 314, HandRankName::FullHouse, HandRankClass::DeucesOverJacks)]
    #[case("2S 2H 2D TH TD", 315, HandRankName::FullHouse, HandRankClass::DeucesOverTens)]
    #[case("2S 2H 2D 9H 9D", 316, HandRankName::FullHouse, HandRankClass::DeucesOverNines)]
    #[case("2S 2H 2D 8H 8D", 317, HandRankName::FullHouse, HandRankClass::DeucesOverEights)]
    #[case("2S 2H 2D 7S 7D", 318, HandRankName::FullHouse, HandRankClass::DeucesOverSevens)]
    #[case("2S 2H 2D 6H 6D", 319, HandRankName::FullHouse, HandRankClass::DeucesOverSixes)]
    #[case("2S 2H 2D 5H 5D", 320, HandRankName::FullHouse, HandRankClass::DeucesOverFives)]
    #[case("2S 2H 2D 4H 4D", 321, HandRankName::FullHouse, HandRankClass::DeucesOverFours)]
    #[case("2S 2H 2D 3H 3D", 322, HandRankName::FullHouse, HandRankClass::DeucesOverTreys)]
    #[case("AS KS QS JS 9S", 323, HandRankName::Flush, HandRankClass::AceHighFlush)]
    #[case("AS 6S 4S 3S 2S", 815, HandRankName::Flush, HandRankClass::AceHighFlush)]
    #[case("KH QH JH TH 8H", 816, HandRankName::Flush, HandRankClass::KingHighFlush)]
    #[case("KC 5C 4C 3C 2C", 1144, HandRankName::Flush, HandRankClass::KingHighFlush)]
    #[case("QH JH TH 9H 7H", 1145, HandRankName::Flush, HandRankClass::QueenHighFlush)]
    #[case("QC 5C 4C 3C 2C", 1353, HandRankName::Flush, HandRankClass::QueenHighFlush)]
    #[case("JH TH 9H 8H 6H", 1354, HandRankName::Flush, HandRankClass::JackHighFlush)]
    #[case("JC 5C 4C 3C 2C", 1478, HandRankName::Flush, HandRankClass::JackHighFlush)]
    #[case("TH 9H 8H 7H 5H", 1479, HandRankName::Flush, HandRankClass::TenHighFlush)]
    #[case("TC 5C 4C 3C 2C", 1547, HandRankName::Flush, HandRankClass::TenHighFlush)]
    #[case("9H 8H 7H 6H 4H", 1548, HandRankName::Flush, HandRankClass::NineHighFlush)]
    #[case("9C 5C 4C 3C 2C", 1581, HandRankName::Flush, HandRankClass::NineHighFlush)]
    #[case("8H 7H 6H 5H 3H", 1582, HandRankName::Flush, HandRankClass::EightHighFlush)]
    #[case("8C 5C 4C 3C 2C", 1595, HandRankName::Flush, HandRankClass::EightHighFlush)]
    #[case("7H 6H 5H 4H 2H", 1596, HandRankName::Flush, HandRankClass::SevenHighFlush)]
    #[case("7C 5C 4C 3C 2C", 1599, HandRankName::Flush, HandRankClass::SevenHighFlush)]
    #[case("A♠ K♠ Q♥ J♠ T♠", 1600, HandRankName::Straight, HandRankClass::AceHighStraight)]
    #[case("K♥ Q♥ J♠ T♥ 9♥", 1601, HandRankName::Straight, HandRankClass::KingHighStraight)]
    #[case("Q♦ J♠ T♦ 9♦ 8♦", 1602, HandRankName::Straight, HandRankClass::QueenHighStraight)]
    #[case("J♣ T♣ 9♣ 8♠ 7♣", 1603, HandRankName::Straight, HandRankClass::JackHighStraight)]
    #[case("T♤ 9♤ 8♡ 7♤ 6♤", 1604, HandRankName::Straight, HandRankClass::TenHighStraight)]
    #[case("9♡ 8♤ 7♡ 6♡ 5♡", 1605, HandRankName::Straight, HandRankClass::NineHighStraight)]
    #[case("8♧ 7♧ 6♡ 5♧ 4♧", 1606, HandRankName::Straight, HandRankClass::EightHighStraight)]
    #[case("7S 6♥ 5S 4S 3S", 1607, HandRankName::Straight, HandRankClass::SevenHighStraight)]
    #[case("6H 5S 4H 3H 2H", 1608, HandRankName::Straight, HandRankClass::SixHighStraight)]
    #[case("5D 4D 3♥ 2D AD", 1609, HandRankName::Straight, HandRankClass::FiveHighStraight)]
    #[case("AS AD AC KS QD", 1610, HandRankName::ThreeOfAKind, HandRankClass::ThreeAces)]
    #[case("AS AD AC 3S 2D", 1675, HandRankName::ThreeOfAKind, HandRankClass::ThreeAces)]
    #[case("KS KH KC AD QD", 1676, HandRankName::ThreeOfAKind, HandRankClass::ThreeKings)]
    #[case("KS KH KC 3D 2D", 1741, HandRankName::ThreeOfAKind, HandRankClass::ThreeKings)]
    #[case("QH QD QC AD KS", 1742, HandRankName::ThreeOfAKind, HandRankClass::ThreeQueens)]
    #[case("QH QD QC 3D 2S", 1807, HandRankName::ThreeOfAKind, HandRankClass::ThreeQueens)]
    #[case("JS JD JC AD KS", 1808, HandRankName::ThreeOfAKind, HandRankClass::ThreeJacks)]
    #[case("JS JD JC 3D 2S", 1873, HandRankName::ThreeOfAKind, HandRankClass::ThreeJacks)]
    #[case("TH TD TC AD KD", 1874, HandRankName::ThreeOfAKind, HandRankClass::ThreeTens)]
    #[case("TH TD TC 3D 2D", 1939, HandRankName::ThreeOfAKind, HandRankClass::ThreeTens)]
    #[case("9H 9D 9C AD KD", 1940, HandRankName::ThreeOfAKind, HandRankClass::ThreeNines)]
    #[case("9H 9D 9C 3D 2D", 2005, HandRankName::ThreeOfAKind, HandRankClass::ThreeNines)]
    #[case("8H 8D 8C AD KD", 2006, HandRankName::ThreeOfAKind, HandRankClass::ThreeEights)]
    #[case("8H 8D 8C 3D 2D", 2071, HandRankName::ThreeOfAKind, HandRankClass::ThreeEights)]
    #[case("7H 7D 7C AS KD", 2072, HandRankName::ThreeOfAKind, HandRankClass::ThreeSevens)]
    #[case("7H 7D 7C 3S 2D", 2137, HandRankName::ThreeOfAKind, HandRankClass::ThreeSevens)]
    #[case("6H 6D 6C AS KD", 2138, HandRankName::ThreeOfAKind, HandRankClass::ThreeSixes)]
    #[case("6H 6D 6C 3S 2D", 2203, HandRankName::ThreeOfAKind, HandRankClass::ThreeSixes)]
    #[case("5S 5H 5C AD KD", 2204, HandRankName::ThreeOfAKind, HandRankClass::ThreeFives)]
    #[case("5S 5H 5C 3D 2D", 2269, HandRankName::ThreeOfAKind, HandRankClass::ThreeFives)]
    #[case("4S 4H 4C AD KD", 2270, HandRankName::ThreeOfAKind, HandRankClass::ThreeFours)]
    #[case("4S 4H 4C 3D 2D", 2335, HandRankName::ThreeOfAKind, HandRankClass::ThreeFours)]
    #[case("3S 3H 3C AD KD", 2336, HandRankName::ThreeOfAKind, HandRankClass::ThreeTreys)]
    #[case("3S 3D 3C 4D 2D", 2401, HandRankName::ThreeOfAKind, HandRankClass::ThreeTreys)]
    #[case("2S 2H 2C AD KD", 2402, HandRankName::ThreeOfAKind, HandRankClass::ThreeDeuces)]
    #[case("2S 2H 2C 4S 3C", 2467, HandRankName::ThreeOfAKind, HandRankClass::ThreeDeuces)]
    #[case("AS AD KS KH Q♥", 2468, HandRankName::TwoPair, HandRankClass::AcesAndKings)]
    #[case("AS AD KS KH 2♥", 2478, HandRankName::TwoPair, HandRankClass::AcesAndKings)]
    #[case("AS AD QS QH K♥", 2479, HandRankName::TwoPair, HandRankClass::AcesAndQueens)]
    #[case("AS AD QS QH 2♥", 2489, HandRankName::TwoPair, HandRankClass::AcesAndQueens)]
    #[case("AS AD JS JH K♥", 2490, HandRankName::TwoPair, HandRankClass::AcesAndJacks)]
    #[case("AS AD JS JH 2♥", 2500, HandRankName::TwoPair, HandRankClass::AcesAndJacks)]
    #[case("AS AD TS TH K♥", 2501, HandRankName::TwoPair, HandRankClass::AcesAndTens)]
    #[case("AS AD TS TH 2♥", 2511, HandRankName::TwoPair, HandRankClass::AcesAndTens)]
    #[case("AS AD 9S 9H K♥", 2512, HandRankName::TwoPair, HandRankClass::AcesAndNines)]
    #[case("AS AD 9S 9H 2♥", 2522, HandRankName::TwoPair, HandRankClass::AcesAndNines)]
    #[case("AS AD 8S 8H K♥", 2523, HandRankName::TwoPair, HandRankClass::AcesAndEights)]
    #[case("AS AD 8S 8H 2♥", 2533, HandRankName::TwoPair, HandRankClass::AcesAndEights)]
    #[case("AS AD 7S 7H K♥", 2534, HandRankName::TwoPair, HandRankClass::AcesAndSevens)]
    #[case("AS AD 7S 7H 2♥", 2544, HandRankName::TwoPair, HandRankClass::AcesAndSevens)]
    #[case("AS AD 6S 6H K♥", 2545, HandRankName::TwoPair, HandRankClass::AcesAndSixes)]
    #[case("AS AD 6S 6H 2♥", 2555, HandRankName::TwoPair, HandRankClass::AcesAndSixes)]
    #[case("AS AD 5S 5H K♥", 2556, HandRankName::TwoPair, HandRankClass::AcesAndFives)]
    #[case("AS AD 5S 5H 2♥", 2566, HandRankName::TwoPair, HandRankClass::AcesAndFives)]
    #[case("AS AD 4S 4H K♥", 2567, HandRankName::TwoPair, HandRankClass::AcesAndFours)]
    #[case("AS AD 4S 4H 2♥", 2577, HandRankName::TwoPair, HandRankClass::AcesAndFours)]
    #[case("AS AD 3S 3H K♥", 2578, HandRankName::TwoPair, HandRankClass::AcesAndTreys)]
    #[case("AS AD 3S 3H 2♥", 2588, HandRankName::TwoPair, HandRankClass::AcesAndTreys)]
    #[case("AS AD 2S 2H K♥", 2589, HandRankName::TwoPair, HandRankClass::AcesAndDeuces)]
    #[case("AS AD 2S 2H 3♥", 2599, HandRankName::TwoPair, HandRankClass::AcesAndDeuces)]
    #[case("KS KH Q♥ QD AC", 2600, HandRankName::TwoPair, HandRankClass::KingsAndQueens)]
    #[case("KS KH Q♥ QD 2♥", 2610, HandRankName::TwoPair, HandRankClass::KingsAndQueens)]
    #[case("KS KH J♥ JD AC", 2611, HandRankName::TwoPair, HandRankClass::KingsAndJacks)]
    #[case("KS KH J♥ JD 2♥", 2621, HandRankName::TwoPair, HandRankClass::KingsAndJacks)]
    #[case("KS KH T♥ TD AC", 2622, HandRankName::TwoPair, HandRankClass::KingsAndTens)]
    #[case("KS KH T♥ TD 2♥", 2632, HandRankName::TwoPair, HandRankClass::KingsAndTens)]
    #[case("KS KH 9♥ 9D AC", 2633, HandRankName::TwoPair, HandRankClass::KingsAndNines)]
    #[case("KS KH 9♥ 9D 2♥", 2643, HandRankName::TwoPair, HandRankClass::KingsAndNines)]
    #[case("KS KH 8♥ 8D AC", 2644, HandRankName::TwoPair, HandRankClass::KingsAndEights)]
    #[case("KS KH 8♥ 8D 2♥", 2654, HandRankName::TwoPair, HandRankClass::KingsAndEights)]
    #[case("KS KH 7♥ 7D AC", 2655, HandRankName::TwoPair, HandRankClass::KingsAndSevens)]
    #[case("KS KH 7♥ 7D 2♥", 2665, HandRankName::TwoPair, HandRankClass::KingsAndSevens)]
    #[case("KS KH 6♥ 6D AC", 2666, HandRankName::TwoPair, HandRankClass::KingsAndSixes)]
    #[case("KS KH 6♥ 6D 2♥", 2676, HandRankName::TwoPair, HandRankClass::KingsAndSixes)]
    #[case("KS KH 5♥ 5D AC", 2677, HandRankName::TwoPair, HandRankClass::KingsAndFives)]
    #[case("KS KH 5♥ 5D 2♥", 2687, HandRankName::TwoPair, HandRankClass::KingsAndFives)]
    #[case("KS KH 4♥ 4D AC", 2688, HandRankName::TwoPair, HandRankClass::KingsAndFours)]
    #[case("KS KH 4♥ 4D 2♥", 2698, HandRankName::TwoPair, HandRankClass::KingsAndFours)]
    #[case("KS KH 3♥ 3D AC", 2699, HandRankName::TwoPair, HandRankClass::KingsAndTreys)]
    #[case("KS KH 3♥ 3D 2♥", 2709, HandRankName::TwoPair, HandRankClass::KingsAndTreys)]
    #[case("KS KH 2♥ 2D AC", 2710, HandRankName::TwoPair, HandRankClass::KingsAndDeuces)]
    #[case("KS KH 2♥ 2D 3♥", 2720, HandRankName::TwoPair, HandRankClass::KingsAndDeuces)]
    #[case("QS QH J♥ JD AC", 2721, HandRankName::TwoPair, HandRankClass::QueensAndJacks)]
    #[case("QS QH J♥ JD 2♥", 2731, HandRankName::TwoPair, HandRankClass::QueensAndJacks)]
    #[case("QS QH T♥ TD AC", 2732, HandRankName::TwoPair, HandRankClass::QueensAndTens)]
    #[case("QS QH T♥ TD 2♥", 2742, HandRankName::TwoPair, HandRankClass::QueensAndTens)]
    #[case("QS QH 9♥ 9D AC", 2743, HandRankName::TwoPair, HandRankClass::QueensAndNines)]
    #[case("QS QH 9♥ 9D 2♥", 2753, HandRankName::TwoPair, HandRankClass::QueensAndNines)]
    #[case("QS QH 8♥ 8D AC", 2754, HandRankName::TwoPair, HandRankClass::QueensAndEights)]
    #[case("QS QH 8♥ 8D 2♥", 2764, HandRankName::TwoPair, HandRankClass::QueensAndEights)]
    #[case("QS QH 7♥ 7D AC", 2765, HandRankName::TwoPair, HandRankClass::QueensAndSevens)]
    #[case("QS QH 7♥ 7D 2♥", 2775, HandRankName::TwoPair, HandRankClass::QueensAndSevens)]
    #[case("QS QH 6♥ 6D AC", 2776, HandRankName::TwoPair, HandRankClass::QueensAndSixes)]
    #[case("QS QH 6♥ 6D 2♥", 2786, HandRankName::TwoPair, HandRankClass::QueensAndSixes)]
    #[case("QS QH 5♥ 5D AC", 2787, HandRankName::TwoPair, HandRankClass::QueensAndFives)]
    #[case("QS QH 5♥ 5D 2♥", 2797, HandRankName::TwoPair, HandRankClass::QueensAndFives)]
    #[case("QS QH 4♥ 4D AC", 2798, HandRankName::TwoPair, HandRankClass::QueensAndFours)]
    #[case("QS QH 4♥ 4D 2♥", 2808, HandRankName::TwoPair, HandRankClass::QueensAndFours)]
    #[case("QS QH 3♥ 3D AC", 2809, HandRankName::TwoPair, HandRankClass::QueensAndTreys)]
    #[case("QS QH 3♥ 3D 2♥", 2819, HandRankName::TwoPair, HandRankClass::QueensAndTreys)]
    #[case("QS QH 2♥ 2D AC", 2820, HandRankName::TwoPair, HandRankClass::QueensAndDeuces)]
    #[case("QS QH 2♥ 2D 3♥", 2830, HandRankName::TwoPair, HandRankClass::QueensAndDeuces)]
    #[case("JS JH T♥ TD AC", 2831, HandRankName::TwoPair, HandRankClass::JacksAndTens)]
    #[case("JS JH T♥ TD 2♥", 2841, HandRankName::TwoPair, HandRankClass::JacksAndTens)]
    #[case("JS JH 9♥ 9D AC", 2842, HandRankName::TwoPair, HandRankClass::JacksAndNines)]
    #[case("JS JH 9♥ 9D 2♥", 2852, HandRankName::TwoPair, HandRankClass::JacksAndNines)]
    #[case("JS JH 8♥ 8D AC", 2853, HandRankName::TwoPair, HandRankClass::JacksAndEights)]
    #[case("JS JH 8♥ 8D 2♥", 2863, HandRankName::TwoPair, HandRankClass::JacksAndEights)]
    #[case("JS JH 7♥ 7D AC", 2864, HandRankName::TwoPair, HandRankClass::JacksAndSevens)]
    #[case("JS JH 7♥ 7D 2♥", 2874, HandRankName::TwoPair, HandRankClass::JacksAndSevens)]
    #[case("JS JH 6♥ 6D AC", 2875, HandRankName::TwoPair, HandRankClass::JacksAndSixes)]
    #[case("JS JH 6♥ 6D 2♥", 2885, HandRankName::TwoPair, HandRankClass::JacksAndSixes)]
    #[case("JS JH 5♥ 5D AC", 2886, HandRankName::TwoPair, HandRankClass::JacksAndFives)]
    #[case("JS JH 5♥ 5D 2♥", 2896, HandRankName::TwoPair, HandRankClass::JacksAndFives)]
    #[case("JS JH 4♥ 4D AC", 2897, HandRankName::TwoPair, HandRankClass::JacksAndFours)]
    #[case("JS JH 4♥ 4D 2♥", 2907, HandRankName::TwoPair, HandRankClass::JacksAndFours)]
    #[case("JS JH 3♥ 3D AC", 2908, HandRankName::TwoPair, HandRankClass::JacksAndTreys)]
    #[case("JS JH 3♥ 3D 2♥", 2918, HandRankName::TwoPair, HandRankClass::JacksAndTreys)]
    #[case("JS JH 2♥ 2D AC", 2919, HandRankName::TwoPair, HandRankClass::JacksAndDeuces)]
    #[case("JS JH 2♥ 2D 3♥", 2929, HandRankName::TwoPair, HandRankClass::JacksAndDeuces)]
    #[case("TS TH 9♥ 9D AC", 2930, HandRankName::TwoPair, HandRankClass::TensAndNines)]
    #[case("TS TH 9♥ 9D 2♥", 2940, HandRankName::TwoPair, HandRankClass::TensAndNines)]
    #[case("TS TH 8♥ 8D AC", 2941, HandRankName::TwoPair, HandRankClass::TensAndEights)]
    #[case("TS TH 8♥ 8D 2♥", 2951, HandRankName::TwoPair, HandRankClass::TensAndEights)]
    #[case("TS TH 7♥ 7D AC", 2952, HandRankName::TwoPair, HandRankClass::TensAndSevens)]
    #[case("TS TH 7♥ 7D 2♥", 2962, HandRankName::TwoPair, HandRankClass::TensAndSevens)]
    #[case("TS TH 6♥ 6D AC", 2963, HandRankName::TwoPair, HandRankClass::TensAndSixes)]
    #[case("TS TH 6♥ 6D 2♥", 2973, HandRankName::TwoPair, HandRankClass::TensAndSixes)]
    #[case("TS TH 5♥ 5D AC", 2974, HandRankName::TwoPair, HandRankClass::TensAndFives)]
    #[case("TS TH 5♥ 5D 2♥", 2984, HandRankName::TwoPair, HandRankClass::TensAndFives)]
    #[case("TS TH 4♥ 4D AC", 2985, HandRankName::TwoPair, HandRankClass::TensAndFours)]
    #[case("TS TH 4♥ 4D 2♥", 2995, HandRankName::TwoPair, HandRankClass::TensAndFours)]
    #[case("TS TH 3♥ 3D AC", 2996, HandRankName::TwoPair, HandRankClass::TensAndTreys)]
    #[case("TS TH 3♥ 3D 2♥", 3006, HandRankName::TwoPair, HandRankClass::TensAndTreys)]
    #[case("TS TH 2♥ 2D AC", 3007, HandRankName::TwoPair, HandRankClass::TensAndDeuces)]
    #[case("TS TH 2♥ 2D 3♥", 3017, HandRankName::TwoPair, HandRankClass::TensAndDeuces)]
    #[case("9S 9H 8♥ 8D AC", 3018, HandRankName::TwoPair, HandRankClass::NinesAndEights)]
    #[case("9S 9H 8♥ 8D 2♥", 3028, HandRankName::TwoPair, HandRankClass::NinesAndEights)]
    #[case("9S 9H 7♥ 7D AC", 3029, HandRankName::TwoPair, HandRankClass::NinesAndSevens)]
    #[case("9S 9H 7♥ 7D 2♥", 3039, HandRankName::TwoPair, HandRankClass::NinesAndSevens)]
    #[case("9S 9H 6♥ 6D AC", 3040, HandRankName::TwoPair, HandRankClass::NinesAndSixes)]
    #[case("9S 9H 6♥ 6D 2♥", 3050, HandRankName::TwoPair, HandRankClass::NinesAndSixes)]
    #[case("9S 9H 5♥ 5D AC", 3051, HandRankName::TwoPair, HandRankClass::NinesAndFives)]
    #[case("9S 9H 5♥ 5D 2♥", 3061, HandRankName::TwoPair, HandRankClass::NinesAndFives)]
    #[case("9S 9H 4♥ 4D AC", 3062, HandRankName::TwoPair, HandRankClass::NinesAndFours)]
    #[case("9S 9H 4♥ 4D 2♥", 3072, HandRankName::TwoPair, HandRankClass::NinesAndFours)]
    #[case("9S 9H 3♥ 3D AC", 3073, HandRankName::TwoPair, HandRankClass::NinesAndTreys)]
    #[case("9S 9H 3♥ 3D 2♥", 3083, HandRankName::TwoPair, HandRankClass::NinesAndTreys)]
    #[case("9S 9H 2♥ 2D AC", 3084, HandRankName::TwoPair, HandRankClass::NinesAndDeuces)]
    #[case("9S 9H 2♥ 2D 3♥", 3094, HandRankName::TwoPair, HandRankClass::NinesAndDeuces)]
    #[case("8S 8H 7♥ 7D AC", 3095, HandRankName::TwoPair, HandRankClass::EightsAndSevens)]
    #[case("8S 8H 7♥ 7D 2♥", 3105, HandRankName::TwoPair, HandRankClass::EightsAndSevens)]
    #[case("8S 8H 6♥ 6D AC", 3106, HandRankName::TwoPair, HandRankClass::EightsAndSixes)]
    #[case("8S 8H 6♥ 6D 2♥", 3116, HandRankName::TwoPair, HandRankClass::EightsAndSixes)]
    #[case("8S 8H 5♥ 5D AC", 3117, HandRankName::TwoPair, HandRankClass::EightsAndFives)]
    #[case("8S 8H 5♥ 5D 2♥", 3127, HandRankName::TwoPair, HandRankClass::EightsAndFives)]
    #[case("8S 8H 4♥ 4D AC", 3128, HandRankName::TwoPair, HandRankClass::EightsAndFours)]
    #[case("8S 8H 4♥ 4D 2♥", 3138, HandRankName::TwoPair, HandRankClass::EightsAndFours)]
    #[case("8S 8H 3♥ 3D AC", 3139, HandRankName::TwoPair, HandRankClass::EightsAndTreys)]
    #[case("8S 8H 3♥ 3D 2♥", 3149, HandRankName::TwoPair, HandRankClass::EightsAndTreys)]
    #[case("8S 8H 2♥ 2D AC", 3150, HandRankName::TwoPair, HandRankClass::EightsAndDeuces)]
    #[case("8S 8H 2♥ 2D 3♥", 3160, HandRankName::TwoPair, HandRankClass::EightsAndDeuces)]
    #[case("7♥ 7D 6S 6C A♥", 3161, HandRankName::TwoPair, HandRankClass::SevensAndSixes)]
    #[case("7♥ 7D 6S 6♥ 2D", 3171, HandRankName::TwoPair, HandRankClass::SevensAndSixes)]
    #[case("7♥ 7D 5S 5C A♥", 3172, HandRankName::TwoPair, HandRankClass::SevensAndFives)]
    #[case("7♥ 7D 5S 5♥ 2D", 3182, HandRankName::TwoPair, HandRankClass::SevensAndFives)]
    #[case("7♥ 7D 4S 4C A♥", 3183, HandRankName::TwoPair, HandRankClass::SevensAndFours)]
    #[case("7♥ 7D 4S 4♥ 2D", 3193, HandRankName::TwoPair, HandRankClass::SevensAndFours)]
    #[case("7♥ 7D 3S 3C A♥", 3194, HandRankName::TwoPair, HandRankClass::SevensAndTreys)]
    #[case("7♥ 7D 3S 3♥ 2D", 3204, HandRankName::TwoPair, HandRankClass::SevensAndTreys)]
    #[case("7♥ 7D 2S 2C A♥", 3205, HandRankName::TwoPair, HandRankClass::SevensAndDeuces)]
    #[case("7♥ 7D 2S 2♥ 3D", 3215, HandRankName::TwoPair, HandRankClass::SevensAndDeuces)]
    #[case("6♥ 6D 5S 5C A♥", 3216, HandRankName::TwoPair, HandRankClass::SixesAndFives)]
    #[case("6♥ 6D 5S 5♥ 2D", 3226, HandRankName::TwoPair, HandRankClass::SixesAndFives)]
    #[case("6♥ 6D 4S 4C A♥", 3227, HandRankName::TwoPair, HandRankClass::SixesAndFours)]
    #[case("6♥ 6D 4S 4♥ 2D", 3237, HandRankName::TwoPair, HandRankClass::SixesAndFours)]
    #[case("6♥ 6D 3S 3C A♥", 3238, HandRankName::TwoPair, HandRankClass::SixesAndTreys)]
    #[case("6♥ 6D 3S 3♥ 2D", 3248, HandRankName::TwoPair, HandRankClass::SixesAndTreys)]
    #[case("6♥ 6D 2S 2C A♥", 3249, HandRankName::TwoPair, HandRankClass::SixesAndDeuces)]
    #[case("6♥ 6D 2S 2♥ 3D", 3259, HandRankName::TwoPair, HandRankClass::SixesAndDeuces)]
    #[case("5S 5C 4S 4D A♥", 3260, HandRankName::TwoPair, HandRankClass::FivesAndFours)]
    #[case("5S 5♥ 4S 4C 2D", 3270, HandRankName::TwoPair, HandRankClass::FivesAndFours)]
    #[case("5S 5C 3S 3D A♥", 3271, HandRankName::TwoPair, HandRankClass::FivesAndTreys)]
    #[case("5S 5♥ 3S 3C 2D", 3281, HandRankName::TwoPair, HandRankClass::FivesAndTreys)]
    #[case("5S 5C 2S 2D A♥", 3282, HandRankName::TwoPair, HandRankClass::FivesAndDeuces)]
    #[case("5S 5♥ 2S 2C 3D", 3292, HandRankName::TwoPair, HandRankClass::FivesAndDeuces)]
    #[case("4♥ 4D 3S 3C A♥", 3293, HandRankName::TwoPair, HandRankClass::FoursAndTreys)]
    #[case("4♥ 4D 3S 3♥ 2D", 3303, HandRankName::TwoPair, HandRankClass::FoursAndTreys)]
    #[case("4♥ 4D 2S 2C A♥", 3304, HandRankName::TwoPair, HandRankClass::FoursAndDeuces)]
    #[case("4♥ 4D 2S 2♥ 3D", 3314, HandRankName::TwoPair, HandRankClass::FoursAndDeuces)]
    #[case("3♥ 3D 2S 2C A♥", 3315, HandRankName::TwoPair, HandRankClass::TreysAndDeuces)]
    #[case("3♥ 3D 2S 2♥ 4D", 3325, HandRankName::TwoPair, HandRankClass::TreysAndDeuces)]
    #[case("A♥ AD KS Q♥ JD", 3326, HandRankName::Pair, HandRankClass::PairOfAces)]
    #[case("A♥ AD 4S 3♥ 2D", 3545, HandRankName::Pair, HandRankClass::PairOfAces)]
    #[case("K♥ KD AS Q♥ JD", 3546, HandRankName::Pair, HandRankClass::PairOfKings)]
    #[case("K♥ KD 4S 3♥ 2D", 3765, HandRankName::Pair, HandRankClass::PairOfKings)]
    #[case("Q♥ QD AS K♥ JD", 3766, HandRankName::Pair, HandRankClass::PairOfQueens)]
    #[case("Q♥ QD 4S 3♥ 2D", 3985, HandRankName::Pair, HandRankClass::PairOfQueens)]
    #[case("J♥ JD AS K♥ QD", 3986, HandRankName::Pair, HandRankClass::PairOfJacks)]
    #[case("J♥ JD 4S 3♥ 2D", 4205, HandRankName::Pair, HandRankClass::PairOfJacks)]
    #[case("T♥ TD AS K♥ QD", 4206, HandRankName::Pair, HandRankClass::PairOfTens)]
    #[case("T♥ TD 4S 3♥ 2D", 4425, HandRankName::Pair, HandRankClass::PairOfTens)]
    #[case("9♥ 9D AS K♥ QD", 4426, HandRankName::Pair, HandRankClass::PairOfNines)]
    #[case("9♥ 9D 4S 3♥ 2D", 4645, HandRankName::Pair, HandRankClass::PairOfNines)]
    #[case("8♥ 8D AS K♥ QD", 4646, HandRankName::Pair, HandRankClass::PairOfEights)]
    #[case("8♥ 8D 4S 3♥ 2D", 4865, HandRankName::Pair, HandRankClass::PairOfEights)]
    #[case("7♥ 7D AS K♥ QD", 4866, HandRankName::Pair, HandRankClass::PairOfSevens)]
    #[case("7♥ 7D 4S 3♥ 2D", 5085, HandRankName::Pair, HandRankClass::PairOfSevens)]
    #[case("6♥ 6D AS K♥ QD", 5086, HandRankName::Pair, HandRankClass::PairOfSixes)]
    #[case("6♥ 6D 4S 3♥ 2D", 5305, HandRankName::Pair, HandRankClass::PairOfSixes)]
    #[case("5♥ 5D AS K♥ QD", 5306, HandRankName::Pair, HandRankClass::PairOfFives)]
    #[case("5♥ 5D 4S 3♥ 2D", 5525, HandRankName::Pair, HandRankClass::PairOfFives)]
    #[case("4♥ 4D AS K♥ QD", 5526, HandRankName::Pair, HandRankClass::PairOfFours)]
    #[case("4♥ 4D 5S 3♥ 2D", 5745, HandRankName::Pair, HandRankClass::PairOfFours)]
    #[case("3♥ 3D AS K♥ QD", 5746, HandRankName::Pair, HandRankClass::PairOfTreys)]
    #[case("3♥ 3D 5S 4♥ 2D", 5965, HandRankName::Pair, HandRankClass::PairOfTreys)]
    #[case("2♥ 2D AS K♥ QD", 5966, HandRankName::Pair, HandRankClass::PairOfDeuces)]
    #[case("2♥ 2D 5S 4♥ 3D", 6185, HandRankName::Pair, HandRankClass::PairOfDeuces)]
    #[case("AD KD Q♥ JD 9D", 6186, HandRankName::HighCard, HandRankClass::AceHigh)]
    #[case("AD 6D 4♥ 3D 2D", 6678, HandRankName::HighCard, HandRankClass::AceHigh)]
    #[case("KD Q♥ JD TD 8C", 6679, HandRankName::HighCard, HandRankClass::KingHigh)]
    #[case("KD 5D 4♥ 3D 2D", 7007, HandRankName::HighCard, HandRankClass::KingHigh)]
    #[case("Q♥ JD TD 9C 7D", 7008, HandRankName::HighCard, HandRankClass::QueenHigh)]
    #[case("QD 5D 4♥ 3D 2D", 7216, HandRankName::HighCard, HandRankClass::QueenHigh)]
    #[case("JD TD 9C 8D 6C", 7217, HandRankName::HighCard, HandRankClass::JackHigh)]
    #[case("JD 5D 4♥ 3D 2D", 7341, HandRankName::HighCard, HandRankClass::JackHigh)]
    #[case("TD 9C 8D 7C 5S", 7342, HandRankName::HighCard, HandRankClass::TenHigh)]
    #[case("TD 5D 4♥ 3D 2D", 7410, HandRankName::HighCard, HandRankClass::TenHigh)]
    #[case("9C 8D 7C 6S 4D", 7411, HandRankName::HighCard, HandRankClass::NineHigh)]
    #[case("9D 5D 4♥ 3D 2D", 7444, HandRankName::HighCard, HandRankClass::NineHigh)]
    #[case("8D 7C 6S 5D 3H", 7445, HandRankName::HighCard, HandRankClass::EightHigh)]
    #[case("8D 5D 4♥ 3D 2D", 7458, HandRankName::HighCard, HandRankClass::EightHigh)]
    #[case("7C 6S 5D 4H 2C", 7459, HandRankName::HighCard, HandRankClass::SevenHigh)]
    #[case("7D 5D 4♥ 3D 2D", 7462, HandRankName::HighCard, HandRankClass::SevenHigh)]
    #[case("A♠ A♠ Q♠ J♠ T♠", 0, HandRankName::Invalid, HandRankClass::Invalid)]
    fn hand_rank_value(
        #[case] index: &'static str,
        #[case] expected_value: HandRankValue,
        #[case] expected_name: HandRankName,
        #[case] expected_class: HandRankClass,
    ) {
        let hand = Five::try_from(index).unwrap();

        // let hand_rank_value = hand.hand_rank_value();
        let hand_rank = hand.hand_rank();

        assert_eq!(expected_value, hand_rank.value);
        assert_eq!(expected_name, hand_rank.name);
        assert_eq!(expected_class, hand_rank.class);
    }

    #[test]
    fn hand_rank_value__royal_flush() {
        assert_eq!(
            1,
            Five::try_from("A♠ K♠ Q♠ J♠ T♠").unwrap().hand_rank_value()
        );
    }

    #[test]
    fn and_bits() {
        let hand = Five::try_from("A♠ K♠ Q♠ J♠ T♠").unwrap();

        let and_bits = hand.and_bits();

        assert_eq!(CardNumber::ACE_SPADES, hand.first());
        assert_eq!(CardNumber::KING_SPADES, hand.second());
        assert_eq!(CardNumber::QUEEN_SPADES, hand.third());
        assert_eq!(CardNumber::JACK_SPADES, hand.forth());
        assert_eq!(CardNumber::TEN_SPADES, hand.fifth());
        assert_eq!(
            "00010000000000001000110000101001",
            format!("{:032b}", hand.first())
        );
        assert_eq!(
            "00001000000000001000101100100101",
            format!("{:032b}", hand.second())
        );
        assert_eq!(
            "00000100000000001000101000011111",
            format!("{:032b}", hand.third())
        );
        assert_eq!(
            "00000010000000001000100100011101",
            format!("{:032b}", hand.forth())
        );
        assert_eq!(
            "00000001000000001000100000010111",
            format!("{:032b}", hand.fifth())
        );
        assert_eq!(
            "00000000000000001000100000000001",
            format!("{:032b}", and_bits)
        );
    }

    #[test]
    fn is_flush() {
        assert!(Five::try_from("A♠ K♠ Q♠ J♠ T♠").unwrap().is_flush());
        assert!(!Five::try_from("A♠ K♥ Q♠ J♠ T♠").unwrap().is_flush());
    }

    #[test]
    fn is_straight() {
        assert!(Five::try_from("A♠ K♥ Q♠ J♠ T♠").unwrap().is_straight());
        assert!(Five::try_from("K♥ Q♥ J♥ T♥ 9♠").unwrap().is_straight());
        assert!(Five::try_from("Q♥ J♥ T♥ 9♠ 8C").unwrap().is_straight());
        assert!(Five::try_from("J♠ T♥ 9♠ 8♠ 7C").unwrap().is_straight());
        assert!(Five::try_from("T♥ 9♠ 8♠ 7C 6S").unwrap().is_straight());
        assert!(Five::try_from("9♠ 8♠ 7C 6S 5♥").unwrap().is_straight());
        assert!(Five::try_from("8♠ 7C 6S 5♥ 4D").unwrap().is_straight());
        assert!(Five::try_from("7C 6S 5♥ 4D 3C").unwrap().is_straight());
        assert!(Five::try_from("6S 5♥ 4D 3C 2H").unwrap().is_straight());
        assert!(Five::try_from("5♥ 4D 3C 2H AS").unwrap().is_straight());
    }

    #[test]
    fn is_straight__false() {
        assert!(!Five::try_from("6♥ 4D 3C 2H AS").unwrap().is_straight());
        assert!(!Five::try_from("K♥ Q♥ J♥ T♥ 8D").unwrap().is_straight());
    }

    #[test]
    fn is_straight_flush() {
        assert!(Five::try_from("A♠ K♠ Q♠ J♠ T♠")
            .unwrap()
            .is_straight_flush());
        assert!(Five::try_from("K♠ Q♠ J♠ T♠ 9♠")
            .unwrap()
            .is_straight_flush());
    }

    #[test]
    fn is_straight_false() {
        assert!(!Five::try_from("A♠ K♥ Q♠ J♠ T♠")
            .unwrap()
            .is_straight_flush());
    }

    #[test]
    fn is_wheel() {
        let wheel = Five::try_from("5♥ 4D 3C 2H A♠").unwrap();

        assert_eq!("0001000000001111", format!("{:016b}", wheel.or_rank_bits()));
        assert_eq!(Five::WHEEL_OR_BITS, wheel.or_rank_bits());
        assert!(wheel.is_wheel());
        assert!(!Five::try_from("7♥ 4D 3C 2H AS").unwrap().is_wheel());
    }

    #[test]
    fn or_rank_bits() {
        let or = Five::try_from("A♠ K♠ Q♠ J♠ T♠").unwrap().or_rank_bits();

        assert_eq!("0001111100000000", format!("{:016b}", or));
        assert_eq!("00000000000000000001111100000000", format!("{:032b}", or));
        assert_eq!(8, or.trailing_zeros());
        assert_eq!(19, or.leading_zeros());
        assert_eq!(or, 7936);
    }

    #[test]
    fn sort() {
        let five = Five::try_from("KC QD A♠ 9h T♠").unwrap().sort();

        let expected = Five::try_from("A♠ KC QD T♠ 9h").unwrap();

        assert_eq!(five, expected);
    }

    #[test]
    fn default() {
        let five = Five::default();

        assert_eq!(five.first(), CardNumber::BLANK);
        assert_eq!(five.second(), CardNumber::BLANK);
        assert_eq!(five.third(), CardNumber::BLANK);
        assert_eq!(five.forth(), CardNumber::BLANK);
        assert_eq!(five.fifth(), CardNumber::BLANK);
        assert!(five.contain_blank());
        assert!(!five.are_unique());
        assert!(!five.is_valid());
    }

    #[test]
    fn hand_validator__is_corrupt() {
        let first = Five::from([
            CardNumber::JACK_CLUBS,
            CardNumber::DEUCE_CLUBS,
            23,
            CardNumber::KING_SPADES,
            CardNumber::TEN_SPADES,
        ]);
        let second = Five::from([
            CardNumber::JACK_CLUBS,
            CardNumber::QUEEN_DIAMONDS,
            CardNumber::TREY_CLUBS,
            CardNumber::KING_SPADES,
            CardNumber::BLANK,
        ]);

        assert!(first.is_corrupt());
        assert!(second.is_corrupt());
    }

    #[test]
    fn hand_validator__are_unique() {
        let first = Five::from([
            CardNumber::JACK_CLUBS,
            CardNumber::TREY_CLUBS,
            CardNumber::DEUCE_CLUBS,
            CardNumber::KING_SPADES,
            CardNumber::TEN_SPADES,
        ]);
        let second = Five::from([
            CardNumber::JACK_CLUBS,
            CardNumber::QUEEN_DIAMONDS,
            CardNumber::TREY_CLUBS,
            CardNumber::KING_SPADES,
            CardNumber::ACE_HEARTS,
        ]);
        let third = Five::try_from("A♠ K♠ Q♠ J♠ T♠").unwrap();

        assert!(first.are_unique());
        assert!(second.are_unique());
        assert!(third.are_unique());
    }

    #[test]
    fn hand_validator__are_unique__false() {
        let first = Five::from([
            CardNumber::JACK_CLUBS,
            CardNumber::DEUCE_CLUBS,
            CardNumber::DEUCE_CLUBS,
            CardNumber::KING_SPADES,
            CardNumber::TEN_SPADES,
        ]);
        let second = Five::from([
            CardNumber::JACK_CLUBS,
            CardNumber::QUEEN_DIAMONDS,
            CardNumber::TREY_CLUBS,
            CardNumber::KING_SPADES,
            CardNumber::KING_SPADES,
        ]);
        let third = Five::try_from("A♠ A♠ Q♠ J♠ T♠").unwrap();

        assert!(!first.are_unique());
        assert!(!second.are_unique());
        assert!(!third.are_unique());
    }

    #[test]
    fn try_from__index() {
        let five = Five::try_from("A♠ K♠ Q♠ J♠ T♠");

        assert!(five.is_ok());
        let five = five.unwrap();
        assert_eq!(five.first(), CardNumber::ACE_SPADES);
        assert_eq!(five.second(), CardNumber::KING_SPADES);
        assert_eq!(five.third(), CardNumber::QUEEN_SPADES);
        assert_eq!(five.forth(), CardNumber::JACK_SPADES);
        assert_eq!(five.fifth(), CardNumber::TEN_SPADES);
        assert!(!five.contain_blank());
        assert!(five.are_unique());
        assert!(five.is_valid());
    }

    #[test]
    fn try_from__index__blank() {
        let five = Five::try_from("A♠ K♠ XX J♠ T♠");

        assert!(five.is_ok());
        let five = five.unwrap();
        assert_eq!(five.first(), CardNumber::ACE_SPADES);
        assert_eq!(five.second(), CardNumber::KING_SPADES);
        assert_eq!(five.third(), CardNumber::BLANK);
        assert_eq!(five.forth(), CardNumber::JACK_SPADES);
        assert_eq!(five.fifth(), CardNumber::TEN_SPADES);
        assert!(five.contain_blank());
        assert!(!five.is_valid());
    }

    #[test]
    fn try_from__index__too_short() {
        let five = Five::try_from("A♠ K♠ Q♠ J♠");

        assert!(five.is_err());
    }
}
