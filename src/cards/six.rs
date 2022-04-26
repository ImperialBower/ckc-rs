use crate::cards::five::Five;
use crate::cards::three::Three;
use crate::cards::two::Two;
use crate::cards::{HandRanker, HandValidator, Permutator};
use crate::hand_rank::HandRankValue;
use crate::{CKCNumber, HandError, PokerCard};
use core::slice::Iter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Six([CKCNumber; 6]);

impl Six {
    /// permutations to evaluate all 6 card combinations.
    pub const FIVE_CARD_PERMUTATIONS: [[u8; 5]; 6] = [
        [0, 1, 2, 3, 4],
        [0, 1, 2, 3, 5],
        [0, 1, 2, 4, 5],
        [0, 1, 3, 4, 5],
        [0, 2, 3, 4, 5],
        [1, 2, 3, 4, 5],
    ];

    #[must_use]
    pub fn from_1_and_2_and_3(one: CKCNumber, two: Two, three: Three) -> Self {
        Self::from([
            one,
            two.first(),
            two.second(),
            three.first(),
            three.second(),
            three.third(),
        ])
    }
    //
    // #[must_use]
    // pub fn five_from_permutation(&self, permutation: [u8; 5]) -> Five {
    //     Five::new(
    //         self.0[permutation[0] as usize],
    //         self.0[permutation[1] as usize],
    //         self.0[permutation[2] as usize],
    //         self.0[permutation[3] as usize],
    //         self.0[permutation[4] as usize],
    //     )
    // }

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

    #[must_use]
    pub fn to_arr(&self) -> [CKCNumber; 6] {
        self.0
    }

    //endregion

    fn from_index(index: &str) -> Option<[CKCNumber; 6]> {
        let mut esses = index.split_whitespace();

        let first = CKCNumber::from_index(esses.next()?);
        let second = CKCNumber::from_index(esses.next()?);
        let third = CKCNumber::from_index(esses.next()?);
        let forth = CKCNumber::from_index(esses.next()?);
        let fifth = CKCNumber::from_index(esses.next()?);
        let sixth = CKCNumber::from_index(esses.next()?);
        let hand: [CKCNumber; 6] = [first, second, third, forth, fifth, sixth];
        Some(hand)
    }
}

impl From<[CKCNumber; 6]> for Six {
    fn from(array: [CKCNumber; 6]) -> Self {
        Six(array)
    }
}

impl HandRanker for Six {
    fn hand_rank_value(&self) -> HandRankValue {
        let mut best_hrv: HandRankValue = 0u16;

        for perm in Six::FIVE_CARD_PERMUTATIONS {
            let hrv = self.five_from_permutation(perm).hand_rank_value();
            if (best_hrv == 0) || hrv != 0 && hrv < best_hrv {
                best_hrv = hrv;
            }
        }

        best_hrv
    }

    fn hand_rank_value_validated(&self) -> HandRankValue {
        if !self.is_valid() {
            return crate::hand_rank::NO_HAND_RANK_VALUE;
        }
        self.hand_rank_value()
    }
}

impl HandValidator for Six {
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

