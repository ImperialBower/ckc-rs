use crate::CkcError;
use crate::standard52::arrays::{HandRanker, HandValidator, impl_hand_ranker_sort_and_permutation, parse_hand};
use crate::standard52::card::Card;
use crate::standard52::five::Five;
use crate::standard52::hand_rank::{HandRankValue, NO_HAND_RANK_VALUE};
use core::fmt::{self, Display, Formatter};
use core::slice::Iter;
use core::str::FromStr;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Six([Card; 6]);

impl Six {
    /// permutations to evaluate all 6 card combinations.
    pub const FIVE_CARD_PERMUTATIONS: [[usize; 5]; 6] = [
        [0, 1, 2, 3, 4],
        [0, 1, 2, 3, 5],
        [0, 1, 2, 4, 5],
        [0, 1, 3, 4, 5],
        [0, 2, 3, 4, 5],
        [1, 2, 3, 4, 5],
    ];

    //region accessors
    #[must_use]
    pub fn first(&self) -> Card {
        self.0[0]
    }

    #[must_use]
    pub fn second(&self) -> Card {
        self.0[1]
    }

    #[must_use]
    pub fn third(&self) -> Card {
        self.0[2]
    }

    #[must_use]
    pub fn forth(&self) -> Card {
        self.0[3]
    }

    #[must_use]
    pub fn fifth(&self) -> Card {
        self.0[4]
    }

    #[must_use]
    pub fn sixth(&self) -> Card {
        self.0[5]
    }

    #[must_use]
    pub fn to_arr(&self) -> [Card; 6] {
        self.0
    }
    //endregion
}

impl Display for Six {
    /// Renders the hand exactly as pkcore does.
    ///
    /// pkcore formats a `Six` as `self.cards()`, and `Pile::cards()` builds a
    /// `Cards(IndexSet<Card>)` that **drops blanks and keeps only the first occurrence of
    /// a repeated card** before joining the survivors with a single space. Both steps are
    /// reproduced here, so `Six::default()` renders as the empty string rather than
    /// `"__ __ __ __ __ __"` — pkcore's own test asserted `Six::default().cards().len() == 0`.
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

impl From<[Card; 6]> for Six {
    fn from(array: [Card; 6]) -> Self {
        Six(array)
    }
}

impl HandValidator for Six {
    fn are_unique(&self) -> bool {
        !(1..6).any(|i| self.0[i..].contains(&self.0[i - 1]))
    }

    fn first(&self) -> Card {
        self.0[0]
    }

    fn iter(&self) -> Iter<'_, Card> {
        self.0.iter()
    }
}

impl HandRanker for Six {
    fn hand_rank_value(&self) -> HandRankValue {
        let mut best_hrv = NO_HAND_RANK_VALUE;

        for perm in Six::FIVE_CARD_PERMUTATIONS {
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

        for perm in Six::FIVE_CARD_PERMUTATIONS {
            let hand = self.five_from_permutation(perm);
            let hrv = hand.hand_rank_value();
            if (best_hrv == 0) || hrv != 0 && hrv < best_hrv {
                best_hrv = hrv;
                best_hand = hand;
            }
        }

        (best_hrv, best_hand.sort())
    }

    impl_hand_ranker_sort_and_permutation!();
}

impl FromStr for Six {
    type Err = CkcError;

    /// Rewritten for the kernel: pkcore delegated to `Cards::from_str` and
    /// `Six::try_from(Cards)`, neither of which follows the kernel down. The observable
    /// behaviour is preserved by the shared `parse_hand`, which `Five` and `Seven` also
    /// use — `,` and `-` are treated as separators, every token must parse as a `Card`,
    /// duplicates collapse, and exactly six distinct cards must remain.
    ///
    /// # Errors
    ///
    /// `CkcError::InvalidIndex` if a token is not a card, or if there are no tokens at all;
    /// `CkcError::Incomplete` for fewer than six distinct cards; `CkcError::InvalidCardCount`
    /// for more than six.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Six(parse_hand::<6>(s)?))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod arrays__six_tests {
    use super::*;
    use crate::standard52::hand_rank_class::HandRankClass;
    use crate::standard52::hand_rank_name::HandRankName;

    #[cfg(feature = "alloc")]
    use alloc::string::ToString;

    const CARDS: [Card; 6] = [
        Card::ACE_DIAMONDS,
        Card::DEUCE_DIAMONDS,
        Card::TREY_DIAMONDS,
        Card::FOUR_DIAMONDS,
        Card::FIVE_DIAMONDS,
        Card::SIX_DIAMONDS,
    ];

