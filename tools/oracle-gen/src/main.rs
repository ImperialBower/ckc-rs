//! Generates the C(52,5) golden oracle from the PUBLISHED ckc-rs 0.1.18.
//! Run once, from the ckc-rs repo root:
//!   cargo run --release --manifest-path tools/oracle-gen/Cargo.toml

use ckc_rs::deck::POKER_DECK;
use ckc_rs::evaluate;
use std::fs;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let deck = POKER_DECK.arr();
    fs::create_dir_all("tests/golden")?;
    let file = fs::File::create("tests/golden/five_card_ranks.bin")?;
    let mut out = BufWriter::new(file);

    let mut count: u64 = 0;
    for a in 0..52 {
        for b in (a + 1)..52 {
            for c in (b + 1)..52 {
                for d in (c + 1)..52 {
                    for e in (d + 1)..52 {
                        let hand = [deck[a], deck[b], deck[c], deck[d], deck[e]];
                        let hrv = evaluate::five_cards(hand);
                        out.write_all(&hrv.to_le_bytes())?;
                        count += 1;
                    }
                }
            }
        }
    }
    out.flush()?;
    assert_eq!(count, 2_598_960, "C(52,5) must be 2598960, got {count}");
    println!("wrote {count} hand rank values");
    Ok(())
}
