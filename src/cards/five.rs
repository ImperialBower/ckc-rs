use crate::cards::HandValidator;
use crate::{CKCNumber, CardNumber, HandError, PokerCard};
use core::slice::Iter;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct Five([CKCNumber; 5]);

impl Five {
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

    //region bitwise

    #[must_use]
    pub fn and_bits(&self) -> u32 {
        self.first() & self.second() & self.third() & self.forth() & self.fifth()
    }

    #[must_use]
    pub fn is_flush(&self) -> bool {
        (self.and_bits() & CardNumber::SUIT_FILTER) != 0
    }

    #[must_use]
    pub fn is_straight(&self) -> bool {
        let rank_bits = self.or_rank_bits();
        self.is_valid()
            && (((rank_bits.trailing_zeros() + rank_bits.leading_zeros())
                == Five::STRAIGHT_PADDING)
                || rank_bits == Five::WHEEL_OR_BITS)
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
        let sorted = self.sort();
        let mut last: CKCNumber = u32::MAX;
        for c in sorted.iter() {
            if *c >= last {
                return false;
            }
            last = *c;
        }
        true
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
mod cards_five_tests {
    use super::*;
    use crate::CardNumber;
    use alloc::format;

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
        let wheel = Five::try_from("5♥ 4D 3C 2H AS").unwrap();

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
