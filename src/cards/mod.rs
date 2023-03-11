use crate::cards::five::Five;
use crate::{CKCNumber, CardNumber};
use core::slice::Iter;

pub mod binary_card;
pub mod five;
pub mod four;
pub mod seven;
pub mod six;
pub mod three;
pub mod two;

pub trait HandRanker {
    fn hand_rank(&self) -> crate::hand_rank::HandRank {
        crate::hand_rank::HandRank::from(self.hand_rank_value())
    }

    fn hand_rank_validated(&self) -> crate::hand_rank::HandRank {
        crate::hand_rank::HandRank::from(self.hand_rank_value_validated())
    }

    fn hand_rank_value(&self) -> crate::hand_rank::HandRankValue {
        let (hrv, _) = self.hand_rank_value_and_hand();
        hrv
    }

    fn hand_rank_value_and_hand(&self) -> (crate::hand_rank::HandRankValue, Five);

    fn hand_rank_value_validated(&self) -> crate::hand_rank::HandRankValue;
}

pub trait HandValidator {
    fn are_unique(&self) -> bool;

    fn first(&self) -> CKCNumber;

    #[must_use]
    fn sort(&self) -> Self;

    fn sort_in_place(&mut self);

    fn contain_blank(&self) -> bool {
        self.iter().any(|c| c == &CardNumber::BLANK)
    }

    /// A corrupt hand is one where any of the values in the array doesn't correspond to any
    /// recognized `CardNumber` or is blank.
    fn is_corrupt(&self) -> bool {
        self.iter().any(|c| CardNumber::filter(*c) == CardNumber::BLANK)
    }

    fn is_valid(&self) -> bool {
        self.are_unique() && !self.is_corrupt()
    }

    fn iter(&self) -> Iter<'_, CKCNumber>;
}

pub trait Permutator {
    fn five_from_permutation(&self, permutation: [u8; 5]) -> Five;
}
