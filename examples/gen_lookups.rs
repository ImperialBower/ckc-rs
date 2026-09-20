//! # Generating the `src/lookups` tables
//!
//! The four files in `src/lookups` (`flushes.rs`, `unique5.rs`, `products.rs`
//! and `values.rs`) arrived in this crate as pre-baked arrays of numbers,
//! inherited from [Vladislav Supalov's](https://github.com/vsupalov)
//! [pokereval-rs](https://github.com/vsupalov/pokereval-rs), which in turn came
//! from Cactus Kev's (aka [Kevin Suffecool](https://suffe.cool/))
//! [Poker Hand Evaluator](https://suffe.cool/poker/evaluator.html) in C.
//!
//! Nobody typed those numbers in by hand. They are *derived*, and this example
//! derives them again from scratch so you can see exactly where each one comes
//! from.
//!
//! Run it with:
//!
//! ```sh
//! cargo run --example gen_lookups
//! ```
//!
//! It rewrites the four files in place. The companion unit test in
//! `src/lookups/mod.rs` builds the same tables in memory and asserts they match
//! the committed ones, so the derivation below is checked on every `cargo test`.
//!
//! ## The one idea behind all four tables
//!
//! There are exactly **7462 distinct five-card poker hand strengths**. Number
//! them 1 (best) through 7462 (worst) and hand evaluation becomes a table
//! lookup instead of a pile of `if` statements.
//!
//! The 7462 are handed out by walking the hand categories from best to worst:
//!
//! | Rank range  | Count | Category        |
//! |-------------|-------|-----------------|
//! | 1..=10      | 10    | Straight flush  |
//! | 11..=166    | 156   | Four of a kind  |
//! | 167..=322   | 156   | Full house      |
//! | 323..=1599  | 1277  | Flush           |
//! | 1600..=1609 | 10    | Straight        |
//! | 1610..=2467 | 858   | Three of a kind |
//! | 2468..=3325 | 858   | Two pair        |
//! | 3326..=6185 | 2860  | One pair        |
//! | 6186..=7462 | 1277  | High card       |
//!
//! ## Why four tables and not one
//!
//! A hand is looked up two different ways, depending on whether its five ranks
//! are all different.
//!
//! **All five ranks different** (straight flush, flush, straight, high card).
//! Each card carries a rank bit, `1 << rank`, with deuce at bit 0 and ace at
//! bit 12. OR the five cards together and you get a 13-bit mask with exactly
//! five bits set. That mask is a perfect array index. The largest such mask is
//! `0b1_1111_0000_0000` = 7936, hence the array length of **7937**.
//!
//! * `FLUSHES[mask]`  - the hand when all five cards share a suit.
//! * `UNIQUE_5[mask]` - the hand when they do not.
//!
//! Two tables because the same five ranks are worth wildly different amounts
//! depending on the suits. Most slots are `0`: only 1287 of the 7937 indices
//! have exactly five bits set.
//!
//! A happy accident makes this work: for hands of five distinct ranks, the
//! numeric value of the mask sorts in the same order as poker strength. A-K-Q-J-9
//! is mask 7808, A-K-Q-J-8 is 7744, A-K-Q-10-8 is 7488. Bigger mask, better hand.
//! The wheel (A-5-4-3-2) would break that rule, but it is a straight, so it never
//! appears in the flush or high-card lists.
//!
//! **Some rank repeated** (quads, full house, trips, two pair, one pair). Now
//! the OR-mask is useless: it has fewer than five bits set and throws away the
//! multiplicity. So each rank is also given a prime number (deuce 2, trey 3, ...
//! ace 41) and the five primes are multiplied. By unique factorisation, that
//! single product identifies the multiset of ranks exactly.
//!
//! There are 4888 such hands. Their products are too sparse to index directly,
//! so they go in a **sorted** array and are found by binary search:
//!
//! * `PRODUCTS[i]` - the 4888 products, ascending.
//! * `VALUES[i]`   - the rank of the hand whose product is `PRODUCTS[i]`.
//!
//! Two parallel arrays, searched with `Five::find_in_products`.

use std::fmt::Write as _;
use std::path::Path;

/// Rank index 0 is the deuce, 12 is the ace.
const RANK_COUNT: usize = 13;

/// One prime per rank, smallest prime for the lowest rank. The mapping matches
/// `CardNumber::RANK_PRIME_FILTER` in `src/lib.rs`.
const PRIMES: [u32; RANK_COUNT] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

/// Used only for the comments this generator writes about itself.
const RANK_CHARS: [char; RANK_COUNT] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

/// Length of the two mask-indexed tables: the biggest five-bit mask is 7936.
const MASK_TABLE_LEN: usize = 7937;

/// Number of five-card hands that have at least one repeated rank.
const PRODUCT_TABLE_LEN: usize = 4888;

