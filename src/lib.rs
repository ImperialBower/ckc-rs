#![no_std]

extern crate alloc;

use strum::EnumIter;

pub mod deck;
mod lookups;

/// A `PokerCard` is a u32 representation of a variant of Cactus Kev's binary
/// representation of a poker card as designed for rapid hand evaluation as
/// documented [here](https://suffe.cool/poker/evaluator.html).
///
/// The variation being that the `Suit` bits order is inverted for easier sorting.
/// ```txt
/// +--------+--------+--------+--------+
/// |xxxbbbbb|bbbbbbbb|SHDCrrrr|xxpppppp|
/// +--------+--------+--------+--------+
///
/// p = prime number of rank (deuce=2,trey=3,four=5,...,ace=41)
/// r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
/// SHDC = suit of card (bit turned on based on suit of card)
/// b = bit turned on depending on rank of card
/// ```
pub type CKCNumber = u32;
pub type HandRankValue = u16;

/// u32 constants for all 52 cards in a standard poker deck.
pub struct CardNumber;

impl CardNumber {
    //region constants
    pub const ACE_SPADES: u32 = 268_471_337;
    pub const KING_SPADES: u32 = 134_253_349;
    pub const QUEEN_SPADES: u32 = 67_144_223;
    pub const JACK_SPADES: u32 = 33_589_533;
    pub const TEN_SPADES: u32 = 16_812_055;
    pub const NINE_SPADES: u32 = 8_423_187;
    pub const EIGHT_SPADES: u32 = 4_228_625;
    pub const SEVEN_SPADES: u32 = 2_131_213;
    pub const SIX_SPADES: u32 = 1_082_379;
    pub const FIVE_SPADES: u32 = 557_831;
    pub const FOUR_SPADES: u32 = 295_429;
    pub const TREY_SPADES: u32 = 164_099;
    pub const DEUCE_SPADES: u32 = 98_306;
    pub const ACE_HEARTS: u32 = 268_454_953;
    pub const KING_HEARTS: u32 = 134_236_965;
    pub const QUEEN_HEARTS: u32 = 67_127_839;
    pub const JACK_HEARTS: u32 = 33_573_149;
    pub const TEN_HEARTS: u32 = 16_795_671;
    pub const NINE_HEARTS: u32 = 8_406_803;
    pub const EIGHT_HEARTS: u32 = 4_212_241;
    pub const SEVEN_HEARTS: u32 = 2_114_829;
    pub const SIX_HEARTS: u32 = 1_065_995;
    pub const FIVE_HEARTS: u32 = 541_447;
    pub const FOUR_HEARTS: u32 = 279_045;
    pub const TREY_HEARTS: u32 = 147_715;
    pub const DEUCE_HEARTS: u32 = 81_922;
    pub const ACE_DIAMONDS: u32 = 268_446_761;
    pub const KING_DIAMONDS: u32 = 134_228_773;
    pub const QUEEN_DIAMONDS: u32 = 67_119_647;
    pub const JACK_DIAMONDS: u32 = 33_564_957;
    pub const TEN_DIAMONDS: u32 = 16_787_479;
    pub const NINE_DIAMONDS: u32 = 8_398_611;
    pub const EIGHT_DIAMONDS: u32 = 4_204_049;
    pub const SEVEN_DIAMONDS: u32 = 2_106_637;
    pub const SIX_DIAMONDS: u32 = 1_057_803;
    pub const FIVE_DIAMONDS: u32 = 533_255;
    pub const FOUR_DIAMONDS: u32 = 270_853;
    pub const TREY_DIAMONDS: u32 = 139_523;
    pub const DEUCE_DIAMONDS: u32 = 73_730;
    pub const ACE_CLUBS: u32 = 268_442_665;
    pub const KING_CLUBS: u32 = 134_224_677;
    pub const QUEEN_CLUBS: u32 = 67_115_551;
    pub const JACK_CLUBS: u32 = 33_560_861;
    pub const TEN_CLUBS: u32 = 16_783_383;
    pub const NINE_CLUBS: u32 = 8_394_515;
    pub const EIGHT_CLUBS: u32 = 4_199_953;
    pub const SEVEN_CLUBS: u32 = 2_102_541;
    pub const SIX_CLUBS: u32 = 1_053_707;
    pub const FIVE_CLUBS: u32 = 529_159;
    pub const FOUR_CLUBS: u32 = 266_757;
    pub const TREY_CLUBS: u32 = 135_427;
    pub const DEUCE_CLUBS: u32 = 69_634;
    pub const BLANK: u32 = 0;
    //endregion

