//! The traits shared by the fixed-size card arrays (`Five`, `Six` and `Seven`), the
//! macro that implements the parts of `HandRanker` those last two have in common, and
//! the shared hand parser/collector.
//!
//! `HandRanker` is the poker half of pkcore's version (`pkcore/src/arrays/mod.rs:51`);
//! the Razz methods and `eval()` deliberately stay in pkcore.
//!
//! `HandValidator` is revived from ckc-rs 0.1 (`src/cards/mod.rs:32-57`), because
//! pkcore has no equivalent: its validity check is `Pile::is_dealt()`, and `Pile`
//! cannot follow the kernel down.

use crate::CkcError;
use crate::standard52::card::Card;
use crate::standard52::card_number::CardNumber;
use crate::standard52::five::Five;
use crate::standard52::hand_rank::{HandRank, HandRankValue};
use core::cmp::Ordering;
use core::str::FromStr;

/// Implements the three `HandRanker` methods that are identical across `Six` and `Seven`:
/// `five_from_permutation`, `sort`, and `sort_in_place`.
///
/// `Five` is excluded because its `sort_in_place` has special wheel handling.
///
/// Moved verbatim from `pkcore/src/arrays/mod.rs:10-32`.
macro_rules! impl_hand_ranker_sort_and_permutation {
    () => {
        fn five_from_permutation(&self, permutation: [usize; 5]) -> Five {
            Five::from([
                self.0[permutation[0]],
                self.0[permutation[1]],
                self.0[permutation[2]],
                self.0[permutation[3]],
                self.0[permutation[4]],
            ])
        }

        fn sort(&self) -> Self {
            let mut copy = *self;
            copy.sort_in_place();
            copy
        }

        fn sort_in_place(&mut self) {
            self.0.sort_unstable();
            self.0.reverse();
        }
    };
}

pub(crate) use impl_hand_ranker_sort_and_permutation;

/// Returns a `HandRank` for a collection of five or more cards.
pub trait HandRanker {
    fn hand_rank(&self) -> HandRank {
        HandRank::from(self.hand_rank_value())
    }

    fn hand_rank_and_hand(&self) -> (HandRank, Five) {
        let (hrv, hand) = self.hand_rank_value_and_hand();
        (HandRank::from(hrv), hand)
    }

    fn hand_rank_value(&self) -> HandRankValue {
        self.hand_rank_value_and_hand().0
    }

    /// Only differs from `hand_rank_value` for collections of more than five cards.
    fn hand_rank_value_and_hand(&self) -> (HandRankValue, Five);

    fn five_from_permutation(&self, permutation: [usize; 5]) -> Five;

    #[must_use]
    fn sort(&self) -> Self;

    fn sort_in_place(&mut self);
}

/// The kernel's minimal validity predicate. Revived from ckc-rs 0.1; pkcore's
/// `Pile::is_dealt` stays in pkcore for pkcore's own types.
pub trait HandValidator {
    fn are_unique(&self) -> bool;

    fn first(&self) -> Card;

    fn iter(&self) -> core::slice::Iter<'_, Card>;

    fn contains_blank(&self) -> bool {
        self.iter().any(|c| *c == Card::BLANK)
    }

    /// Corrupt = any value that is not a recognized `CardNumber`. Because
    /// `Card::BLANK` (0) is not a valid `CardNumber`, this subsumes blanks.
    fn is_corrupt(&self) -> bool {
        self.iter().any(|c| CardNumber::try_from(c.as_u32()).is_err())
    }

    fn is_valid(&self) -> bool {
        self.are_unique() && !self.is_corrupt()
    }
}

/// Parses a whitespace/`,`/`-` separated card index into exactly `N` distinct cards.
///
/// Written for `Five` in Task 8 and generalized here so `Five`, `Six` and `Seven` share
/// one parser instead of three transcriptions of it. pkcore routed all three through
/// `Cards::from_str` followed by `TryFrom<Cards>` (`five.rs:351`, `six.rs:177`,
/// `seven.rs:232`); `Cards` is a pkcore-only `IndexSet` type that does not follow the
/// kernel down. The observable behaviour is preserved — `,` and `-` are separators
/// (pkcore's `Terminal::index_cleaner`), every token must parse as a `Card` and a parse
/// failure is reported before any count error, blanks are dropped, duplicates collapse
/// keeping first-seen order, and exactly `N` distinct cards must remain — but nothing
/// is allocated.
///
/// The duplicate check only scans the first `N` distinct cards retained, so a repeat of
/// an `N+1`th-or-later distinct card is counted rather than collapsed. That cannot change
/// the returned `Result`: the count only ever increases, and any state past `N` resolves
/// to `CkcError::InvalidCardCount` regardless of the exact total.
///
/// # Errors
///
/// `CkcError::InvalidIndex` if a token is not a card, or if there are no tokens at all
/// (pkcore's `PKError::InvalidCardIndex`); `CkcError::Incomplete` for fewer than `N`
/// distinct cards (`PKError::NotEnoughCards`); `CkcError::InvalidCardCount` for more
/// than `N` (`PKError::TooManyCards`).
pub(crate) fn parse_hand<const N: usize>(s: &str) -> Result<[Card; N], CkcError> {
    let mut acc = [Card::BLANK; N];
    let mut len = 0usize;
    let mut tokens = 0usize;

    for token in s.split(|c: char| c.is_whitespace() || c == ',' || c == '-') {
        if token.is_empty() {
            continue;
        }
        tokens += 1;
        let card = Card::from_str(token)?;
        if card == Card::BLANK || acc[..len.min(N)].contains(&card) {
            continue;
        }
        if len < N {
            acc[len] = card;
        }
        len += 1;
    }

    if tokens == 0 {
        return Err(CkcError::InvalidIndex);
    }

    finish_hand(acc, len)
}

/// Collects an iterator of `Card` into exactly `N` distinct cards, reproducing the
/// filtering pkcore's `Cards` — an `IndexSet` — performed on the way in. Same rules as
/// [`parse_hand`] minus the tokenizing; see that function for the full contract.
///
/// # Errors
///
/// `CkcError::Incomplete` for fewer than `N` distinct non-blank cards;
/// `CkcError::InvalidCardCount` for more.
#[cfg(feature = "alloc")]
pub(crate) fn collect_hand<const N: usize, I: Iterator<Item = Card>>(cards: I) -> Result<[Card; N], CkcError> {
    let mut acc = [Card::BLANK; N];
    let mut len = 0usize;

    for card in cards {
        if card == Card::BLANK || acc[..len.min(N)].contains(&card) {
            continue;
        }
        if len < N {
            acc[len] = card;
        }
        len += 1;
    }

    finish_hand(acc, len)
}

/// The shared length verdict: `Incomplete` under, `InvalidCardCount` over.
fn finish_hand<const N: usize>(acc: [Card; N], len: usize) -> Result<[Card; N], CkcError> {
    match len.cmp(&N) {
        Ordering::Less => Err(CkcError::Incomplete),
        Ordering::Equal => Ok(acc),
        Ordering::Greater => Err(CkcError::InvalidCardCount),
    }
}