/// The four tables, exactly as they appear in `src/lookups`.
pub struct Lookups {
    pub flushes: Vec<u16>,
    pub unique5: Vec<u16>,
    pub products: Vec<u32>,
    pub values: Vec<u16>,
}

/// The ten straight rank-masks, strongest first.
///
/// Nine of them are a run of five adjacent bits, from ace-high down to six-high.
/// The tenth is the wheel, where the ace plays low: it is the ace bit welded onto
/// the bottom four bits, which is why it cannot be written as a shifted run.
fn straight_masks() -> Vec<u16> {
    let mut masks: Vec<u16> = (4..=12).rev().map(|high| 0b1_1111_u16 << (high - 4)).collect();
    masks.push((1 << 12) | 0b1111);
    masks
}

/// Every mask with exactly five bits set, strongest hand first.
///
/// Counting *down* from 7936 gives descending poker strength for free, per the
/// happy accident described in the module docs. There are 1287 of these
/// (13 choose 5); ten are straights, leaving 1277 flushes and 1277 high cards.
fn five_bit_masks_descending() -> Vec<u16> {
    (0b1_1111..=0b1_1111_0000_0000_u16)
        .rev()
        .filter(|m| m.count_ones() == 5)
        .collect()
}

/// Build all four tables from first principles.
///
/// The `rank` counter starts at 1 and only ever moves forward, so the category
/// order in the table below *is* the strength order. Read it top to bottom and
/// you are reading the rules of poker.
///
/// # Panics
///
/// If the nine categories do not add up to exactly 7462 hand ranks and 4888
/// products. That would mean the rules encoded below are wrong, so it is a
/// deliberate hard stop rather than a silently bad table.
// The length lint is waived on purpose: splitting the nine categories into nine
// functions would hide the single most important fact about them, which is that
// they run in this order, once, in one pass.
#[allow(clippy::too_many_lines)]
#[must_use]
pub fn build() -> Lookups {
    let mut flushes = vec![0_u16; MASK_TABLE_LEN];
    let mut unique5 = vec![0_u16; MASK_TABLE_LEN];
    // Collected as (product, rank) pairs, then sorted by product at the end.
    let mut by_product: Vec<(u32, u16)> = Vec::with_capacity(PRODUCT_TABLE_LEN);

    let mut rank: u16 = 1;
    let straights = straight_masks();
    let distinct = five_bit_masks_descending();

    // 1..=10 - Straight flushes. Same masks as straights, but into FLUSHES.
    for &mask in &straights {
        flushes[mask as usize] = rank;
        rank += 1;
    }

    // 11..=166 - Four of a kind. 13 quad ranks x 12 possible kickers.
    for quad in (0..RANK_COUNT).rev() {
        for kicker in (0..RANK_COUNT).rev() {
            if kicker == quad {
                continue;
            }
            by_product.push((PRIMES[quad].pow(4) * PRIMES[kicker], rank));
            rank += 1;
        }
    }

    // 167..=322 - Full house. 13 trip ranks x 12 pair ranks.
    for trips in (0..RANK_COUNT).rev() {
        for pair in (0..RANK_COUNT).rev() {
            if pair == trips {
                continue;
            }
            by_product.push((PRIMES[trips].pow(3) * PRIMES[pair].pow(2), rank));
            rank += 1;
        }
    }

    // 323..=1599 - Flushes. Every five-distinct-rank mask that is not a straight.
    for &mask in &distinct {
        if straights.contains(&mask) {
            continue;
        }
        flushes[mask as usize] = rank;
        rank += 1;
    }

    // 1600..=1609 - Straights. The same ten masks again, now unsuited.
    for &mask in &straights {
        unique5[mask as usize] = rank;
        rank += 1;
    }

    // 1610..=2467 - Three of a kind. 13 trip ranks x 66 kicker pairs.
    for trips in (0..RANK_COUNT).rev() {
        for high in (0..RANK_COUNT).rev() {
            if high == trips {
                continue;
            }
            for low in (0..high).rev() {
                if low == trips {
                    continue;
                }
                by_product.push((PRIMES[trips].pow(3) * PRIMES[high] * PRIMES[low], rank));
                rank += 1;
            }
        }
    }

    // 2468..=3325 - Two pair. 78 pair-pairs x 11 kickers.
    for high in (0..RANK_COUNT).rev() {
        for low in (0..high).rev() {
            for kicker in (0..RANK_COUNT).rev() {
                if kicker == high || kicker == low {
                    continue;
                }
                by_product.push((PRIMES[high].pow(2) * PRIMES[low].pow(2) * PRIMES[kicker], rank));
                rank += 1;
            }
        }
    }

    // 3326..=6185 - One pair. 13 pair ranks x 220 kicker triples.
    for pair in (0..RANK_COUNT).rev() {
        for first in (0..RANK_COUNT).rev() {
            if first == pair {
                continue;
            }
            for second in (0..first).rev() {
                if second == pair {
                    continue;
                }
                for third in (0..second).rev() {
                    if third == pair {
                        continue;
                    }
                    let product = PRIMES[pair].pow(2) * PRIMES[first] * PRIMES[second] * PRIMES[third];
                    by_product.push((product, rank));
                    rank += 1;
                }
            }
        }
    }

    // 6186..=7462 - High card. The flush masks one more time, now unsuited.
    for &mask in &distinct {
        if straights.contains(&mask) {
            continue;
        }
        unique5[mask as usize] = rank;
        rank += 1;
    }

    assert_eq!(rank, 7463, "expected to hand out exactly 7462 hand ranks");
    assert_eq!(by_product.len(), PRODUCT_TABLE_LEN);

    // The evaluator binary-searches PRODUCTS, so it has to be ascending.
    by_product.sort_unstable_by_key(|&(product, _)| product);

    let products = by_product.iter().map(|&(product, _)| product).collect();
    let values = by_product.iter().map(|&(_, value)| value).collect();

    Lookups {
        flushes,
        unique5,
        products,
        values,
    }
}

