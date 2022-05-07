use crate::cards::five::Five;
use crate::cards::two::Two;
use crate::cards::{HandRanker, HandValidator, Permutator};
use crate::hand_rank::HandRankValue;
use crate::{CKCNumber, HandError, PokerCard};
use core::slice::Iter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Seven([CKCNumber; 7]);

impl Seven {
    /// permutations to evaluate all 7 card combinations.
    pub const FIVE_CARD_PERMUTATIONS: [[u8; 5]; 21] = [
        [0, 1, 2, 3, 4],
        [0, 1, 2, 3, 5],
        [0, 1, 2, 3, 6],
        [0, 1, 2, 4, 5],
        [0, 1, 2, 4, 6],
        [0, 1, 2, 5, 6],
        [0, 1, 3, 4, 5],
        [0, 1, 3, 4, 6],
        [0, 1, 3, 5, 6],
        [0, 1, 4, 5, 6],
        [0, 2, 3, 4, 5],
        [0, 2, 3, 4, 6],
        [0, 2, 3, 5, 6],
        [0, 2, 4, 5, 6],
        [0, 3, 4, 5, 6],
        [1, 2, 3, 4, 5],
        [1, 2, 3, 4, 6],
        [1, 2, 3, 5, 6],
        [1, 2, 4, 5, 6],
        [1, 3, 4, 5, 6],
        [2, 3, 4, 5, 6],
    ];

    #[must_use]
    pub fn new(two: Two, five: Five) -> Self {
        Self([
            two.first(),
            two.second(),
            five.first(),
            five.second(),
            five.third(),
            five.forth(),
            five.fifth(),
        ])
    }

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

    #[must_use]
    pub fn sixth(&self) -> CKCNumber {
        self.0[5]
    }

    #[must_use]
    pub fn seventh(&self) -> CKCNumber {
        self.0[6]
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

    pub fn set_sixth(&mut self, card_number: CKCNumber) {
        self.0[5] = card_number;
    }

    pub fn set_seventh(&mut self, card_number: CKCNumber) {
        self.0[6] = card_number;
    }

    #[must_use]
    pub fn to_arr(&self) -> [CKCNumber; 7] {
        self.0
    }

    //endregion

    fn from_index(index: &str) -> Option<[CKCNumber; 7]> {
        let mut esses = index.split_whitespace();

        let first = CKCNumber::from_index(esses.next()?);
        let second = CKCNumber::from_index(esses.next()?);
        let third = CKCNumber::from_index(esses.next()?);
        let forth = CKCNumber::from_index(esses.next()?);
        let fifth = CKCNumber::from_index(esses.next()?);
        let sixth = CKCNumber::from_index(esses.next()?);
        let seventh = CKCNumber::from_index(esses.next()?);
        let hand: [CKCNumber; 7] = [first, second, third, forth, fifth, sixth, seventh];
        Some(hand)
    }
}

impl From<[CKCNumber; 7]> for Seven {
    fn from(array: [CKCNumber; 7]) -> Self {
        Seven(array)
    }
}

impl HandRanker for Seven {
    fn hand_rank_value_and_hand(&self) -> (HandRankValue, Five) {
        let mut best_hrv: HandRankValue = 0u16;
        let mut best_hand = Five::default();

        for perm in Seven::FIVE_CARD_PERMUTATIONS {
            let hand = self.five_from_permutation(perm);
            let hrv = hand.hand_rank_value();
            if (best_hrv == 0) || hrv != 0 && hrv < best_hrv {
                best_hrv = hrv;
                best_hand = hand;
            }
        }

        (best_hrv, best_hand.sort())
    }

    fn hand_rank_value_validated(&self) -> HandRankValue {
        if !self.is_valid() {
            return crate::hand_rank::NO_HAND_RANK_VALUE;
        }
        self.hand_rank_value()
    }
}

impl HandValidator for Seven {
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

