use crate::cards::HandValidator;
use crate::{CKCNumber, HandError, PokerCard};
use core::slice::Iter;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct Four([CKCNumber; 4]);

impl Four {
    //region accessors

    #[must_use]
    pub fn first(&self) -> CKCNumber {
        self.0[0]
    }

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

    #[must_use]
    pub fn to_arr(&self) -> [CKCNumber; 4] {
        self.0
    }

    //endregion

    fn from_index(index: &str) -> Option<[CKCNumber; 4]> {
        let mut esses = index.split_whitespace();

        let first = CKCNumber::from_index(esses.next()?);
        let second = CKCNumber::from_index(esses.next()?);
        let third = CKCNumber::from_index(esses.next()?);
        let forth = CKCNumber::from_index(esses.next()?);
        let hand: [CKCNumber; 4] = [first, second, third, forth];
        Some(hand)
    }
}

impl From<[CKCNumber; 4]> for Four {
    fn from(array: [CKCNumber; 4]) -> Self {
        Four(array)
    }
}

impl HandValidator for Four {
    fn are_unique(&self) -> bool {
        (self.first() != self.second())
            && (self.first() != self.third())
            && (self.first() != self.forth())
            && (self.second() != self.third())
            && (self.second() != self.forth())
            && (self.third() != self.forth())
    }

    fn sort(&self) -> Four {
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

impl TryFrom<&'static str> for Four {
    type Error = HandError;

    fn try_from(index: &'static str) -> Result<Self, Self::Error> {
        match Four::from_index(index) {
            None => Err(HandError::InvalidIndex),
            Some(four) => Ok(Four::from(four)),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cards_four_tests {
    use super::*;
    use crate::CardNumber;

    #[test]
    fn sort() {
        let four = Four::try_from("KC QD A♠ T♠").unwrap().sort();

        let expected = Four::try_from("A♠ KC QD T♠").unwrap();

        assert_eq!(four, expected);
    }

    #[test]
    fn default() {
        let four = Four::default();

        assert_eq!(four.first(), CardNumber::BLANK);
        assert_eq!(four.second(), CardNumber::BLANK);
        assert_eq!(four.third(), CardNumber::BLANK);
        assert_eq!(four.forth(), CardNumber::BLANK);
        assert!(four.contain_blank());
        assert!(!four.are_unique());
        assert!(!four.is_valid());
    }

    #[test]
    fn try_from__index() {
        let four = Four::try_from("A♠ K♠ Q♠ J♠");

        assert!(four.is_ok());
        let four = four.unwrap();
        assert_eq!(four.first(), CardNumber::ACE_SPADES);
        assert_eq!(four.second(), CardNumber::KING_SPADES);
        assert_eq!(four.third(), CardNumber::QUEEN_SPADES);
        assert_eq!(four.forth(), CardNumber::JACK_SPADES);
        assert!(!four.contain_blank());
        assert!(four.are_unique());
        assert!(four.is_valid());
    }

    #[test]
    fn try_from__index__blank() {
        let four = Four::try_from("A♠ K♠ XX J♠");

        assert!(four.is_ok());
        let four = four.unwrap();
        assert_eq!(four.first(), CardNumber::ACE_SPADES);
        assert_eq!(four.second(), CardNumber::KING_SPADES);
        assert_eq!(four.third(), CardNumber::BLANK);
        assert_eq!(four.forth(), CardNumber::JACK_SPADES);
        assert!(four.contain_blank());
        assert!(!four.is_valid());
    }

    #[test]
    fn try_from__index__too_short() {
        let four = Four::try_from("A♠ K♠ Q♠");

        assert!(four.is_err());
    }
}
