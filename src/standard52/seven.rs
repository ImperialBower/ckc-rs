use crate::CkcError;
use crate::standard52::arrays::{HandRanker, HandValidator, impl_hand_ranker_sort_and_permutation, parse_hand};
use crate::standard52::card::Card;
use crate::standard52::five::Five;
use crate::standard52::hand_rank::{HandRankValue, NO_HAND_RANK_VALUE};
use core::fmt::{self, Display, Formatter};
use core::slice::Iter;
use core::str::FromStr;

#[cfg(feature = "alloc")]
use crate::standard52::arrays::collect_hand;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Seven([Card; 7]);

impl Seven {
    /// permutations to evaluate all 7 card combinations.
    pub const FIVE_CARD_PERMUTATIONS: [[usize; 5]; 21] = [
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
    pub fn to_arr(&self) -> [Card; 7] {
        self.0
    }
}

impl Display for Seven {
    /// Renders the hand exactly as pkcore does.
    ///
    /// pkcore formats a `Seven` as `self.cards()`, and `Pile::cards()` builds a
    /// `Cards(IndexSet<Card>)` that **drops blanks and keeps only the first occurrence of
    /// a repeated card** before joining the survivors with a single space. Both steps are
    /// reproduced here, so `Seven::default()` renders as the empty string — pkcore's own
    /// test asserted `Seven::default().cards().len() == 0`.
    ///
    /// Same two-layer rewrite as [`Five`]'s `Display`; see that impl for the full
    /// derivation. Unlike pkcore's version, which collects a `Vec<String>` to join, this
    /// allocates nothing and so stays available without the `alloc` feature.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut wrote = false;
        for (i, card) in self.0.iter().enumerate() {
            if *card == Card::BLANK || self.0[..i].contains(card) {
                continue;
            }
            if wrote {
                write!(f, " ")?;
            }
            write!(f, "{card}")?;
            wrote = true;
        }
        Ok(())
    }
}

impl From<[Card; 7]> for Seven {
    fn from(array: [Card; 7]) -> Self {
        Seven(array)
    }
}

impl HandValidator for Seven {
    fn are_unique(&self) -> bool {
        !(1..7).any(|i| self.0[i..].contains(&self.0[i - 1]))
    }

    fn first(&self) -> Card {
        self.0[0]
    }

    fn iter(&self) -> Iter<'_, Card> {
        self.0.iter()
    }
}

impl HandRanker for Seven {
    fn hand_rank_value(&self) -> HandRankValue {
        let mut best_hrv = NO_HAND_RANK_VALUE;

        for perm in Seven::FIVE_CARD_PERMUTATIONS {
            let hand = self.five_from_permutation(perm);
            let hrv = hand.hand_rank_value();
            if (best_hrv == 0) || hrv != 0 && hrv < best_hrv {
                best_hrv = hrv;
            }
        }
        best_hrv
    }

    fn hand_rank_value_and_hand(&self) -> (HandRankValue, Five) {
        let mut best_hrv: HandRankValue = NO_HAND_RANK_VALUE;
        let mut best_hand = Five::default();

        for perm in Seven::FIVE_CARD_PERMUTATIONS {
            let hand = self.five_from_permutation(perm);
            let hrv = hand.hand_rank_value();
            if (best_hrv == 0) || hrv != 0 && hrv < best_hrv {
                best_hrv = hrv;
                best_hand = hand;
            }
        }

        (best_hrv, best_hand.sort().clean())
    }

    impl_hand_ranker_sort_and_permutation!();
}

impl FromStr for Seven {
    type Err = CkcError;

    /// Rewritten for the kernel: pkcore delegated to `Cards::from_str` and
    /// `Seven::try_from(Cards)`, neither of which follows the kernel down. The observable
    /// behaviour is preserved by the shared `parse_hand`, which `Five` and `Six` also
    /// use — `,` and `-` are treated as separators, every token must parse as a `Card`,
    /// duplicates collapse, and exactly seven distinct cards must remain.
    ///
    /// # Errors
    ///
    /// `CkcError::InvalidIndex` if a token is not a card, or if there are no tokens at all;
    /// `CkcError::Incomplete` for fewer than seven distinct cards;
    /// `CkcError::InvalidCardCount` for more than seven.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Seven(parse_hand::<7>(s)?))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<Vec<Card>> for Seven {
    type Error = CkcError;

