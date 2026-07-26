//! The headline convenience entry point into the evaluator.

use crate::standard52::arrays::HandRanker;
use crate::standard52::card::Card;
use crate::standard52::five::Five;
use crate::standard52::hand_rank::HandRankValue;

/// The headline entry point. Returns `NO_HAND_RANK_VALUE` (0) for any hand that
/// is not five distinct, well-formed cards.
///
/// ```
/// use ckc_rs::prelude::*;
///
/// let royal_flush = [
///     Card::ACE_SPADES,
///     Card::KING_SPADES,
///     Card::QUEEN_SPADES,
///     Card::JACK_SPADES,
///     Card::TEN_SPADES,
/// ];
///
/// assert_eq!(1, evaluate::five_cards(royal_flush));
/// ```
#[must_use]
pub fn five_cards(cards: [Card; 5]) -> HandRankValue {
    Five::from(cards).hand_rank_value()
}
