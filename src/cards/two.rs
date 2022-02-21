use crate::cards::HandValidator;
use crate::{CKCNumber, HandError, PokerCard};
use core::cmp;
use core::slice::Iter;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct Two([CKCNumber; 2]);

impl Two {
    #[must_use]
    pub fn new(first: CKCNumber, second: CKCNumber) -> Self {
        Self([first, second])
    }

    //region accessors

    #[must_use]
    pub fn second(&self) -> CKCNumber {
        self.0[1]
    }

    pub fn set_first(&mut self, card_number: CKCNumber) {
        self.0[0] = card_number;
    }

    pub fn set_second(&mut self, card_number: CKCNumber) {
        self.0[1] = card_number;
    }

    #[must_use]
    pub fn to_arr(&self) -> [CKCNumber; 2] {
        self.0
    }

    //endregion

    #[must_use]
    pub fn high_card(&self) -> CKCNumber {
        cmp::max(self.first(), self.second())
    }

    fn from_index(index: &str) -> Option<[CKCNumber; 2]> {
        let mut esses = index.split_whitespace();

        let first = CKCNumber::from_index(esses.next()?);
        let second = CKCNumber::from_index(esses.next()?);
        let hand: [CKCNumber; 2] = [first, second];
        Some(hand)
    }
}

impl From<&[CKCNumber; 2]> for Two {
    fn from(array: &[CKCNumber; 2]) -> Self {
        Two(*array)
    }
}

impl From<[CKCNumber; 2]> for Two {
    fn from(array: [CKCNumber; 2]) -> Self {
        Two(array)
    }
}

impl TryFrom<&'static str> for Two {
    type Error = HandError;

    /// # Errors
    ///
    /// Will return `CardError::InvalidIndex` for an invalid index.
    fn try_from(index: &'static str) -> Result<Self, Self::Error> {
        match Two::from_index(index) {
            None => Err(HandError::InvalidIndex),
            Some(five) => Ok(Two::from(five)),
        }
    }
}

impl HandValidator for Two {
    fn are_unique(&self) -> bool {
        self.first() != self.second()
    }

    fn first(&self) -> CKCNumber {
        self.0[0]
    }

    fn sort(&self) -> Self {
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
mod cards_two_tests {
    use super::*;
    use crate::CardNumber;

    #[test]
    fn are_unique() {
        assert!(!Two::new(CardNumber::ACE_CLUBS, CardNumber::ACE_CLUBS).are_unique());
        assert!(!Two::new(CardNumber::BLANK, CardNumber::BLANK).are_unique());
        assert!(Two::new(CardNumber::ACE_SPADES, CardNumber::ACE_CLUBS).are_unique());
    }

    #[test]
    fn contain_blank() {
        assert!(!Two::new(CardNumber::ACE_SPADES, CardNumber::ACE_CLUBS).contain_blank());
        assert!(!Two::new(CardNumber::ACE_CLUBS, CardNumber::ACE_CLUBS).contain_blank());
        assert!(Two::new(CardNumber::BLANK, CardNumber::BLANK).contain_blank());
        assert!(Two::new(CardNumber::BLANK, CardNumber::ACE_CLUBS).contain_blank());
        assert!(Two::new(CardNumber::ACE_CLUBS, CardNumber::BLANK).contain_blank());
    }

    #[test]
    fn is_valid() {
        assert!(!Two::new(CardNumber::ACE_CLUBS, CardNumber::ACE_CLUBS).is_valid());
        assert!(!Two::new(CardNumber::BLANK, CardNumber::BLANK).is_valid());
        assert!(!Two::new(CardNumber::BLANK, CardNumber::ACE_CLUBS).is_valid());
        assert!(!Two::new(CardNumber::ACE_CLUBS, CardNumber::BLANK).is_valid());
        assert!(Two::new(CardNumber::ACE_SPADES, CardNumber::ACE_CLUBS).is_valid());
    }

    #[test]
    fn high_card() {
        let hand = Two::new(CardNumber::ACE_CLUBS, CardNumber::KING_SPADES);

        assert_eq!(hand.high_card(), CardNumber::ACE_CLUBS);
    }

    #[test]
    fn try_from__index() {
        let two = Two::try_from("J♠ T♠");

        assert!(two.is_ok());
        let two = two.unwrap();
        assert_eq!(two.first(), CardNumber::JACK_SPADES);
        assert_eq!(two.second(), CardNumber::TEN_SPADES);
    }

    #[test]
    fn try_from__index__blank() {
        let two = Two::try_from("A♠ XX");

        assert!(two.is_ok());
        let two = two.unwrap();
        assert_eq!(two.first(), CardNumber::ACE_SPADES);
        assert_eq!(two.second(), CardNumber::BLANK);
    }

    #[test]
    fn try_from__index__too_short() {
        let two = Two::try_from("A♠");

        assert!(two.is_err());
    }
}
