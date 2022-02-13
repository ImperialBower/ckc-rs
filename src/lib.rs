#![cfg_attr(not(test), no_std)]
#![warn(clippy::pedantic)]
#![allow(clippy::unreadable_literal)]

extern crate alloc;

use crate::parse::get_rank_and_suit;
use strum::EnumIter;

pub mod cards;
pub mod deck;
pub mod hand_rank;
mod lookups;
pub mod parse;

/// A `PokerCard` is a u32 representation of a variant of Cactus Kev's binary
/// representation of a poker card as designed for rapid hand evaluation as
/// documented [here](https://suffe.cool/poker/evaluator.html).
///
/// The variation being that the `Suit` bits order is inverted for easier sorting.
/// ```txt
/// +--------+--------+--------+--------+
/// |mmmbbbbb|bbbbbbbb|SHDCrrrr|xxpppppp|
/// +--------+--------+--------+--------+
///
/// p = prime number of rank (deuce=2,trey=3,four=5,...,ace=41)
/// r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
/// SHDC = suit of card (bit turned on based on suit of card)
/// b = bit turned on depending on rank of card
/// m = Flags reserved for multiples of the same rank. Stripped for evals.
/// ```
pub type CKCNumber = u32;

/// u32 constants for all 52 cards in a standard poker deck.
pub struct CardNumber;

impl CardNumber {
    pub const RANK_FLAG_FILTER: u32 = 0x1FFF0000; // 536805376 aka 0b00011111_11111111_00000000_00000000
    pub const RANK_FLAG_SHIFT: u32 = 16;
    pub const RANK_PRIME_FILTER: u32 = 0b00111111;

    /// Binary filter for `CardNumber` `Suit` flags.
    /// 00000000 00000000 11110000 00000000
    pub const SUIT_FILTER: u32 = 0xF000; // 61440 aka 0b11110000_00000000
    pub const SUIT_SHORT_MASK: u32 = 0b1111;
    pub const SUIT_SHIFT: u32 = 12;

    //region multiples

    /// These flags are used to give sorting priority when more than one card
    /// of a specific rank is present.
    pub const PAIR: u32 = 536_870_912;
    pub const TRIPS: u32 = 1_073_741_824;
    pub const QUADS: u32 = 2_147_483_648;
    pub const MULTIPLES_FILTER: u32 = 536_870_911;

    //endregion

    //region cardnumbers
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

    #[must_use]
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
    BLANK,
}

impl CardRank {
    #[must_use]
    pub fn from_char(index: char) -> CardRank {
        match index {
            'A' | 'a' => CardRank::ACE,
            'K' | 'k' => CardRank::KING,
            'Q' | 'q' => CardRank::QUEEN,
            'J' | 'j' => CardRank::JACK,
            'T' | 't' | '0' => CardRank::TEN,
            '9' => CardRank::NINE,
            '8' => CardRank::EIGHT,
            '7' => CardRank::SEVEN,
            '6' => CardRank::SIX,
            '5' => CardRank::FIVE,
            '4' => CardRank::FOUR,
            '3' => CardRank::THREE,
            '2' => CardRank::TWO,
            _ => CardRank::BLANK,
        }
    }

    fn bits(self) -> u32 {
        1 << (16 + self.number())
    }

    fn number(self) -> u32 {
        match self {
            CardRank::ACE => 12,
            CardRank::KING => 11,
            CardRank::QUEEN => 10,
            CardRank::JACK => 9,
            CardRank::TEN => 8,
            CardRank::NINE => 7,
            CardRank::EIGHT => 6,
            CardRank::SEVEN => 5,
            CardRank::SIX => 4,
            CardRank::FIVE => 3,
            CardRank::FOUR => 2,
            CardRank::THREE => 1,
            _ => 0,
        }
    }

    fn prime(self) -> u32 {
        match self {
            CardRank::ACE => 41,
            CardRank::KING => 37,
            CardRank::QUEEN => 31,
            CardRank::JACK => 29,
            CardRank::TEN => 23,
            CardRank::NINE => 19,
            CardRank::EIGHT => 17,
            CardRank::SEVEN => 13,
            CardRank::SIX => 11,
            CardRank::FIVE => 7,
            CardRank::FOUR => 5,
            CardRank::THREE => 3,
            CardRank::TWO => 2,
            CardRank::BLANK => 0,
        }
    }