    /// pkcore was `Seven::try_from(Cards::from(vec))`; `Cards::from(Vec<Card>)`
    /// (`pkcore/src/cards.rs:856-863`) drops blanks and collapses duplicates on the way
    /// into its `IndexSet`, and the `TryFrom<Cards>` impl then demanded exactly seven.
    /// `collect_hand` reproduces both layers without allocating.
    ///
    /// # Errors
    ///
    /// `CkcError::Incomplete` for fewer than seven distinct non-blank cards;
    /// `CkcError::InvalidCardCount` for more.
    fn try_from(vec: Vec<Card>) -> Result<Self, Self::Error> {
        Ok(Seven(collect_hand::<7, _>(vec.into_iter())?))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod arrays__seven_tests {
    use super::*;
    use crate::standard52::hand_rank_class::HandRankClass;
    use crate::standard52::hand_rank_name::HandRankName;

    #[cfg(feature = "alloc")]
    use alloc::string::ToString;

    const CARDS: [Card; 7] = [
        Card::ACE_DIAMONDS,
        Card::SIX_SPADES,
        Card::FOUR_SPADES,
        Card::ACE_SPADES,
        Card::FIVE_DIAMONDS,
        Card::TREY_CLUBS,
        Card::DEUCE_SPADES,
    ];

    #[cfg(feature = "alloc")]
    #[test]
    fn display() {
        assert_eq!("A♦ 6♠ 4♠ A♠ 5♦ 3♣ 2♠", Seven(CARDS).to_string());
    }

    /// Replaces pkcore's `cards()` test, which asserted
    /// `Seven::default().cards().len() == 0`. `Cards` does not follow the kernel down, but
    /// the blank-dropping it performed is now inside `Display`, so the same property is
    /// asserted through the only surface that still exposes it.
    #[cfg(feature = "alloc")]
    #[test]
    fn display__drops_blanks_and_collapses_duplicates() {
        assert_eq!("", Seven::default().to_string());
        assert_eq!(
            "A♠ K♠ Q♠",
            Seven::from([
                Card::ACE_SPADES,
                Card::KING_SPADES,
                Card::ACE_SPADES,
                Card::QUEEN_SPADES,
                Card::BLANK,
                Card::KING_SPADES,
                Card::BLANK,
            ])
            .to_string()
        );
        assert_eq!(
            "A♠",
            Seven::from([
                Card::BLANK,
                Card::ACE_SPADES,
                Card::BLANK,
                Card::BLANK,
                Card::BLANK,
                Card::BLANK,
                Card::BLANK,
            ])
            .to_string()
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(Seven::from_str("A♦ 6♠ 4♠ A♠ 5♦ 3♣ 2♠").unwrap(), Seven::from(CARDS));
        assert_eq!(Seven::from_str("AD 2D 3D 4D 5d").unwrap_err(), CkcError::Incomplete);
        assert_eq!(
            Seven::from_str("AD 2D 3D 4D 5d 6d 7d 8d").unwrap_err(),
            CkcError::InvalidCardCount
        );
    }

    /// The `Cards`-shaped behaviour pkcore's `TryFrom<Cards>` tests covered, asserted
    /// through `FromStr` — the only route into a `Seven` that still performs it.
    #[test]
    fn from_str__separators_duplicates_and_garbage() {
        assert_eq!(Seven::from_str("A♦,6♠-4♠ A♠ 5♦ 3♣ 2♠").unwrap(), Seven::from(CARDS));
        assert_eq!(
            Seven::from_str("A♦ 6♠ 6♠ 4♠ A♠ 5♦ 3♣ 2♠").unwrap(),
            Seven::from(CARDS),
            "duplicates collapse to their first occurrence"
        );
        assert_eq!(
            Seven::from_str("A♦ 6♠ 4♠ A♠ 5♦ 3♣ XX").unwrap_err(),
            CkcError::InvalidIndex
        );
        assert_eq!(Seven::from_str("").unwrap_err(), CkcError::InvalidIndex);
    }

    #[test]
    fn five_from_permutation() {
        assert_eq!(
            Five::from_str("AD 6S 4S AS 5D").unwrap(),
            Seven::from(CARDS).five_from_permutation(Seven::FIVE_CARD_PERMUTATIONS[0])
        );
    }

    #[test]
    fn hand_rank() {
        let (hr, best) = Seven::from(CARDS).hand_rank_and_hand();
        assert_eq!(1608, hr.value);
        assert_eq!(HandRankClass::SixHighStraight, hr.class);
        assert_eq!(HandRankName::Straight, hr.name);
        assert_eq!(Five::from_str("6S 5D 4S 3C 2S").unwrap(), best);
    }

    /// Added by the kernel: pkcore had no `HandValidator`, so these impls are new code and
    /// need their own coverage.
    #[test]
    fn hand_validator() {
        let seven = Seven::from(CARDS);
        assert!(seven.are_unique());
        assert!(!seven.contains_blank());
        assert!(!seven.is_corrupt());
        assert!(seven.is_valid());
        assert_eq!(Card::ACE_DIAMONDS, HandValidator::first(&seven));
        assert_eq!(7, seven.iter().count());

        let mut dupe = CARDS;
        dupe[6] = Card::ACE_DIAMONDS;
        let dupe = Seven::from(dupe);
        assert!(!dupe.are_unique());
        assert!(!dupe.is_valid());

        assert!(Seven::default().contains_blank());
        assert!(Seven::default().is_corrupt());
        assert!(!Seven::default().is_valid());
    }

    /// Added by the kernel: `TryFrom<Vec<Card>>` is a rewrite (pkcore routed it through
    /// `Cards`), and pkcore had no test for it.
    #[cfg(feature = "alloc")]
    #[test]
    fn try_from__vec() {
        use alloc::vec;

        assert_eq!(Seven::try_from(CARDS.to_vec()).unwrap(), Seven::from(CARDS));
        assert_eq!(
            Seven::try_from(vec![Card::ACE_DIAMONDS; 7]).unwrap_err(),
            CkcError::Incomplete,
            "duplicates collapse, leaving one card"
        );
        assert_eq!(Seven::try_from(vec![Card::BLANK; 7]).unwrap_err(), CkcError::Incomplete);
        let mut too_many = CARDS.to_vec();
        too_many.push(Card::KING_SPADES);
        assert_eq!(Seven::try_from(too_many).unwrap_err(), CkcError::InvalidCardCount);
    }
}
