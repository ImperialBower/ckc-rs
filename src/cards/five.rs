use crate::{CKCNumber, HandError, PokerCard};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct Five([CKCNumber; 5]);

impl Five {
    #[must_use]
    pub fn sort(&self) -> Five {
        let mut array = *self;
        array.sort_in_place();
        array
    }

    pub fn sort_in_place(&mut self) {
        self.0.sort_unstable();
        self.0.reverse();
    }

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
}

impl From<[CKCNumber; 5]> for Five {
    fn from(array: [CKCNumber; 5]) -> Self {
        Five(array)
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
    }

    #[test]
    fn try_from__index__too_short() {
        let five = Five::try_from("A♠ K♠ Q♠ J♠");

        assert!(five.is_err());
    }
}