    fn shift8(self) -> u32 {
        self.number() << 8
    }
}

#[cfg(test)]
mod card_rank_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case('A', CardRank::ACE)]
    #[case('a', CardRank::ACE)]
    #[case('K', CardRank::KING)]
    #[case('k', CardRank::KING)]
    #[case('Q', CardRank::QUEEN)]
    #[case('q', CardRank::QUEEN)]
    #[case('J', CardRank::JACK)]
    #[case('j', CardRank::JACK)]
    #[case('T', CardRank::TEN)]
    #[case('t', CardRank::TEN)]
    #[case('0', CardRank::TEN)]
    #[case('9', CardRank::NINE)]
    #[case('8', CardRank::EIGHT)]
    #[case('7', CardRank::SEVEN)]
    #[case('6', CardRank::SIX)]
    #[case('5', CardRank::FIVE)]
    #[case('4', CardRank::FOUR)]
    #[case('3', CardRank::THREE)]
    #[case('2', CardRank::TWO)]
    #[case('_', CardRank::BLANK)]
    #[case(' ', CardRank::BLANK)]
    fn from_char(#[case] input: char, #[case] expected: CardRank) {
        assert_eq!(expected, CardRank::from_char(input));
    }
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq)]
pub enum CardSuit {
    SPADES,
    HEARTS,
    DIAMONDS,
    CLUBS,
    BLANK,
}

impl CardSuit {
    #[must_use]
    pub fn binary_signature(&self) -> u32 {
        match self {
            CardSuit::SPADES => 0x8000,
            CardSuit::HEARTS => 0x4000,
            CardSuit::DIAMONDS => 0x2000,
            CardSuit::CLUBS => 0x1000,
            CardSuit::BLANK => 0,
        }
    }

    #[must_use]
    pub fn from_char(symbol: char) -> CardSuit {
        match symbol {
            '♤' | '♠' | 'S' | 's' => CardSuit::SPADES,
            '♡' | '♥' | 'H' | 'h' => CardSuit::HEARTS,
            '♢' | '♦' | 'D' | 'd' => CardSuit::DIAMONDS,
            '♧' | '♣' | 'C' | 'c' => CardSuit::CLUBS,
            _ => CardSuit::BLANK,
        }
    }
}

#[cfg(test)]
mod card_suit_tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn binary_signature() {
        assert_eq!(32768, CardSuit::SPADES.binary_signature());
        assert_eq!(16384, CardSuit::HEARTS.binary_signature());
        assert_eq!(8192, CardSuit::DIAMONDS.binary_signature());
        assert_eq!(4096, CardSuit::CLUBS.binary_signature());
        assert_eq!(0, CardSuit::BLANK.binary_signature());
    }

    #[rstest]
    #[case('♠', CardSuit::SPADES)]
    #[case('S', CardSuit::SPADES)]
    #[case('s', CardSuit::SPADES)]
    #[case('♥', CardSuit::HEARTS)]
    #[case('H', CardSuit::HEARTS)]
    #[case('h', CardSuit::HEARTS)]
    #[case('♦', CardSuit::DIAMONDS)]
    #[case('D', CardSuit::DIAMONDS)]
    #[case('d', CardSuit::DIAMONDS)]
    #[case('♣', CardSuit::CLUBS)]
    #[case('C', CardSuit::CLUBS)]
    #[case('c', CardSuit::CLUBS)]
    #[case(' ', CardSuit::BLANK)]
    #[case('F', CardSuit::BLANK)]
    fn from_char(#[case] input: char, #[case] expected: CardSuit) {
        assert_eq!(expected, CardSuit::from_char(input));
    }
}

pub mod evaluate {
    use crate::hand_rank::HandRankValue;
    use crate::{CKCNumber, CardNumber};

    pub const POSSIBLE_COMBINATIONS: usize = 7937;

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
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
            & CardNumber::SUIT_FILTER)
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

    #[allow(clippy::cast_possible_truncation)]
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