    fn sort(&self) -> Six {
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

impl Permutator for Six {
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

impl TryFrom<&'static str> for Six {
    type Error = HandError;

    fn try_from(index: &'static str) -> Result<Self, Self::Error> {
        match Six::from_index(index) {
            None => Err(HandError::InvalidIndex),
            Some(six) => Ok(Six::from(six)),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cards_six_tests {
    use super::*;
    use crate::CardNumber;

    #[test]
    fn five_from_permutation() {
        let six = Six::try_from("A♠ K♠ Q♠ J♠ T♠ 9♠").unwrap();

        assert_eq!(
            six.five_from_permutation(Six::FIVE_CARD_PERMUTATIONS[0]),
            Five::try_from("A♠ K♠ Q♠ J♠ T♠").unwrap()
        );
        assert_eq!(
            six.five_from_permutation(Six::FIVE_CARD_PERMUTATIONS[1]),
            Five::try_from("A♠ K♠ Q♠ J♠ 9♠").unwrap()
        );
        assert_eq!(
            six.five_from_permutation(Six::FIVE_CARD_PERMUTATIONS[2]),
            Five::try_from("A♠ K♠ Q♠ T♠ 9♠").unwrap()
        );
        assert_eq!(
            six.five_from_permutation(Six::FIVE_CARD_PERMUTATIONS[3]),
            Five::try_from("A♠ K♠ J♠ T♠ 9♠").unwrap()
        );
        assert_eq!(
            six.five_from_permutation(Six::FIVE_CARD_PERMUTATIONS[4]),
            Five::try_from("A♠ Q♠ J♠ T♠ 9♠").unwrap()
        );
        assert_eq!(
            six.five_from_permutation(Six::FIVE_CARD_PERMUTATIONS[5]),
            Five::try_from("K♠ Q♠ J♠ T♠ 9♠").unwrap()
        );
    }

    #[test]
    fn hand_rank_value() {
        assert_eq!(1, Six::try_from("T♠ A♠ K♠ J♠ Q♠ 9♠").unwrap().hand_rank_value());
        assert_eq!(2, Six::try_from("T♠ 8♠ K♠ J♠ Q♠ 9♠").unwrap().hand_rank_value());
        assert_eq!(3, Six::try_from("T♠ 8♠ 7♠ J♠ Q♠ 9♠").unwrap().hand_rank_value());
        assert_eq!(7450, Six::try_from("8D 7C 5D 4♥ 3D 2D").unwrap().hand_rank_value());
    }

    #[test]
    fn hand_rank_value_validated() {
        assert_eq!(
            0,
            Six::try_from("8D 8D 5D 4♥ 3D 2D").unwrap().hand_rank_value_validated()
        );
    }

    #[test]
    fn sort() {
        let six = Six::try_from("KC 8C QD A♠ 9h T♠").unwrap().sort();

        let expected = Six::try_from("A♠ KC QD T♠ 9h 8c").unwrap();

        assert_eq!(six, expected);
    }

    #[test]
    fn default() {
        let six = Six::default();

        assert_eq!(six.first(), CardNumber::BLANK);
        assert_eq!(six.second(), CardNumber::BLANK);
        assert_eq!(six.third(), CardNumber::BLANK);
        assert_eq!(six.forth(), CardNumber::BLANK);
        assert_eq!(six.fifth(), CardNumber::BLANK);
        assert_eq!(six.sixth(), CardNumber::BLANK);
        assert!(six.contain_blank());
        assert!(!six.are_unique());
        assert!(!six.is_valid());
    }

    #[test]
    fn try_from__index() {
        let six = Six::try_from("A♠ K♠ Q♠ J♠ T♠ 9♠");

        assert!(six.is_ok());
        let six = six.unwrap();
        assert_eq!(six.first(), CardNumber::ACE_SPADES);
        assert_eq!(six.second(), CardNumber::KING_SPADES);
        assert_eq!(six.third(), CardNumber::QUEEN_SPADES);
        assert_eq!(six.forth(), CardNumber::JACK_SPADES);
        assert_eq!(six.fifth(), CardNumber::TEN_SPADES);
        assert_eq!(six.sixth(), CardNumber::NINE_SPADES);
        assert!(!six.contain_blank());
        assert!(six.are_unique());
        assert!(six.is_valid());
    }

    #[test]
    fn try_from__index__blank() {
        let six = Six::try_from("A♠ K♠ XX J♠ T♠ 9♠");

        assert!(six.is_ok());
        let six = six.unwrap();
        assert_eq!(six.first(), CardNumber::ACE_SPADES);
        assert_eq!(six.second(), CardNumber::KING_SPADES);
        assert_eq!(six.third(), CardNumber::BLANK);
        assert_eq!(six.forth(), CardNumber::JACK_SPADES);
        assert_eq!(six.fifth(), CardNumber::TEN_SPADES);
        assert_eq!(six.sixth(), CardNumber::NINE_SPADES);
        assert!(six.contain_blank());
        assert!(!six.is_valid());
    }

    #[test]
    fn try_from__index__too_short() {
        let six = Six::try_from("A♠ K♠ Q♠ J♠ T♠");

        assert!(six.is_err());
    }
}