    #[cfg(feature = "alloc")]
    #[test]
    fn display() {
        assert_eq!("A♦ 2♦ 3♦ 4♦ 5♦ 6♦", Six(CARDS).to_string());
    }

    /// Replaces pkcore's `cards()` test, which asserted
    /// `Six::default().cards().len() == 0`. `Cards` does not follow the kernel down, but
    /// the blank-dropping it performed is now inside `Display`, so the same property is
    /// asserted through the only surface that still exposes it.
    #[cfg(feature = "alloc")]
    #[test]
    fn display__drops_blanks_and_collapses_duplicates() {
        assert_eq!("", Six::default().to_string());
        assert_eq!(
            "A♠ K♠ Q♠",
            Six::from([
                Card::ACE_SPADES,
                Card::KING_SPADES,
                Card::ACE_SPADES,
                Card::QUEEN_SPADES,
                Card::BLANK,
                Card::KING_SPADES,
            ])
            .to_string()
        );
        assert_eq!(
            "A♠",
            Six::from([
                Card::BLANK,
                Card::ACE_SPADES,
                Card::BLANK,
                Card::BLANK,
                Card::BLANK,
                Card::BLANK,
            ])
            .to_string()
        );
    }

    #[test]
    fn from__array() {
        assert_eq!(Six::from(CARDS).0, CARDS);
    }

    #[test]
    fn from_str() {
        assert_eq!(Six::from_str("AD 2D 3D 4D 5d 6d").unwrap(), Six::from(CARDS));
        assert_eq!(Six::from_str("AD 2D 3D 4D 5d").unwrap_err(), CkcError::Incomplete);
        assert_eq!(
            Six::from_str("AD 2D 3D 4D 5d 6d 7d").unwrap_err(),
            CkcError::InvalidCardCount
        );
    }

    /// The `Cards`-shaped behaviour pkcore's `TryFrom<Cards>` tests covered, asserted
    /// through `FromStr` — the only route into a `Six` that still performs it.
    #[test]
    fn from_str__separators_duplicates_and_garbage() {
        assert_eq!(Six::from_str("AD,2D-3D 4D 5d 6d").unwrap(), Six::from(CARDS));
        assert_eq!(
            Six::from_str("AD 2D 2D 3D 4D 5d 6d").unwrap(),
            Six::from(CARDS),
            "duplicates collapse to their first occurrence"
        );
        assert_eq!(Six::from_str("AD 2D 3D 4D 5d XX").unwrap_err(), CkcError::InvalidIndex);
        assert_eq!(Six::from_str("").unwrap_err(), CkcError::InvalidIndex);
    }

    #[test]
    fn five_from_permutation() {
        assert_eq!(
            Five::from_str("AD 2D 3D 4D 5d").unwrap(),
            Six::from(CARDS).five_from_permutation(Six::FIVE_CARD_PERMUTATIONS[0])
        );
    }

    #[test]
    fn hand_rank() {
        let (hr, best) = Six::from(CARDS).hand_rank_and_hand();
        assert_eq!(9, hr.value);
        assert_eq!(HandRankClass::SixHighStraightFlush, hr.class);
        assert_eq!(HandRankName::StraightFlush, hr.name);
        assert_eq!(Five::from_str("6d 5D 4D 3D 2d").unwrap(), best);
    }

    #[test]
    fn sort() {
        assert_eq!(Six::from_str("Ad 6d 5D 4D 3D 2d").unwrap(), Six::from(CARDS).sort());
    }

    /// Added by the kernel: pkcore had no `HandValidator`, so these impls are new code and
    /// need their own coverage.
    #[test]
    fn hand_validator() {
        let six = Six::from(CARDS);
        assert!(six.are_unique());
        assert!(!six.contains_blank());
        assert!(!six.is_corrupt());
        assert!(six.is_valid());
        assert_eq!(Card::ACE_DIAMONDS, HandValidator::first(&six));
        assert_eq!(6, six.iter().count());

        let dupe = Six::from([
            Card::ACE_DIAMONDS,
            Card::ACE_DIAMONDS,
            Card::TREY_DIAMONDS,
            Card::FOUR_DIAMONDS,
            Card::FIVE_DIAMONDS,
            Card::SIX_DIAMONDS,
        ]);
        assert!(!dupe.are_unique());
        assert!(!dupe.is_valid());

        assert!(Six::default().contains_blank());
        assert!(Six::default().is_corrupt());
        assert!(!Six::default().is_valid());
    }
}