    pub fn filter(number: u32) -> CKCNumber {
        <CKCNumber as PokerCard>::filter(number)
    }
}

#[cfg(test)]
mod card_number_tests {
    use super::*;

    #[test]
    fn filter() {
        assert_eq!(CardNumber::filter(2), CardNumber::BLANK);
        assert_eq!(
            CardNumber::filter(CardNumber::NINE_CLUBS),
            CardNumber::NINE_CLUBS
        );
    }
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq)]
pub enum CardRank {
    ACE,
    KING,
    QUEEN,
    JACK,
    TEN,
    NINE,
    EIGHT,
    SEVEN,
    SIX,
    FIVE,
    FOUR,
    THREE,
    TWO,
    Blank,
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq)]
pub enum CardSuit {
    SPADES,
    HEARTS,
    DIAMONDS,
    CLUBS,
    Blank,
}

pub mod evaluate {
    use crate::{CKCNumber, CardNumber, HandRankValue};

    pub const POSSIBLE_COMBINATIONS: usize = 7937;
    /// Binary filter for `CardNumber` `Suit` flags.
    /// 00000000 00000000 11110000 00000000
    pub const SUITS_FILTER: u32 = 0xf000;

    #[must_use]
    pub fn five_cards(five_cards: [CKCNumber; 5]) -> HandRankValue {
        if is_corrupt(five_cards) || has_dupes(five_cards) {
            return CardNumber::BLANK as HandRankValue;
        }
        let i = or_rank_bits(five_cards);

        if is_flush(five_cards) {
            return crate::lookups::FLUSHES[i];
        }

        // Continue to evaluate if it's not a flush and the cards aren't
        // unique (straight or high card).
        let unique = unique(i);
        match unique {
            0 => not_unique(five_cards),
            _ => unique,
        }
    }

    #[must_use]
    pub fn is_flush(five_cards: [CKCNumber; 5]) -> bool {
        (five_cards[0]
            & five_cards[1]
            & five_cards[2]
            & five_cards[3]
            & five_cards[4]
            & SUITS_FILTER)
            != 0
    }

    /// Returns a value that is made up of performing an or operation on all of the
    /// rank bit flags of the `PokerCard`.
    #[must_use]
    pub fn or_rank_bits(five_cards: [CKCNumber; 5]) -> usize {
        ((five_cards[0] | five_cards[1] | five_cards[2] | five_cards[3] | five_cards[4]) as usize)
            >> 16
    }

    #[allow(clippy::comparison_chain)]
    fn find_in_products(key: usize) -> usize {
        let mut low = 0;
        let mut high = 4887;
        let mut mid;

        while low <= high {
            mid = (high + low) >> 1; // divide by two

            let product = crate::lookups::PRODUCTS[mid] as usize;
            if key < product {
                high = mid - 1;
            } else if key > product {
                low = mid + 1;
            } else {
                return mid;
            }
        }
        0
    }

    fn has_dupes(c: [CKCNumber; 5]) -> bool {
        (1..5).any(|i| c[i..].contains(&c[i - 1]))
    }

    fn is_corrupt(c: [CKCNumber; 5]) -> bool {
        for x in &c {
            if CardNumber::filter(*x) == CardNumber::BLANK {
                return true;
            }
        }
        false
    }

    fn multiply_primes(five_cards: [CKCNumber; 5]) -> usize {
        ((five_cards[0] & 0xff)
            * (five_cards[1] & 0xff)
            * (five_cards[2] & 0xff)
            * (five_cards[3] & 0xff)
            * (five_cards[4] & 0xff)) as usize
    }

