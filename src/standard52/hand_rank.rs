use crate::standard52::hand_rank_class::HandRankClass;
use crate::standard52::hand_rank_name::HandRankName;
use core::cmp::Ordering;
use core::fmt::{Display, Formatter};

// https://en.wikipedia.org/wiki/Se%C3%B1or_Wences#Catchphrases
/// The more I think about this, the more I feel like this is me avoiding the best practice
/// of returning `Result` and `Option`. I'm worried about speed, but that's probably Knuth's
/// dreaded [premature optimization](http://wiki.c2.com/?PrematureOptimization).
pub trait SOK {
    fn salright(&self) -> bool;
}

/// `HandRankValue` is the integer representing the `HandRank` for a particular five card
/// `PokerHand`. This value is used to compare one hand against the other, the lower the value,
/// the stronger the hand in a traditional, highest to lowest, ranking. A `HandRankValue` can have
/// only one `HandRankName` and `HandRankClass`.
#[allow(clippy::module_name_repetitions)]
pub type HandRankValue = u16;

pub const NO_HAND_RANK_VALUE: HandRankValue = 0;

/// `HandRank` represents the value of a specific 5 card hand of poker. The lower the
/// `HandRankValue` the better the hand. When a `HandRank` is instantiated it can only
/// have a specific matching `HandRankName` and `HandRankValue`.
///
/// # REFACTORING
///
/// Remove assessors; make fields public.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HandRank {
    pub value: HandRankValue,
    pub name: HandRankName,
    pub class: HandRankClass,
}

impl Display for HandRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {:?}", self.value, self.class)
    }
}

impl From<HandRankValue> for HandRank {
    fn from(value: HandRankValue) -> Self {
        let hr = HandRank {
            value,
            name: HandRankName::from(value),
            class: HandRankClass::from(value),
        };

        if !hr.salright() {
            return HandRank::default();
        }

        hr
    }
}

/// The lower the `HandRankValue` the higher the value of the `HandRank`, unless it's invalid.
#[allow(clippy::if_same_then_else)]
impl Ord for HandRank {
    fn cmp(&self, other: &HandRank) -> Ordering {
        if !self.salright() && !other.salright() {
            Ordering::Equal
        } else if !self.salright() {
            Ordering::Less
        } else if !other.salright() {
            Ordering::Greater
        } else if self.value < other.value {
            Ordering::Greater
        } else if self.value > other.value {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd<Self> for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SOK for HandRank {
    fn salright(&self) -> bool {
        self.name.salright() && self.class.salright()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod hand_rank_tests {
    use super::*;

    #[test]
    fn default() {
        let default = HandRank::default();

        assert_eq!(default.value, 0);
        assert_eq!(default.name, HandRankName::Invalid);
        assert_eq!(default.class, HandRankClass::None);
    }

    #[test]
    fn from() {
        assert!(HandRank::from(1).salright());
        assert!(HandRank::from(7462).salright());
        assert!(!HandRank::from(0).salright());
        assert!(!HandRank::from(7463).salright());
    }

    #[test]
    fn ord() {
        assert!(HandRank::from(1) > HandRank::from(2));
        assert!(HandRank::from(2000) < HandRank::from(2));
        assert!(HandRank::from(0) < HandRank::from(2));
        assert_eq!(HandRank::from(2), HandRank::from(2));
    }

    #[test]
    fn lower_value_is_the_stronger_hand() {
        let royal = HandRank::from(1);
        let steel_wheel = HandRank::from(10);
        assert!(royal > steel_wheel, "rank 1 must beat rank 10");
    }

    #[test]
    fn invalid_ranks_sort_below_everything() {
        let invalid = HandRank::from(0);
        let worst_real = HandRank::from(7462);
        assert!(worst_real > invalid);
        assert!(!invalid.salright());
    }

    #[test]
    fn every_valid_value_yields_a_name_and_class() {
        for v in 1..=7462u16 {
            let hr = HandRank::from(v);
            assert!(hr.salright(), "rank {v} must be valid");
            assert_eq!(hr.value, v);
            assert_ne!(hr.name, HandRankName::default(), "rank {v} has no name");
            assert_ne!(hr.class, HandRankClass::default(), "rank {v} has no class");
        }
    }

    #[test]
    fn out_of_range_values_are_rejected() {
        assert!(!HandRank::from(0).salright());
        assert!(!HandRank::from(7463).salright());
        assert!(!HandRank::from(u16::MAX).salright());
    }
}