#[derive(Debug, PartialEq)]
pub enum HandError {
    BlankCard,
    DuplicateCard,
    Incomplete,
    InvalidCard,
    InvalidCardCount,
    InvalidIndex,
    NotEnoughCards,
    TooManyCards,
}

pub trait PokerCard {
    //region static

    #[must_use]
    fn create(rank: CardRank, suit: CardSuit) -> CKCNumber {
        CKCNumber::filter(rank.bits() | rank.prime() | rank.shift8() | suit.binary_signature())
    }

    /// Only allows you to create a `CKCNumber` that is valid.
    #[must_use]
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

    #[must_use]
    fn from_index(index: &str) -> CKCNumber {
        let (rank, suit) = get_rank_and_suit(index);
        CKCNumber::create(rank, suit)
    }

    //endregion

    fn as_u32(&self) -> u32;

    fn get_card_rank(&self) -> CardRank {
        match self.get_rank_bit() {
            4096 => CardRank::ACE,
            2048 => CardRank::KING,
            1024 => CardRank::QUEEN,
            512 => CardRank::JACK,
            256 => CardRank::TEN,
            128 => CardRank::NINE,
            64 => CardRank::EIGHT,
            32 => CardRank::SEVEN,
            16 => CardRank::SIX,
            8 => CardRank::FIVE,
            4 => CardRank::FOUR,
            2 => CardRank::THREE,
            1 => CardRank::TWO,
            _ => CardRank::BLANK,
        }
    }

    fn get_card_suit(&self) -> CardSuit {
        match self.get_suit_bit() {
            8 => CardSuit::SPADES,
            4 => CardSuit::HEARTS,
            2 => CardSuit::DIAMONDS,
            1 => CardSuit::CLUBS,
            _ => CardSuit::BLANK,
        }
    }

    fn get_rank_bit(&self) -> u32 {
        self.get_rank_flag() >> CardNumber::RANK_FLAG_SHIFT
    }

    fn get_rank_char(&self) -> char {
        match self.get_rank_bit() {
            4096 => 'A',
            2048 => 'K',
            1024 => 'Q',
            512 => 'J',
            256 => 'T',
            128 => '9',
            64 => '8',
            32 => '7',
            16 => '6',
            8 => '5',
            4 => '4',
            2 => '3',
            1 => '2',
            _ => '_',
        }
    }

    fn get_rank_flag(&self) -> u32 {
        self.as_u32() & CardNumber::RANK_FLAG_FILTER
    }

    fn get_rank_prime(&self) -> u32 {
        self.as_u32() & CardNumber::RANK_PRIME_FILTER
    }

    fn get_suit_bit(&self) -> u32 {
        self.get_suit_flag() >> CardNumber::SUIT_SHIFT
    }

    fn get_suit_char(&self) -> char {
        match self.get_suit_bit() {
            8 => '♠',
            4 => '♥',
            2 => '♦',
            1 => '♣',
            _ => '_',
        }
    }

    fn get_suit_letter(&self) -> char {
        match self.get_suit_bit() {
            8 => 'S',
            4 => 'H',
            2 => 'D',
            1 => 'C',
            _ => '_',
        }
    }

    fn get_suit_flag(&self) -> u32 {
        self.as_u32() & CardNumber::SUIT_FILTER
    }

    fn is_blank(&self) -> bool;

    //region multiples

    fn flag_as_pair(&self) -> CKCNumber {
        self.as_u32() | CardNumber::PAIR
    }

    fn flag_as_trips(&self) -> CKCNumber {
        self.as_u32() | CardNumber::TRIPS
    }

    fn flag_as_quads(&self) -> CKCNumber {
        self.as_u32() | CardNumber::QUADS
    }

    fn strip_multiples_flags(&self) -> CKCNumber {
        CardNumber::MULTIPLES_FILTER & self.as_u32()
    }

    //endregion
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
    use rstest::rstest;

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