    fn sort(&self) -> Seven {
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

impl Permutator for Seven {
    fn five_from_permutation(&self, permutation: [u8; 5]) -> Five {
        Five::new(
            self.0[permutation[0] as usize],
            self.0[permutation[1] as usize],
            self.0[permutation[2] as usize],
            self.0[permutation[3] as usize],
            self.0[permutation[4] as usize],
        )
    }
}

impl TryFrom<&'static str> for Seven {
    type Error = HandError;

    fn try_from(index: &'static str) -> Result<Self, Self::Error> {
        match Seven::from_index(index) {
            None => Err(HandError::InvalidIndex),
            Some(seven) => Ok(Seven::from(seven)),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cards_seven_tests {
    use super::*;
    use crate::CardNumber;

    #[test]
    fn sort() {
        let seven = Seven::try_from("KC 8C QD A♠ 9h 2C T♠").unwrap().sort();

        let expected = Seven::try_from("A♠ KC QD T♠ 9h 8c 2C").unwrap();

        assert_eq!(seven, expected);
    }

    #[test]
    fn default() {
        let seven = Seven::default();

        assert_eq!(seven.first(), CardNumber::BLANK);
        assert_eq!(seven.second(), CardNumber::BLANK);
        assert_eq!(seven.third(), CardNumber::BLANK);
        assert_eq!(seven.forth(), CardNumber::BLANK);
        assert_eq!(seven.fifth(), CardNumber::BLANK);
        assert_eq!(seven.sixth(), CardNumber::BLANK);
        assert_eq!(seven.seventh(), CardNumber::BLANK);
        assert!(seven.contain_blank());
        assert!(!seven.are_unique());
        assert!(!seven.is_valid());
    }

    #[test]
    fn five_from_permutation() {
        let seven = Seven::try_from("A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠").unwrap();

        assert_eq!(
            seven.five_from_permutation(Seven::FIVE_CARD_PERMUTATIONS[0]),
            Five::try_from("A♠ K♠ Q♠ J♠ T♠").unwrap()
        );
        assert_eq!(
            seven.five_from_permutation(Seven::FIVE_CARD_PERMUTATIONS[1]),
            Five::try_from("A♠ K♠ Q♠ J♠ 9♠").unwrap()
        );
        assert_eq!(
            seven.five_from_permutation(Seven::FIVE_CARD_PERMUTATIONS[2]),
            Five::try_from("A♠ K♠ Q♠ J♠ 8♠").unwrap()
        );
        assert_eq!(
            seven.five_from_permutation(Seven::FIVE_CARD_PERMUTATIONS[3]),
            Five::try_from("A♠ K♠ Q♠ T♠ 9♠").unwrap()
        );
        assert_eq!(
            seven.five_from_permutation(Seven::FIVE_CARD_PERMUTATIONS[4]),
            Five::try_from("A♠ K♠ Q♠ T♠ 8♠").unwrap()
        );
        assert_eq!(
            seven.five_from_permutation(Seven::FIVE_CARD_PERMUTATIONS[5]),
            Five::try_from("A♠ K♠ Q♠ 9♠ 8♠").unwrap()
        );
        assert_eq!(
            seven.five_from_permutation(Seven::FIVE_CARD_PERMUTATIONS[6]),
            Five::try_from("A♠ K♠ J♠ T♠ 9♠").unwrap()
        );
        assert_eq!(
            seven.five_from_permutation(Seven::FIVE_CARD_PERMUTATIONS[7]),
            Five::try_from("A♠ K♠ J♠ T♠ 8♠").unwrap()
        );
        assert_eq!(
            seven.five_from_permutation(Seven::FIVE_CARD_PERMUTATIONS[20]),
            Five::try_from("Q♠ J♠ T♠ 9♠ 8♠").unwrap()
        );
    }

    #[test]
    fn hand_rank_value() {
        assert_eq!(1, Seven::try_from("T♠ A♠ K♠ J♠ Q♠ 9♠ 8♠").unwrap().hand_rank_value());
        assert_eq!(2, Seven::try_from("T♠ 8♠ K♠ J♠ Q♠ 9♠ 7♠").unwrap().hand_rank_value());
        assert_eq!(3, Seven::try_from("T♠ 8♠ 7♠ J♠ 6♠ Q♠ 9♠").unwrap().hand_rank_value());
        assert_eq!(7414, Seven::try_from("9S 8D 7C 5D 4♥ 3D 2D").unwrap().hand_rank_value());
    }

    #[test]
    fn hand_rank_value_and_hand() {
        let (value, hand) = Seven::try_from("T♠ A♠ K♠ J♠ Q♠ 9♠ 8♠")
            .unwrap()
            .hand_rank_value_and_hand();

        assert_eq!(hand, Five::try_from("A♠ K♠ Q♠ J♠ T♠").unwrap());
        assert_eq!(value, 1);
    }

    #[test]
    fn try_from__index() {
        let seven = Seven::try_from("A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠");

        assert!(seven.is_ok());
        let seven = seven.unwrap();
        assert_eq!(seven.first(), CardNumber::ACE_SPADES);
        assert_eq!(seven.second(), CardNumber::KING_SPADES);
        assert_eq!(seven.third(), CardNumber::QUEEN_SPADES);
        assert_eq!(seven.forth(), CardNumber::JACK_SPADES);
        assert_eq!(seven.fifth(), CardNumber::TEN_SPADES);
        assert_eq!(seven.sixth(), CardNumber::NINE_SPADES);
        assert_eq!(seven.seventh(), CardNumber::EIGHT_SPADES);
        assert!(!seven.contain_blank());
        assert!(seven.are_unique());
        assert!(seven.is_valid());
    }

    #[test]
    fn try_from__index__blank() {
        let seven = Seven::try_from("A♠ K♠ XX J♠ T♠ 9♠ 8♠");

        assert!(seven.is_ok());
        let seven = seven.unwrap();
        assert_eq!(seven.first(), CardNumber::ACE_SPADES);
        assert_eq!(seven.second(), CardNumber::KING_SPADES);
        assert_eq!(seven.third(), CardNumber::BLANK);
        assert_eq!(seven.forth(), CardNumber::JACK_SPADES);
        assert_eq!(seven.fifth(), CardNumber::TEN_SPADES);
        assert_eq!(seven.sixth(), CardNumber::NINE_SPADES);
        assert_eq!(seven.seventh(), CardNumber::EIGHT_SPADES);
        assert!(seven.contain_blank());
        assert!(!seven.is_valid());
    }

    #[test]
    fn try_from__index__too_short() {
        let seven = Seven::try_from("A♠ K♠ Q♠ J♠ T♠");

        assert!(seven.is_err());
    }
}
