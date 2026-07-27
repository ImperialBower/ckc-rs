//! EPIC-80's invalid-hand semantics: `hand_rank_value` guards with `is_valid()`
//! rather than evaluating whatever it is handed.
//!
//! ckc-rs 0.1 returned a garbage-but-plausible rank for all of these.
//!
//! The golden oracle covers only valid hands and cannot see any of it.
//!
//! Note on the corrupt-`u32` case: every *constructor* sanitizes, so
//! `Card::from(23)` yields `BLANK` and the direct route needs the `pub(crate)`
//! field (see `five.rs`'s `is_corrupt_rejects_a_non_cardnumber_hand`). But the
//! public `frequency_*` *transformations* reach a corrupt card without any
//! crate-internal access — pinned below by
//! `frequency_flagged_cards_are_corrupt_through_public_api`.

use ckc_rs::standard52::{Card, Five, HandRanker, HandValidator, NO_HAND_RANK_VALUE, Six, evaluate};

#[test]
fn duplicate_cards_yield_no_rank() {
    let hand = [
        Card::JACK_CLUBS,
        Card::DEUCE_CLUBS,
        Card::TREY_CLUBS,
        Card::KING_SPADES,
        Card::JACK_CLUBS, // dupe of the first
    ];
    assert_eq!(evaluate::five_cards(hand), NO_HAND_RANK_VALUE);
    assert_eq!(Five::from(hand).hand_rank_value(), NO_HAND_RANK_VALUE);
}

#[test]
fn blank_card_yields_no_rank() {
    let hand = [
        Card::JACK_CLUBS,
        Card::QUEEN_DIAMONDS,
        Card::TREY_CLUBS,
        Card::KING_SPADES,
        Card::BLANK,
    ];
    assert_eq!(evaluate::five_cards(hand), NO_HAND_RANK_VALUE);
    assert_eq!(Five::from(hand).hand_rank_value(), NO_HAND_RANK_VALUE);
}

/// `Card::from` sanitizes, so this is a *blank* hand, not a corrupt one — which is
/// exactly the point. Pins that the public API cannot smuggle a non-`CardNumber`
/// value past the guard; the genuinely-corrupt case is in `five.rs`.
///
/// 23 is `0b10111`. Every `CardNumber` carries a suit bit in the 12..=15 nibble and a
/// rank bit in 16..=28; 23 has neither, so it is not a card under any reading — it is
/// a bare rank-prime field with nothing else set.
#[test]
fn from_u32_sanitizes_rather_than_producing_a_corrupt_card() {
    assert_eq!(Card::BLANK, Card::from(23));

    let hand = [
        Card::JACK_CLUBS,
        Card::DEUCE_CLUBS,
        Card::from(23),
        Card::KING_SPADES,
        Card::TEN_SPADES,
    ];
    assert_eq!(evaluate::five_cards(hand), NO_HAND_RANK_VALUE);
    assert_eq!(Five::from(hand).hand_rank_value(), NO_HAND_RANK_VALUE);
}

#[test]
fn all_blanks_yield_no_rank() {
    let hand = [Card::BLANK; 5];
    assert_eq!(evaluate::five_cards(hand), NO_HAND_RANK_VALUE);
}

/// Regression pin: a valid hand must still evaluate, so the guard cannot be
/// "fixed" by making everything invalid.
///
/// 1 is the top of Cactus Kev's 1..=7462 ladder — a royal flush — and the frozen
/// golden file independently contains exactly four hands at that rank.
#[test]
fn valid_hands_still_evaluate() {
    let royal = [
        Card::ACE_SPADES,
        Card::KING_SPADES,
        Card::QUEEN_SPADES,
        Card::JACK_SPADES,
        Card::TEN_SPADES,
    ];
    assert_eq!(evaluate::five_cards(royal), 1);
}

/// Inherited off-by-one: `POSSIBLE_COMBINATIONS` is a count, not a max index, so
/// `UNIQUE_5[7937]` was indexed one past the end of a `[u16; 7937]`.
///
/// The expected values are anchored *outside* the fix. 7938 was already out of range
/// before the change and already returned the blank sentinel, so the fix is exactly
/// the claim "7937 behaves like 7938". 1600 is the Cactus Kev rank of an ace-high
/// straight, whose `or_rank_bits()` is `0b1111100000000 == 7936` — the last genuinely
/// valid index — and the golden oracle pins that rank for every A-K-Q-J-T rainbow hand.
#[test]
fn unique_rank_rejects_out_of_range_index_without_panicking() {
    // Behaviour that predates the fix, used here as the reference.
    assert_eq!(0, Five::unique_rank(7938));

    assert_eq!(Five::unique_rank(7938), Five::unique_rank(7937));
    assert_eq!(0, Five::unique_rank(7937));
    assert_eq!(0, Five::unique_rank(usize::MAX));

    // The last genuinely valid index must still resolve, and to the same rank an
    // ace-high straight gets through the evaluator.
    assert_eq!(1600, Five::unique_rank(7936));
}

/// Inherited `usize` underflow: a key below every product walked `high` to -1.
///
/// An all-blank hand multiplies out to a key of 0 and the smallest entry in
/// `PRODUCTS` is 48, so 0 is below the whole table — the exact shape that used to
/// panic. 0 is the not-found return, which the original code also used and which the
/// rewrite leaves unchanged.
#[test]
fn not_unique_on_a_blank_hand_does_not_panic() {
    let blanks = Five::from([Card::BLANK; 5]);
    assert_eq!(blanks.find_in_products(), 0);
    let _ = blanks.not_unique();
}