    #[rstest]
    #[case("A♠", CardNumber::ACE_SPADES)]
    #[case("ks", CardNumber::KING_SPADES)]
    #[case("QS", CardNumber::QUEEN_SPADES)]
    #[case("J♠", CardNumber::JACK_SPADES)]
    #[case("TS", CardNumber::TEN_SPADES)]
    #[case("9s", CardNumber::NINE_SPADES)]
    #[case("8♠", CardNumber::EIGHT_SPADES)]
    #[case("7S", CardNumber::SEVEN_SPADES)]
    #[case("6♠", CardNumber::SIX_SPADES)]
    #[case("5S", CardNumber::FIVE_SPADES)]
    #[case("4♠", CardNumber::FOUR_SPADES)]
    #[case("3s", CardNumber::TREY_SPADES)]
    #[case("2S", CardNumber::DEUCE_SPADES)]
    #[case("A♥", CardNumber::ACE_HEARTS)]
    #[case("k♥", CardNumber::KING_HEARTS)]
    #[case("QH", CardNumber::QUEEN_HEARTS)]
    #[case("jh", CardNumber::JACK_HEARTS)]
    #[case("T♥", CardNumber::TEN_HEARTS)]
    #[case("9♥", CardNumber::NINE_HEARTS)]
    #[case("8h", CardNumber::EIGHT_HEARTS)]
    #[case("7H", CardNumber::SEVEN_HEARTS)]
    #[case("6h", CardNumber::SIX_HEARTS)]
    #[case("5H", CardNumber::FIVE_HEARTS)]
    #[case("4♥", CardNumber::FOUR_HEARTS)]
    #[case("3♥", CardNumber::TREY_HEARTS)]
    #[case("2h", CardNumber::DEUCE_HEARTS)]
    #[case("A♦", CardNumber::ACE_DIAMONDS)]
    #[case("k♦", CardNumber::KING_DIAMONDS)]
    #[case("Q♦", CardNumber::QUEEN_DIAMONDS)]
    #[case("Jd", CardNumber::JACK_DIAMONDS)]
    #[case("tD", CardNumber::TEN_DIAMONDS)]
    #[case("9♦", CardNumber::NINE_DIAMONDS)]
    #[case("8D", CardNumber::EIGHT_DIAMONDS)]
    #[case("7♦", CardNumber::SEVEN_DIAMONDS)]
    #[case("6D", CardNumber::SIX_DIAMONDS)]
    #[case("5D", CardNumber::FIVE_DIAMONDS)]
    #[case("4♦", CardNumber::FOUR_DIAMONDS)]
    #[case("3♦", CardNumber::TREY_DIAMONDS)]
    #[case("2d", CardNumber::DEUCE_DIAMONDS)]
    #[case("a♣", CardNumber::ACE_CLUBS)]
    #[case("k♣", CardNumber::KING_CLUBS)]
    #[case("QC", CardNumber::QUEEN_CLUBS)]
    #[case("jc", CardNumber::JACK_CLUBS)]
    #[case("tC", CardNumber::TEN_CLUBS)]
    #[case("9♣", CardNumber::NINE_CLUBS)]
    #[case("8♣", CardNumber::EIGHT_CLUBS)]
    #[case("7c", CardNumber::SEVEN_CLUBS)]
    #[case("6♣", CardNumber::SIX_CLUBS)]
    #[case("5C", CardNumber::FIVE_CLUBS)]
    #[case("4c", CardNumber::FOUR_CLUBS)]
    #[case("3C", CardNumber::TREY_CLUBS)]
    #[case("2C", CardNumber::DEUCE_CLUBS)]
    fn from_index(#[case] index: &str, #[case] expected: CKCNumber) {
        assert_eq!(CKCNumber::from_index(index), expected);
    }