    fn not_unique(five_cards: [CKCNumber; 5]) -> HandRankValue {
        crate::lookups::VALUES[find_in_products(multiply_primes(five_cards))]
    }

    fn unique(index: usize) -> HandRankValue {
        if index > POSSIBLE_COMBINATIONS {
            return CardNumber::BLANK as HandRankValue;
        }
        crate::lookups::UNIQUE_5[index]
    }
}

#[cfg(test)]
mod evaluate_tests {
    use super::*;
    use alloc::format;

    #[test]
    fn five_cards_royal_flush() {
        let cards = [
            CardNumber::ACE_SPADES,
            CardNumber::KING_SPADES,
            CardNumber::QUEEN_SPADES,
            CardNumber::JACK_SPADES,
            CardNumber::TEN_SPADES,
        ];
        assert_eq!(evaluate::five_cards(cards), 1);
    }

    #[test]
    fn five_cards_straight() {
        let first = [
            CardNumber::NINE_CLUBS,
            CardNumber::KING_SPADES,
            CardNumber::QUEEN_SPADES,
            CardNumber::JACK_SPADES,
            CardNumber::TEN_SPADES,
        ];
        let second = [
            CardNumber::NINE_CLUBS,
            CardNumber::QUEEN_SPADES,
            CardNumber::JACK_SPADES,
            CardNumber::TEN_SPADES,
            CardNumber::EIGHT_CLUBS,
        ];
        assert_eq!(evaluate::five_cards(first), 1601);
        assert_eq!(evaluate::five_cards(second), 1602);
    }

    #[test]
    fn five_cards_two_pair() {
        let cards = [
            CardNumber::JACK_CLUBS,
            CardNumber::DEUCE_CLUBS,
            CardNumber::DEUCE_DIAMONDS,
            CardNumber::JACK_SPADES,
            CardNumber::TEN_SPADES,
        ];
        assert_eq!(evaluate::five_cards(cards), 2922);
    }

    #[test]
    fn five_cards_king_high() {
        let first = [
            CardNumber::JACK_CLUBS,
            CardNumber::DEUCE_CLUBS,
            CardNumber::TREY_CLUBS,
            CardNumber::KING_SPADES,
            CardNumber::TEN_SPADES,
        ];
        let second = [
            CardNumber::JACK_CLUBS,
            CardNumber::QUEEN_DIAMONDS,
            CardNumber::TREY_CLUBS,
            CardNumber::KING_SPADES,
            CardNumber::TEN_SPADES,
        ];
        assert_eq!(evaluate::five_cards(first), 6825);
        assert_eq!(evaluate::five_cards(second), 6684);
    }

    #[test]
    fn or_rank_bits() {
        let cards = [
            CardNumber::ACE_SPADES,
            CardNumber::KING_SPADES,
            CardNumber::QUEEN_SPADES,
            CardNumber::JACK_SPADES,
            CardNumber::TEN_SPADES,
        ];

        let or = evaluate::or_rank_bits(cards);
        assert_eq!("0001111100000000", format!("{:016b}", or));
        assert_eq!(or, 7936);
    }

    #[test]
    fn check_dupes() {
        let hand = [
            CardNumber::JACK_CLUBS,
            CardNumber::DEUCE_CLUBS,
            CardNumber::TREY_CLUBS,
            CardNumber::KING_SPADES,
            CardNumber::JACK_CLUBS,
        ];
        assert_eq!(evaluate::five_cards(hand), 0);
    }

    #[test]
    fn check_corrupt() {
        let first = [
            CardNumber::JACK_CLUBS,
            CardNumber::DEUCE_CLUBS,
            23,
            CardNumber::KING_SPADES,
            CardNumber::TEN_SPADES,
        ];
        let second = [
            CardNumber::JACK_CLUBS,
            CardNumber::QUEEN_DIAMONDS,
            CardNumber::TREY_CLUBS,
            CardNumber::KING_SPADES,
            CardNumber::BLANK,
        ];
        assert_eq!(evaluate::five_cards(first), 0);
        assert_eq!(evaluate::five_cards(second), 0);
    }
}

