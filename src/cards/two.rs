use crate::cards::HandValidator;
use crate::{BinaryCard, CKCNumber, CardNumber, HandError, PokerCard, Shifty, BC64};
use core::cmp;
use core::slice::Iter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Two([CKCNumber; 2]);

#[allow(non_upper_case_globals)]
impl Two {
    //region hands
    pub const AA: [Two; 6] = [
        Two([CardNumber::ACE_SPADES, CardNumber::ACE_HEARTS]),
        Two([CardNumber::ACE_SPADES, CardNumber::ACE_DIAMONDS]),
        Two([CardNumber::ACE_SPADES, CardNumber::ACE_CLUBS]),
        Two([CardNumber::ACE_HEARTS, CardNumber::ACE_DIAMONDS]),
        Two([CardNumber::ACE_HEARTS, CardNumber::ACE_CLUBS]),
        Two([CardNumber::ACE_DIAMONDS, CardNumber::ACE_CLUBS]),
    ];
    pub const AKs: [Two; 4] = [
        Two([CardNumber::ACE_SPADES, CardNumber::KING_SPADES]),
        Two([CardNumber::ACE_HEARTS, CardNumber::KING_HEARTS]),
        Two([CardNumber::ACE_DIAMONDS, CardNumber::KING_DIAMONDS]),
        Two([CardNumber::ACE_CLUBS, CardNumber::KING_CLUBS]),
    ];
    pub const AKo: [Two; 12] = [
        Two([CardNumber::ACE_SPADES, CardNumber::KING_HEARTS]),
        Two([CardNumber::ACE_SPADES, CardNumber::KING_DIAMONDS]),
        Two([CardNumber::ACE_SPADES, CardNumber::KING_CLUBS]),
        Two([CardNumber::ACE_HEARTS, CardNumber::KING_SPADES]),
        Two([CardNumber::ACE_HEARTS, CardNumber::KING_DIAMONDS]),
        Two([CardNumber::ACE_HEARTS, CardNumber::KING_CLUBS]),
        Two([CardNumber::ACE_DIAMONDS, CardNumber::KING_SPADES]),
        Two([CardNumber::ACE_DIAMONDS, CardNumber::KING_HEARTS]),
        Two([CardNumber::ACE_DIAMONDS, CardNumber::KING_CLUBS]),
        Two([CardNumber::ACE_CLUBS, CardNumber::KING_SPADES]),
        Two([CardNumber::ACE_CLUBS, CardNumber::KING_HEARTS]),
        Two([CardNumber::ACE_CLUBS, CardNumber::KING_DIAMONDS]),
    ];
    pub const AQs: [Two; 4] = [
        Two([CardNumber::ACE_SPADES, CardNumber::QUEEN_SPADES]),
        Two([CardNumber::ACE_HEARTS, CardNumber::QUEEN_HEARTS]),
        Two([CardNumber::ACE_DIAMONDS, CardNumber::QUEEN_DIAMONDS]),
        Two([CardNumber::ACE_CLUBS, CardNumber::QUEEN_CLUBS]),
    ];
    pub const AQo: [Two; 12] = [
        Two([CardNumber::ACE_SPADES, CardNumber::QUEEN_HEARTS]),
        Two([CardNumber::ACE_SPADES, CardNumber::QUEEN_DIAMONDS]),
        Two([CardNumber::ACE_SPADES, CardNumber::QUEEN_CLUBS]),
        Two([CardNumber::ACE_HEARTS, CardNumber::QUEEN_SPADES]),
        Two([CardNumber::ACE_HEARTS, CardNumber::QUEEN_DIAMONDS]),
        Two([CardNumber::ACE_HEARTS, CardNumber::QUEEN_CLUBS]),
        Two([CardNumber::ACE_DIAMONDS, CardNumber::QUEEN_SPADES]),
        Two([CardNumber::ACE_DIAMONDS, CardNumber::QUEEN_HEARTS]),
        Two([CardNumber::ACE_DIAMONDS, CardNumber::QUEEN_CLUBS]),
        Two([CardNumber::ACE_CLUBS, CardNumber::QUEEN_SPADES]),
        Two([CardNumber::ACE_CLUBS, CardNumber::QUEEN_HEARTS]),
        Two([CardNumber::ACE_CLUBS, CardNumber::QUEEN_DIAMONDS]),
    ];

    //endregion
    #[must_use]
    pub fn new(first: CKCNumber, second: CKCNumber) -> Self {
        Self([first, second])
    }