    #[test]
    fn get_rank() {
        let card = CardNumber::ACE_CLUBS as CKCNumber;
        assert_eq!(0b00010000_00000000, card.get_rank_bit());
        assert_eq!(0b00101001, card.get_rank_prime());
        assert_eq!(CardRank::ACE, card.get_card_rank());

        let card = CardNumber::KING_DIAMONDS as CKCNumber;
        assert_eq!(0b00001000_00000000, card.get_rank_bit());
        assert_eq!(0b00100101, card.get_rank_prime());
        assert_eq!(CardRank::KING, card.get_card_rank());

        let card = CardNumber::QUEEN_SPADES as CKCNumber;
        assert_eq!(0b00000100_00000000, card.get_rank_bit());
        assert_eq!(0b00011111, card.get_rank_prime());
        assert_eq!(CardRank::QUEEN, card.get_card_rank());

        let card = CardNumber::JACK_HEARTS as CKCNumber;
        assert_eq!(0b00000010_00000000, card.get_rank_bit());
        assert_eq!(0b00011101, card.get_rank_prime());
        assert_eq!(CardRank::JACK, card.get_card_rank());

        let card = CardNumber::TEN_SPADES as CKCNumber;
        assert_eq!(0b00000001_00000000, card.get_rank_bit());
        assert_eq!(0b00010111, card.get_rank_prime());
        assert_eq!(CardRank::TEN, card.get_card_rank());

        let card = CardNumber::NINE_HEARTS as CKCNumber;
        assert_eq!(0b00000000_10000000, card.get_rank_bit());
        assert_eq!(0b00010011, card.get_rank_prime());
        assert_eq!(CardRank::NINE, card.get_card_rank());

        let card = CardNumber::EIGHT_DIAMONDS as CKCNumber;
        assert_eq!(0b00000000_01000000, card.get_rank_bit());
        assert_eq!(0b00010001, card.get_rank_prime());
        assert_eq!(CardRank::EIGHT, card.get_card_rank());

        let card = CardNumber::SEVEN_CLUBS as CKCNumber;
        assert_eq!(0b00000000_00100000, card.get_rank_bit());
        assert_eq!(0b00001101, card.get_rank_prime());
        assert_eq!(CardRank::SEVEN, card.get_card_rank());

        let card = CardNumber::SIX_SPADES as CKCNumber;
        assert_eq!(0b00000000_00010000, card.get_rank_bit());
        assert_eq!(0b00001011, card.get_rank_prime());
        assert_eq!(CardRank::SIX, card.get_card_rank());

        let card = CardNumber::FIVE_HEARTS as CKCNumber;
        assert_eq!(0b00000000_00001000, card.get_rank_bit());
        assert_eq!(0b00000111, card.get_rank_prime());
        assert_eq!(CardRank::FIVE, card.get_card_rank());

        let card = CardNumber::FOUR_DIAMONDS as CKCNumber;
        assert_eq!(0b00000000_00000100, card.get_rank_bit());
        assert_eq!(0b00000101, card.get_rank_prime());
        assert_eq!(CardRank::FOUR, card.get_card_rank());

        let card = CardNumber::TREY_CLUBS as CKCNumber;
        assert_eq!(0b00000000_00000010, card.get_rank_bit());
        assert_eq!(0b00000011, card.get_rank_prime());
        assert_eq!(CardRank::THREE, card.get_card_rank());

        let card = CardNumber::DEUCE_SPADES as CKCNumber;
        assert_eq!(0b00000000_00000001, card.get_rank_bit());
        assert_eq!(0b00000010, card.get_rank_prime());
        assert_eq!(CardRank::TWO, card.get_card_rank());

        let card = CardNumber::BLANK as CKCNumber;
        assert_eq!(0b00000000_00000000, card.get_rank_bit());
        assert_eq!(CardRank::BLANK, card.get_card_rank());
        assert_eq!(CardSuit::BLANK, card.get_card_suit());
    }

    #[test]
    fn get_rank_flag() {
        let card = CardNumber::ACE_CLUBS as CKCNumber;
        assert_eq!(0b00010000_00000000_00000000_00000000, card.get_rank_flag());
        let card = CardNumber::KING_DIAMONDS as CKCNumber;
        assert_eq!(0b00001000_00000000_00000000_00000000, card.get_rank_flag());
        let card = CardNumber::QUEEN_SPADES as CKCNumber;
        assert_eq!(0b00000100_00000000_00000000_00000000, card.get_rank_flag());
        let card = CardNumber::JACK_HEARTS as CKCNumber;
        assert_eq!(0b00000010_00000000_00000000_00000000, card.get_rank_flag());
        let card = CardNumber::TEN_SPADES as CKCNumber;
        assert_eq!(0b00000001_00000000_00000000_00000000, card.get_rank_flag());
        let card = CardNumber::NINE_HEARTS as CKCNumber;
        assert_eq!(0b00000000_10000000_00000000_00000000, card.get_rank_flag());
        let card = CardNumber::EIGHT_DIAMONDS as CKCNumber;
        assert_eq!(0b00000000_01000000_00000000_00000000, card.get_rank_flag());
        let card = CardNumber::SEVEN_CLUBS as CKCNumber;
        assert_eq!(0b00000000_00100000_00000000_00000000, card.get_rank_flag());
        let card = CardNumber::SIX_SPADES as CKCNumber;
        assert_eq!(0b00000000_00010000_00000000_00000000, card.get_rank_flag());
        let card = CardNumber::FIVE_HEARTS as CKCNumber;
        assert_eq!(0b00000000_00001000_00000000_00000000, card.get_rank_flag());
        let card = CardNumber::FOUR_DIAMONDS as CKCNumber;
        assert_eq!(0b00000000_00000100_00000000_00000000, card.get_rank_flag());
        let card = CardNumber::TREY_CLUBS as CKCNumber;
        assert_eq!(0b00000000_00000010_00000000_00000000, card.get_rank_flag());
        let card = CardNumber::DEUCE_SPADES as CKCNumber;
        assert_eq!(0b00000000_00000001_00000000_00000000, card.get_rank_flag());
    }