pub trait PokerCard {
    /// Only allows you to create a `CKCNumber` that is valid.
    fn filter(number: CKCNumber) -> CKCNumber {
        match number {
            CardNumber::ACE_SPADES
            | CardNumber::KING_SPADES
            | CardNumber::QUEEN_SPADES
            | CardNumber::JACK_SPADES
            | CardNumber::TEN_SPADES
            | CardNumber::NINE_SPADES
            | CardNumber::EIGHT_SPADES
            | CardNumber::SEVEN_SPADES
            | CardNumber::SIX_SPADES
            | CardNumber::FIVE_SPADES
            | CardNumber::FOUR_SPADES
            | CardNumber::TREY_SPADES
            | CardNumber::DEUCE_SPADES
            | CardNumber::ACE_HEARTS
            | CardNumber::KING_HEARTS
            | CardNumber::QUEEN_HEARTS
            | CardNumber::JACK_HEARTS
            | CardNumber::TEN_HEARTS
            | CardNumber::NINE_HEARTS
            | CardNumber::EIGHT_HEARTS
            | CardNumber::SEVEN_HEARTS
            | CardNumber::SIX_HEARTS
            | CardNumber::FIVE_HEARTS
            | CardNumber::FOUR_HEARTS
            | CardNumber::TREY_HEARTS
            | CardNumber::DEUCE_HEARTS
            | CardNumber::ACE_DIAMONDS
            | CardNumber::KING_DIAMONDS
            | CardNumber::QUEEN_DIAMONDS
            | CardNumber::JACK_DIAMONDS
            | CardNumber::TEN_DIAMONDS
            | CardNumber::NINE_DIAMONDS
            | CardNumber::EIGHT_DIAMONDS
            | CardNumber::SEVEN_DIAMONDS
            | CardNumber::SIX_DIAMONDS
            | CardNumber::FIVE_DIAMONDS
            | CardNumber::FOUR_DIAMONDS
            | CardNumber::TREY_DIAMONDS
            | CardNumber::DEUCE_DIAMONDS
            | CardNumber::ACE_CLUBS
            | CardNumber::KING_CLUBS
            | CardNumber::QUEEN_CLUBS
            | CardNumber::JACK_CLUBS
            | CardNumber::TEN_CLUBS
            | CardNumber::NINE_CLUBS
            | CardNumber::EIGHT_CLUBS
            | CardNumber::SEVEN_CLUBS
            | CardNumber::SIX_CLUBS
            | CardNumber::FIVE_CLUBS
            | CardNumber::FOUR_CLUBS
            | CardNumber::TREY_CLUBS
            | CardNumber::DEUCE_CLUBS => number,
            _ => CardNumber::BLANK,
        }
    }

    fn as_u32(&self) -> u32;

    fn is_blank(&self) -> bool;
}

impl PokerCard for CKCNumber {
    fn as_u32(&self) -> u32 {
        *self
    }

    fn is_blank(&self) -> bool {
        *self == CardNumber::BLANK
    }
}

#[cfg(test)]
mod poker_card_tests {
    use super::*;

    #[test]
    fn filter() {
        assert_eq!(
            <CKCNumber as PokerCard>::filter(CardNumber::ACE_SPADES),
            CardNumber::ACE_SPADES
        );
        assert_eq!(
            <CKCNumber as PokerCard>::filter(CardNumber::KING_CLUBS),
            CardNumber::filter(CardNumber::KING_CLUBS)
        );
        assert_eq!(<CKCNumber as PokerCard>::filter(2), CardNumber::BLANK);
    }

    #[test]
    fn is_blank() {
        let card = CardNumber::BLANK;

        assert!(card.is_blank());
        assert!(0_u32.is_blank());
        assert!(0.is_blank());
    }
}
