use crate::cards::HandValidator;
use crate::{CKCNumber, HandError, PokerCard};
use core::slice::Iter;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Three(pub [CKCNumber; 3]);

impl Three {
    //region getters

    #[must_use]
    pub fn second(&self) -> CKCNumber {
        self.0[1]
    }

    #[must_use]
    pub fn third(&self) -> CKCNumber {
        self.0[2]
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

    //endregion

    #[must_use]
    pub fn to_arr(&self) -> [CKCNumber; 3] {
        self.0
    }

    fn from_index(index: &str) -> Option<[CKCNumber; 3]> {
        let mut esses = index.split_whitespace();

        let first = CKCNumber::from_index(esses.next()?);
        let second = CKCNumber::from_index(esses.next()?);
        let third = CKCNumber::from_index(esses.next()?);
        let hand: [CKCNumber; 3] = [first, second, third];
        Some(hand)
    }
}

impl From<[CKCNumber; 3]> for Three {
    fn from(array: [CKCNumber; 3]) -> Self {
        Three(array)
    }
}

impl TryFrom<&'static str> for Three {
    type Error = HandError;

    fn try_from(index: &'static str) -> Result<Self, Self::Error> {
        match Three::from_index(index) {
            None => Err(HandError::InvalidIndex),
            Some(three) => Ok(Three::from(three)),
        }
    }
}

impl HandValidator for Three {
    fn are_unique(&self) -> bool {
        (self.first() != self.second())
            && (self.first() != self.third())
            && (self.second() != self.third())
    }

    fn first(&self) -> CKCNumber {
        self.0[0]
    }

    fn sort(&self) -> Three {
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

#[cfg(test)]
#[allow(non_snake_case)]
mod cards_three_tests {
    use super::*;
    use crate::CardNumber;

    #[test]
    fn sort() {
        let three = Three::try_from("KC QD A♠").unwrap().sort();

        let expected = Three::try_from("A♠ KC QD").unwrap();

        assert_eq!(three, expected);
    }

    #[test]
    fn default() {
        let three = Three::default();

        assert_eq!(three.first(), CardNumber::BLANK);
        assert_eq!(three.second(), CardNumber::BLANK);
        assert_eq!(three.third(), CardNumber::BLANK);
        assert!(three.contain_blank());
        assert!(!three.are_unique());
        assert!(!three.is_valid());
    }

    #[test]
    fn try_from__index() {
        let three = Three::try_from("A♠ K♠ Q♠");

        assert!(three.is_ok());
        let three = three.unwrap();
        assert_eq!(three.first(), CardNumber::ACE_SPADES);
        assert_eq!(three.second(), CardNumber::KING_SPADES);
        assert_eq!(three.third(), CardNumber::QUEEN_SPADES);
        assert!(!three.contain_blank());
        assert!(three.are_unique());
        assert!(three.is_valid());
    }

    #[test]
    fn try_from__index__blank() {
        let three = Three::try_from("A♠ K♠ XX");

        assert!(three.is_ok());
        let three = three.unwrap();
        assert_eq!(three.first(), CardNumber::ACE_SPADES);
        assert_eq!(three.second(), CardNumber::KING_SPADES);
        assert_eq!(three.third(), CardNumber::BLANK);
        assert!(three.contain_blank());
        assert!(!three.is_valid());
    }

    #[test]
    fn try_from__index__too_short() {
        let three = Three::try_from("A♠ K♠");

        assert!(three.is_err());
    }
}