    fn from_index(index: &str) -> Option<[CKCNumber; 2]> {
        let mut esses = index.split_whitespace();

        let first = CKCNumber::from_index(esses.next()?);
        let second = CKCNumber::from_index(esses.next()?);
        let hand: [CKCNumber; 2] = [first, second];
        Some(hand)
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
    #[allow(clippy::cast_possible_truncation)]
    pub fn chen_formula(&self) -> i8 {
        let high_card = self.high_card();
        let mut points = high_card.get_chen_points();

        if self.is_pocket_pair() {
            points = f32::max(points * 2.0, 5.0);
        } else {
            let gap = self.get_gap();
            points -= match gap {
                1 => 1.0,
                2 => 2.0,
                3 => 4.0,
                0 => 0.0,
                _ => 5.0,
            };

            let top_rank = high_card.get_card_rank() as u8;
            if (gap < 2) && (top_rank < 12u8) {
                points += 1.0;
            }
        }

        if self.is_suited() {
            points += 2.0;
        }

        points.ceil() as i8
    }

    #[must_use]
    pub fn get_gap(&self) -> u8 {
        let s = self.sort();
        let distance_between = s.first().get_card_rank() as u8 - s.second().get_card_rank() as u8;
        if distance_between < 1 {
            0
        } else {
            distance_between - 1
        }
    }

    #[must_use]
    pub fn high_card(&self) -> CKCNumber {
        cmp::max(self.first(), self.second())
    }

    #[must_use]
    pub fn is_connector(&self) -> bool {
        self.get_gap() == 0
    }

    #[must_use]
    pub fn is_pocket_pair(&self) -> bool {
        self.first().get_card_rank() == self.second().get_card_rank()
    }

    #[must_use]
    pub fn is_suited(&self) -> bool {
        self.first().get_card_suit() == self.second().get_card_suit()
    }

    #[must_use]
    pub fn is_suited_connector(&self) -> bool {
        self.is_suited() && self.is_connector()
    }

    //region vs
    //endregion -> Result Preflop <-

    // pub fn types() -> Vec<&str> {
    //     vec![
    //         "A♠ A♥ A♦ A♣",  // EQUALS
    //         "A♠ A♥ A♦ K♦",  // Dominated / Connector / Suited
    //         "A♠ A♥ A♦ K♠",  // Dominated / Partially Covered / Connector / Off
    //         "A♠ A♥ K♠ K♥",  // Dominated / Covered / Connector / Off
    //         "A♠ A♥ K♠ K♥",  // Dominated / Covered / Connector / Off
    //     ]
    // }

    //endregion
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

impl TryFrom<BinaryCard> for Two {
    type Error = HandError;

    fn try_from(binary_card: BinaryCard) -> Result<Self, Self::Error> {
        match binary_card.number_of_cards() {
            0..=1 => Err(HandError::NotEnoughCards),
            2 => {
                let mut bc = binary_card;
                let two = Two::new(
                    CKCNumber::from_binary_card(bc.peel()),
                    CKCNumber::from_binary_card(bc.peel()),
                );
                if two.is_valid() {
                    Ok(two)
                } else {
                    Err(HandError::InvalidBinaryFormat)
                }
            },
            _ => Err(HandError::TooManyCards),
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

impl Shifty for Two {
    fn shift_suit(&self) -> Self {
        Two::new(self.first().shift_suit(), self.second().shift_suit())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cards_two_tests {
    use super::*;
    use crate::CardNumber;
    use rstest::rstest;

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

    #[rstest]
    #[case(20, Two::new(CardNumber::ACE_SPADES, CardNumber::ACE_CLUBS))]
    #[case(12, Two::new(CardNumber::ACE_SPADES, CardNumber::KING_SPADES))]
    #[case(10, Two::new(CardNumber::ACE_SPADES, CardNumber::KING_CLUBS))]
    #[case(16, Two::new(CardNumber::KING_SPADES, CardNumber::KING_CLUBS))]
    #[case(14, Two::new(CardNumber::QUEEN_SPADES, CardNumber::QUEEN_CLUBS))]
    #[case(12, Two::new(CardNumber::JACK_SPADES, CardNumber::JACK_CLUBS))]
    #[case(10, Two::new(CardNumber::TEN_SPADES, CardNumber::TEN_CLUBS))]
    #[case(9, Two::new(CardNumber::JACK_SPADES, CardNumber::TEN_SPADES))]
    #[case(5, Two::new(CardNumber::FIVE_SPADES, CardNumber::ACE_CLUBS))]
    #[case(7, Two::new(CardNumber::FIVE_SPADES, CardNumber::ACE_SPADES))]
    #[case(6, Two::new(CardNumber::FIVE_SPADES, CardNumber::SIX_SPADES))]
    #[case(5, Two::new(CardNumber::TREY_SPADES, CardNumber::TREY_CLUBS))]
    #[case(5, Two::new(CardNumber::DEUCE_SPADES, CardNumber::DEUCE_CLUBS))]
    #[case(-1, Two::new(CardNumber::DEUCE_SPADES, CardNumber::SEVEN_CLUBS))]
    fn chen_formula(#[case] chen_number: i8, #[case] hand: Two) {
        assert_eq!(chen_number, hand.chen_formula());
    }

    #[test]
    fn get_gap() {
        assert_eq!(11, Two::new(CardNumber::DEUCE_CLUBS, CardNumber::ACE_CLUBS).get_gap());
        assert_eq!(11, Two::new(CardNumber::ACE_SPADES, CardNumber::DEUCE_CLUBS).get_gap());
        assert_eq!(0, Two::new(CardNumber::ACE_SPADES, CardNumber::ACE_CLUBS).get_gap());
    }

    #[test]
    fn high_card() {
        let hand = Two::new(CardNumber::ACE_CLUBS, CardNumber::KING_SPADES);

        assert_eq!(hand.high_card(), CardNumber::ACE_CLUBS);
    }

    #[test]
    fn is_connector() {
        assert!(Two::new(CardNumber::ACE_CLUBS, CardNumber::KING_SPADES).is_connector());
        assert!(!Two::new(CardNumber::ACE_CLUBS, CardNumber::DEUCE_CLUBS).is_connector());
    }

    #[test]
    fn is_pocket_pair() {
        assert!(Two::new(CardNumber::ACE_CLUBS, CardNumber::ACE_SPADES).is_pocket_pair());
        assert!(!Two::new(CardNumber::ACE_CLUBS, CardNumber::KING_SPADES).is_pocket_pair());
    }

    #[test]
    fn is_suited() {
        assert!(Two::new(CardNumber::ACE_CLUBS, CardNumber::KING_CLUBS).is_suited());
        assert!(!Two::new(CardNumber::ACE_CLUBS, CardNumber::KING_SPADES).is_suited());
    }

    #[test]
    fn is_suited_connector() {
        assert!(Two::new(CardNumber::ACE_CLUBS, CardNumber::KING_CLUBS).is_suited_connector());
        assert!(Two::new(CardNumber::NINE_CLUBS, CardNumber::EIGHT_CLUBS).is_suited_connector());
        assert!(!Two::new(CardNumber::NINE_CLUBS, CardNumber::EIGHT_DIAMONDS).is_suited_connector());
        assert!(!Two::new(CardNumber::NINE_CLUBS, CardNumber::SEVEN_CLUBS).is_suited_connector());
        assert!(!Two::new(CardNumber::ACE_CLUBS, CardNumber::KING_SPADES).is_suited_connector());
    }

    #[test]
    fn shifty__shift_suit() {
        assert_eq!(
            Two::try_from("AS AD").unwrap().shift_suit(),
            Two::try_from("AH AC").unwrap()
        )
    }

    #[test]
    fn try_from__binary_card() {
        let t = Two::try_from(BinaryCard::ACE_SPADES.fold_in(BinaryCard::ACE_DIAMONDS));
        assert!(t.is_ok());
        assert!(!t.is_err());
        assert_eq!(Two::new(CardNumber::ACE_SPADES, CardNumber::ACE_DIAMONDS), t.unwrap());
    }

    #[test]
    fn try_from__binary_card__not_enough() {
        let t = Two::try_from(BinaryCard::ACE_SPADES);
        assert!(t.is_err());
        assert_eq!(t.unwrap_err(), HandError::NotEnoughCards);
        assert_eq!(Two::try_from(BinaryCard::BLANK).unwrap_err(), HandError::NotEnoughCards);
    }

    #[test]
    fn try_from__binary_card__too_many() {
        let t = Two::try_from(
            BinaryCard::ACE_SPADES
                .fold_in(BinaryCard::ACE_DIAMONDS)
                .fold_in(BinaryCard::ACE_CLUBS),
        );
        assert!(t.is_err());
        assert_eq!(t.unwrap_err(), HandError::TooManyCards);
    }

    #[test]
    fn try_from__binary_card__invalid_binary_format() {
        let bc = 0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000
            .fold_in(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
        let t = Two::try_from(bc);
        assert!(t.is_err());
        assert_eq!(t.unwrap_err(), HandError::InvalidBinaryFormat);
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
