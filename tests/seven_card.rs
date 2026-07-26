//! Seven-card evaluation. The exhaustive sweep is C(52,7) = 133,784,560 hands
//! × 21 permutations ≈ 2.8 billion evaluations — far too slow for CI, so the
//! per-commit test is a deterministic sample and the full run is #[ignore]d.
//!
//! **The reference computation deliberately does not read
//! `Seven::FIVE_CARD_PERMUTATIONS`.** It enumerates the C(7,5) index subsets from
//! scratch. The plan's draft of this file used the constant on both sides of the
//! assertion, which made the 21 hand-written 5-tuples — the single most
//! transcription-prone artifact in `seven.rs` — compare against themselves; a
//! corrupted entry passed. Verified by mutation: changing the last row to
//! `[2, 3, 4, 5, 5]` left the self-referential version green.

use ckc_rs::standard52::{Card, CardNumber, HandRanker, Seven, Six, evaluate};

fn deck() -> [Card; 52] {
    let mut deck = [Card::BLANK; 52];
    for (i, cn) in CardNumber::ALL.iter().enumerate() {
        deck[i] = Card::from(*cn as u32);
    }
    deck
}

/// The best five-card rank over **every** five-element subset of `cards`, enumerated
/// independently of the type's permutation table and evaluated with
/// `evaluate::five_cards` — the function the golden oracle already pins exhaustively.
fn best_of_all_subsets<const N: usize>(cards: [Card; N]) -> u16 {
    let mut best = 0u16;
    for i in 0..N {
        for j in (i + 1)..N {
            for k in (j + 1)..N {
                for l in (k + 1)..N {
                    for m in (l + 1)..N {
                        let hrv = evaluate::five_cards([cards[i], cards[j], cards[k], cards[l], cards[m]]);
                        if hrv != 0 && (best == 0 || hrv < best) {
                            best = hrv;
                        }
                    }
                }
            }
        }
    }
    best
}

/// Collects the C(N,5) index subsets in lexicographic order, independently of the
/// constant under test.
fn subsets<const N: usize>() -> Vec<[usize; 5]> {
    let mut out = Vec::new();
    for i in 0..N {
        for j in (i + 1)..N {
            for k in (j + 1)..N {
                for l in (k + 1)..N {
                    for m in (l + 1)..N {
                        out.push([i, j, k, l, m]);
                    }
                }
            }
        }
    }
    out
}

/// The permutation tables must be exactly the complete set of five-card subsets — every
/// one present, none repeated, none out of range. Nothing else in the suite pins them:
/// `hand_rank_value` takes the *best* result, so a duplicated row is invisible and a
/// missing row only shows up on the hands whose winner it held.
#[test]
fn permutation_tables_are_the_complete_subset_enumeration() {
    assert_eq!(subsets::<6>(), Six::FIVE_CARD_PERMUTATIONS.to_vec());
    assert_eq!(subsets::<7>(), Seven::FIVE_CARD_PERMUTATIONS.to_vec());
}

/// A seven-card hand must rank exactly as well as the best of its 21 five-card
/// subsets — checked against `evaluate::five_cards`, which the golden oracle
/// already pins exhaustively.
#[test]
fn seven_matches_best_of_twenty_one_subsets() {
    let deck = deck();
    // Deterministic stride sample: every 9973rd (prime) 7-card index.
    let mut n = 0u64;
    let mut checked = 0u32;

    for a in 0..52 {
        for b in (a + 1)..52 {
            for c in (b + 1)..52 {
                for d in (c + 1)..52 {
                    for e in (d + 1)..52 {
                        for f in (e + 1)..52 {
                            for g in (f + 1)..52 {
                                n += 1;
                                if n % 9973 != 0 {
                                    continue;
                                }
                                let cards = [deck[a], deck[b], deck[c], deck[d], deck[e], deck[f], deck[g]];
                                let seven = Seven::from(cards);
                                assert_eq!(seven.hand_rank_value(), best_of_all_subsets(cards));
                                checked += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(checked > 13_000, "sample too small: {checked}");
}

/// The same property for `Six` over its six subsets, on a denser stride sample —
/// `Six` is otherwise covered only by the two unit tests in `six.rs`.
#[test]
fn six_matches_best_of_six_subsets() {
    let deck = deck();
    let mut n = 0u64;
    let mut checked = 0u32;

    for a in 0..52 {
        for b in (a + 1)..52 {
            for c in (b + 1)..52 {
                for d in (c + 1)..52 {
                    for e in (d + 1)..52 {
                        for f in (e + 1)..52 {
                            n += 1;
                            if n % 997 != 0 {
                                continue;
                            }
                            let cards = [deck[a], deck[b], deck[c], deck[d], deck[e], deck[f]];
                            let six = Six::from(cards);
                            assert_eq!(six.hand_rank_value(), best_of_all_subsets(cards));
                            checked += 1;
                        }
                    }
                }
            }
        }
    }
    assert!(checked > 20_000, "sample too small: {checked}");
}

/// The full C(52,7) sweep. Run explicitly:
///
/// ```text
/// cargo test --release --test seven_card -- --ignored seven_exhaustive
/// ```
#[test]
#[ignore]
fn seven_exhaustive() {
    let deck = deck();
    let mut count = 0u64;
    for a in 0..52 {
        for b in (a + 1)..52 {
            for c in (b + 1)..52 {
                for d in (c + 1)..52 {
                    for e in (d + 1)..52 {
                        for f in (e + 1)..52 {
                            for g in (f + 1)..52 {
                                let seven =
                                    Seven::from([deck[a], deck[b], deck[c], deck[d], deck[e], deck[f], deck[g]]);
                                assert_ne!(seven.hand_rank_value(), 0);
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(count, 133_784_560);
}