    #[test]
    fn get_suit_bit() {
        let card = CardNumber::SEVEN_SPADES as CKCNumber;
        assert_eq!(0b1000, card.get_suit_bit());
        assert_eq!(CardSuit::SPADES, card.get_card_suit());

        let card = CardNumber::SEVEN_HEARTS as CKCNumber;
        assert_eq!(0b0100, card.get_suit_bit());
        assert_eq!(CardSuit::HEARTS, card.get_card_suit());

        let card = CardNumber::SEVEN_DIAMONDS as CKCNumber;
        assert_eq!(0b0010, card.get_suit_bit());
        assert_eq!(CardSuit::DIAMONDS, card.get_card_suit());

        let card = CardNumber::SEVEN_CLUBS as CKCNumber;
        assert_eq!(0b0001, card.get_suit_bit());
        assert_eq!(CardSuit::CLUBS, card.get_card_suit());
    }

    #[test]
    fn get_suit_flag() {
        let card = CardNumber::SEVEN_SPADES as CKCNumber;
        assert_eq!(0b10000000_00000000, card.get_suit_flag());

        let card = CardNumber::SEVEN_HEARTS as CKCNumber;
        assert_eq!(0b01000000_00000000, card.get_suit_flag());

        let card = CardNumber::SEVEN_DIAMONDS as CKCNumber;
        assert_eq!(0b00100000_00000000, card.get_suit_flag());

        let card = CardNumber::SEVEN_CLUBS as CKCNumber;
        assert_eq!(0b00010000_00000000, card.get_suit_flag());
    }

    #[test]
    fn is_blank() {
        let card = CardNumber::BLANK;

        assert!(card.is_blank());
        assert!(0_u32.is_blank());
        assert!(0.is_blank());
    }

    #[test]
    fn flag_as_pair() {
        assert_eq!(805_342_249, CardNumber::ACE_SPADES.flag_as_pair());
    }

    #[test]
    fn flag_as_trips() {
        assert_eq!(1_342_213_161, CardNumber::ACE_SPADES.flag_as_trips());
    }

    #[test]
    fn flag_as_quads() {
        assert_eq!(2_415_954_985, CardNumber::ACE_SPADES.flag_as_quads());
    }

    #[test]
    fn strip_multiples_flags() {
        assert_eq!(
            CardNumber::ACE_SPADES,
            CardNumber::ACE_SPADES
                .flag_as_pair()
                .strip_multiples_flags()
        );
        assert_eq!(
            CardNumber::ACE_SPADES,
            CardNumber::ACE_SPADES
                .flag_as_trips()
                .strip_multiples_flags()
        );
        assert_eq!(
            CardNumber::ACE_SPADES,
            CardNumber::ACE_SPADES
                .flag_as_quads()
                .strip_multiples_flags()
        );
        assert_eq!(
            CardNumber::ACE_SPADES,
            CardNumber::ACE_SPADES
                .flag_as_pair()
                .flag_as_trips()
                .flag_as_quads()
                .strip_multiples_flags()
        );
    }

    #[test]
    fn scratch() {
        // let paired = 0b10010000000000001000110000101001
    }
}