//region file writing

/// Render numbers the way rustfmt would: four-space indent, `, ` between items,
/// a trailing comma on every line, wrapped at `max_width` from `.rustfmt.toml`.
///
/// rustfmt keeps one column in hand, so its longest line here is 119, not 120.
/// Matching that exactly means `cargo fmt` leaves the generated files alone.
const MAX_WIDTH: usize = 119;

fn render_body<T: std::fmt::Display>(items: &[T]) -> String {
    let mut out = String::new();
    let mut line = String::from("    ");

    for item in items {
        let text = item.to_string();
        // `line` already holds a trailing space after the previous comma, so the
        // cost of adding this item is its own text plus the comma that follows.
        if line.len() > 4 && line.len() + text.len() + 1 > MAX_WIDTH {
            let _ = writeln!(out, "{}", line.trim_end());
            line = String::from("    ");
        }
        let _ = write!(line, "{text}, ");
    }

    if line.len() > 4 {
        let _ = writeln!(out, "{}", line.trim_end());
    }
    out
}

fn write_table<T: std::fmt::Display>(dir: &Path, file: &str, header: &str, decl: &str, items: &[T]) {
    let contents = format!("{header}{decl} = [\n{}];\n", render_body(items));
    let path = dir.join(file);
    std::fs::write(&path, contents).unwrap_or_else(|e| panic!("could not write {}: {e}", path.display()));
    println!("wrote {} ({} entries)", path.display(), items.len());
}

const FLUSHES_HEADER: &str = "\
/// this is a table lookup for all \"flush\" hands (e.g.  both
/// flushes and straight-flushes.  entries containing a zero
/// mean that combination is not possible with a five-card
/// flush hand.
";

const UNIQUE5_HEADER: &str = "\
/// this is a table lookup for all non-flush hands consisting
/// of five unique ranks (i.e.  either Straights or High Card
/// hands).  it's similar to the above \"flushes\" array.
";

fn main() {
    let lookups = build();

    // Defaults to overwriting the real tables. Pass a directory to write
    // somewhere harmless first and diff the result:
    //
    //   cargo run --example gen_lookups -- /tmp/lookups
    //
    // CARGO_MANIFEST_DIR keeps the default working no matter where it is run from.
    let dir = match std::env::args().nth(1) {
        Some(arg) => std::path::PathBuf::from(arg),
        None => Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("lookups"),
    };
    std::fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("could not create {}: {e}", dir.display()));

    write_table(
        &dir,
        "flushes.rs",
        FLUSHES_HEADER,
        "pub const FLUSHES: [u16; 7937]",
        &lookups.flushes,
    );
    write_table(
        &dir,
        "unique5.rs",
        UNIQUE5_HEADER,
        "pub const UNIQUE_5: [u16; 7937]",
        &lookups.unique5,
    );
    write_table(
        &dir,
        "products.rs",
        "",
        "pub static PRODUCTS: [u32; 4888]",
        &lookups.products,
    );
    write_table(&dir, "values.rs", "", "pub const VALUES: [u16; 4888]", &lookups.values);

    // A small readout so the run is not silent about what it just derived.
    let best_flush = lookups.flushes.iter().position(|&v| v == 1).unwrap();
    println!(
        "\nsanity: mask {best_flush:#015b} is the royal flush, and it is hand rank {}",
        lookups.flushes[best_flush]
    );
    let worst = lookups.unique5.iter().position(|&v| v == 7462).unwrap();
    let ranks: String = (0..RANK_COUNT)
        .rev()
        .filter(|r| worst & (1 << r) != 0)
        .map(|r| RANK_CHARS[r])
        .collect();
    println!("sanity: the worst hand in poker is {ranks} offsuit, hand rank 7462");
}

//endregion