/// **Documents an asymmetry rather than asserting an ideal.** `Five` rejects an
/// invalid hand outright, but `Six`/`Seven` guard per-permutation, so a duplicate
/// card does NOT produce `NO_HAND_RANK_VALUE` — the permutations omitting one copy
/// are five distinct cards and evaluate normally, and the best of those is returned.
///
/// This is inherited pkcore behavior, deliberately left intact by EPIC-80. The test
/// exists so the asymmetry is a recorded decision rather than an accident, and so
/// that anyone who later "fixes" it has to do so knowingly.
#[test]
fn six_and_seven_do_not_reject_duplicates_the_way_five_does() {
    // Mixed suits, deliberately. An all-spades version of this hand is a *flush*, so
    // `or_rank_bits` is 0b1111000000000 and the flush table happens to hold 0 at that
    // index — meaning it returns 0 whether or not the guard exists, and documents
    // nothing. With mixed suits the guard is the only thing that can reject it.
    let five = Five::from([
        Card::ACE_SPADES,
        Card::KING_HEARTS,
        Card::QUEEN_DIAMONDS,
        Card::JACK_CLUBS,
        Card::ACE_SPADES, // dupe
    ]);
    assert!(!five.is_flush(), "must not take the flush path");
    assert_eq!(NO_HAND_RANK_VALUE, five.hand_rank_value());

    // The SAME six cards. Dropping one duplicate ace leaves a broadway straight, so
    // `Six` returns a real rank where `Five` refused. 1600 is the broadway straight,
    // read from the frozen ckc-rs 0.1.18 golden file — not from this crate's tables.
    let six = Six::from([
        Card::ACE_SPADES,
        Card::KING_HEARTS,
        Card::QUEEN_DIAMONDS,
        Card::JACK_CLUBS,
        Card::TEN_SPADES,
        Card::ACE_SPADES, // dupe, but a valid five-card hand hides inside
    ]);
    assert_eq!(1600, six.hand_rank_value(), "Six evaluates the best valid subset");
}

/// EPIC-80's **one externally-visible behavior change**, pinned.
///
/// The EPIC twice claimed there was none, on the grounds that every public path to a
/// `Card` sanitizes. That enumerated *constructors* and missed *transformations*:
/// `Card::frequency_paired`/`frequency_tripped`/`frequency_quaded` are public, set bits
/// 29..=31 that no `CardNumber` sets, and leave the card non-`BLANK`. `Five::from` does
/// not validate. So a corrupt hand is reachable with no `pub(crate)` access at all.
///
/// The divergence is real and favours the kernel: pkcore's `is_dealt` (`are_unique() &&
/// !contains_blank()`) accepts this hand, and for a *flush* it then indexes `FLUSHES`
/// with `or_rank_bits() == 16128` against a 7937-entry table — out of bounds. `is_valid`
/// rejects it first and returns `NO_HAND_RANK_VALUE`.
#[test]
fn frequency_flagged_cards_are_corrupt_through_public_api() {
    let flagged = Card::ACE_SPADES.frequency_paired();

    // Reachable, non-blank, and not a real card.
    assert_ne!(Card::BLANK, flagged);
    assert_ne!(Card::ACE_SPADES, flagged);

    let hand = Five::from([
        flagged,
        Card::KING_HEARTS,
        Card::QUEEN_DIAMONDS,
        Card::JACK_CLUBS,
        Card::TEN_SPADES,
    ]);

    // The distinguishing property: pkcore's weaker check would have accepted this.
    assert!(hand.are_unique() && !hand.contains_blank(), "is_dealt-style: accepts");
    assert!(hand.is_corrupt() && !hand.is_valid(), "is_valid: rejects");
    assert_eq!(NO_HAND_RANK_VALUE, hand.hand_rank_value());

    // Unflagged, the same five cards are a broadway straight.
    let clean = Five::from([
        Card::ACE_SPADES,
        Card::KING_HEARTS,
        Card::QUEEN_DIAMONDS,
        Card::JACK_CLUBS,
        Card::TEN_SPADES,
    ]);
    assert_eq!(1600, clean.hand_rank_value());
}

/// The flush variant, which is where the old behavior was not merely different but a
/// panic: `or_rank_bits()` lands far outside `FLUSHES`, and pkcore reads that table with
/// no bounds guard.
#[test]
fn a_flagged_flush_would_have_indexed_out_of_bounds() {
    let hand = Five::from([
        Card::ACE_SPADES.frequency_paired(),
        Card::KING_SPADES,
        Card::QUEEN_SPADES,
        Card::JACK_SPADES,
        Card::TEN_SPADES,
    ]);

    assert!(hand.is_flush(), "still reads as a flush");
    assert!(
        hand.or_rank_bits() as usize >= Five::POSSIBLE_COMBINATIONS,
        "or_rank_bits {} must be outside the {}-entry table",
        hand.or_rank_bits(),
        Five::POSSIBLE_COMBINATIONS
    );
    assert_eq!(NO_HAND_RANK_VALUE, hand.hand_rank_value(), "guarded, not evaluated");
}
