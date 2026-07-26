use crate::CkcError;
use crate::standard52::card_number::CardNumber;
use crate::standard52::rank::Rank;
use crate::standard52::suit::{Suit, SuitShift};
use core::fmt;
use core::str::FromStr;

#[cfg(feature = "alloc")]
use alloc::{format, string::String};

/// A `Card` is a [`Newtype`](https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html)
/// u32 representation of a variant of Cactus Kev's binary representation of a poker card as
/// designed for rapid hand evaluation as documented [here](https://suffe.cool/poker/evaluator.html).
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
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Card(#[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_card_index"))] u32);

impl Card {
    //region binary filters
    pub const RANK_FLAG_FILTER: u32 = 0x1FFF_0000; // 536805376 aka 0b00011111_11111111_00000000_00000000
    pub const RANK_FLAG_SHIFT: u32 = 16;
    pub const RANK_PRIME_FILTER: u32 = 0b0011_1111;

    /// Binary filter for `CardNumber` `Suit` flags.
    /// 00000000 00000000 11110000 00000000
    pub const SUIT_FLAG_FILTER: u32 = 0xF000; // 61440 aka 0b11110000_00000000
    pub const SUIT_SHORT_MASK: u32 = 0b1111;
    pub const SUIT_FLAG_SHIFT: u32 = 12;

    /// Frequency Weight masks
    pub const FREQUENCY_PAIRED_MASK: u32 = 0b0010_0000_0000_0000_0000_0000_0000_0000;
    pub const FREQUENCY_TRIPPED_MASK: u32 = 0b0100_0000_0000_0000_0000_0000_0000_0000;
    pub const FREQUENCY_QUADED_MASK: u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
    pub const FREQUENCY_MASK: u32 = 0b1110_0000_0000_0000_0000_0000_0000_0000;
    pub const FREQUENCY_MASK_FILTER: u32 = 0b0001_1111_1111_1111_1111_1111_1111_1111;

    pub(crate) const BLANK_NUMBER: u32 = 0;
    //endregion

    //region cards
    pub const ACE_SPADES: Card = Card(CardNumber::AceSpades as u32);
    pub const KING_SPADES: Card = Card(CardNumber::KingSpades as u32);
    pub const QUEEN_SPADES: Card = Card(CardNumber::QueenSpades as u32);
    pub const JACK_SPADES: Card = Card(CardNumber::JackSpades as u32);
    pub const TEN_SPADES: Card = Card(CardNumber::TenSpades as u32);
    pub const NINE_SPADES: Card = Card(CardNumber::NineSpades as u32);
    pub const EIGHT_SPADES: Card = Card(CardNumber::EightSpades as u32);
    pub const SEVEN_SPADES: Card = Card(CardNumber::SevenSpades as u32);
    pub const SIX_SPADES: Card = Card(CardNumber::SixSpades as u32);
    pub const FIVE_SPADES: Card = Card(CardNumber::FiveSpades as u32);
    pub const FOUR_SPADES: Card = Card(CardNumber::FourSpades as u32);
    pub const TREY_SPADES: Card = Card(CardNumber::TreySpades as u32);
    pub const DEUCE_SPADES: Card = Card(CardNumber::DeuceSpades as u32);
    pub const ACE_HEARTS: Card = Card(CardNumber::AceHearts as u32);
    pub const KING_HEARTS: Card = Card(CardNumber::KingHearts as u32);
    pub const QUEEN_HEARTS: Card = Card(CardNumber::QueenHearts as u32);
    pub const JACK_HEARTS: Card = Card(CardNumber::JackHearts as u32);
    pub const TEN_HEARTS: Card = Card(CardNumber::TenHearts as u32);
    pub const NINE_HEARTS: Card = Card(CardNumber::NineHearts as u32);
    pub const EIGHT_HEARTS: Card = Card(CardNumber::EightHearts as u32);
    pub const SEVEN_HEARTS: Card = Card(CardNumber::SevenHearts as u32);
    pub const SIX_HEARTS: Card = Card(CardNumber::SixHearts as u32);
    pub const FIVE_HEARTS: Card = Card(CardNumber::FiveHearts as u32);
    pub const FOUR_HEARTS: Card = Card(CardNumber::FourHearts as u32);
    pub const TREY_HEARTS: Card = Card(CardNumber::TreyHearts as u32);
    pub const DEUCE_HEARTS: Card = Card(CardNumber::DeuceHearts as u32);
    pub const ACE_DIAMONDS: Card = Card(CardNumber::AceDiamonds as u32);
    pub const KING_DIAMONDS: Card = Card(CardNumber::KingDiamonds as u32);
    pub const QUEEN_DIAMONDS: Card = Card(CardNumber::QueenDiamonds as u32);
    pub const JACK_DIAMONDS: Card = Card(CardNumber::JackDiamonds as u32);
    pub const TEN_DIAMONDS: Card = Card(CardNumber::TenDiamonds as u32);
    pub const NINE_DIAMONDS: Card = Card(CardNumber::NineDiamonds as u32);
    pub const EIGHT_DIAMONDS: Card = Card(CardNumber::EightDiamonds as u32);
    pub const SEVEN_DIAMONDS: Card = Card(CardNumber::SevenDiamonds as u32);
    pub const SIX_DIAMONDS: Card = Card(CardNumber::SixDiamonds as u32);
    pub const FIVE_DIAMONDS: Card = Card(CardNumber::FiveDiamonds as u32);
    pub const FOUR_DIAMONDS: Card = Card(CardNumber::FourDiamonds as u32);
    pub const TREY_DIAMONDS: Card = Card(CardNumber::TreyDiamonds as u32);
    pub const DEUCE_DIAMONDS: Card = Card(CardNumber::DeuceDiamonds as u32);
    pub const ACE_CLUBS: Card = Card(CardNumber::AceClubs as u32);
    pub const KING_CLUBS: Card = Card(CardNumber::KingClubs as u32);
    pub const QUEEN_CLUBS: Card = Card(CardNumber::QueenClubs as u32);
    pub const JACK_CLUBS: Card = Card(CardNumber::JackClubs as u32);
    pub const TEN_CLUBS: Card = Card(CardNumber::TenClubs as u32);
    pub const NINE_CLUBS: Card = Card(CardNumber::NineClubs as u32);
    pub const EIGHT_CLUBS: Card = Card(CardNumber::EightClubs as u32);
    pub const SEVEN_CLUBS: Card = Card(CardNumber::SevenClubs as u32);
    pub const SIX_CLUBS: Card = Card(CardNumber::SixClubs as u32);
    pub const FIVE_CLUBS: Card = Card(CardNumber::FiveClubs as u32);
    pub const FOUR_CLUBS: Card = Card(CardNumber::FourClubs as u32);
    pub const TREY_CLUBS: Card = Card(CardNumber::TreyClubs as u32);
    pub const DEUCE_CLUBS: Card = Card(CardNumber::DeuceClubs as u32);
    pub const BLANK: Card = Card(Card::BLANK_NUMBER);

    #[cfg(feature = "alloc")]
    const GUIDE: &'static str = "xxxAKQJT 98765432 ♠♥♦♣rrrr xxpppppp";
    //endregion

    #[must_use]
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self::from(rank.bits() | rank.prime() | rank.shift8() | suit.binary_signature())
    }

    /// # Errors
    ///
    /// Returns `CkcError::BlankCard` if the `Card` is blank.
    pub fn filter(card: Card) -> Result<Self, CkcError> {
        match card {
            Card::BLANK => Err(CkcError::BlankCard),
            _ => Ok(card),
        }
    }

    /// Returns the Cactus Kev Card u32 number of the `Card`.
    #[must_use]
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    /// Strips the multiple-of-a-rank flags, which are set during evaluation and
    /// must not survive into a returned hand.
    ///
    /// Ported from pkcore's `impl Pile for Card` (`pkcore/src/card.rs:320`); `Pile`
    /// itself does not follow the kernel down, but `Five::clean()` needs this.
    #[must_use]
    pub fn clean(&self) -> Self {
        Card(self.0 & Card::FREQUENCY_MASK_FILTER)
    }

    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn bit_string(&self) -> String {
        let b = format!("{:b}", self.0);
        // OK, let's take a moment to really stan on the rust std libraries. The fmt
        // [Fill/Alignment](https://doc.rust-lang.org/std/fmt/#fillalignment) is FIRE!
        let b = format!("{b:0>32}");
        let mut bit_string = String::with_capacity(34);

        for (i, c) in b.chars().enumerate() {
            bit_string.push(c);
            if i % 8 == 7 && i % 31 != 0 {
                bit_string.push(' ');
            }
        }
        bit_string
    }

    /// This code is doing too much. I need to Uncle Bob it. Aside on why I am giving up
    /// that phrase.
    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn bit_string_guided(&self) -> String {
        format!("{}\n{}", Card::GUIDE, self.bit_string())
    }

    //region frequency methods

    /// Returns a new version of `Card` with the paired frequency bit set.
    #[must_use]
    pub fn frequency_paired(&self) -> Card {
        Card(self.0 | Card::FREQUENCY_PAIRED_MASK)
    }

    /// Returns a new version of `Card` with the tripped frequency bit set.
    #[must_use]
    pub fn frequency_tripped(&self) -> Card {
        Card(self.0 | Card::FREQUENCY_TRIPPED_MASK)
    }

    /// Returns a new version of `Card` with the quaded frequency bit set.
    ///
    /// Quaded??!!
    #[must_use]
    pub fn frequency_quaded(&self) -> Card {
        Card(self.0 | Card::FREQUENCY_QUADED_MASK)
    }

    //endregion

    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn get_letter_index(&self) -> String {
        format!("{}{}", self.get_rank().to_char(), self.get_suit().to_char_letter())
    }

    #[must_use]
    pub fn get_rank(&self) -> Rank {
        match self.get_rank_bit() {
            4096 => Rank::ACE,
            2048 => Rank::KING,
            1024 => Rank::QUEEN,
            512 => Rank::JACK,
            256 => Rank::TEN,
            128 => Rank::NINE,
            64 => Rank::EIGHT,
            32 => Rank::SEVEN,
            16 => Rank::SIX,
            8 => Rank::FIVE,
            4 => Rank::FOUR,
            2 => Rank::TREY,
            1 => Rank::DEUCE,
            _ => Rank::BLANK,
        }
    }

    fn get_rank_bit(self) -> u32 {
        self.get_rank_flag() >> Card::RANK_FLAG_SHIFT
    }

    fn get_rank_flag(self) -> u32 {
        self.as_u32() & Card::RANK_FLAG_FILTER
    }

    #[must_use]
    pub fn get_rank_prime(&self) -> u32 {
        self.as_u32() & Card::RANK_PRIME_FILTER
    }

    #[must_use]
    pub fn get_suit(&self) -> Suit {
        match self.get_suit_bit() {
            8 => Suit::SPADES,
            4 => Suit::HEARTS,
            2 => Suit::DIAMONDS,
            1 => Suit::CLUBS,
            _ => Suit::BLANK,
        }
    }

    fn get_suit_bit(self) -> u32 {
        self.get_suit_flag() >> Card::SUIT_FLAG_SHIFT
    }

    fn get_suit_flag(self) -> u32 {
        self.as_u32() & Card::SUIT_FLAG_FILTER
    }

    #[must_use]
    pub fn is_flagged(&self, flag: u32) -> bool {
        (self.as_u32() & flag) == flag
    }
}

impl fmt::Display for Card {
    /// ```
    /// use ckc_rs::prelude::*;
    ///
    /// assert_eq!("A♠", Card::ACE_SPADES.to_string());
    /// assert_eq!("__", Card::BLANK.to_string());
    ///
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.get_rank().to_char(), self.get_suit().to_char_symbol())
    }
}

/// Filters u32 so that only valid Cactus Kev Card values are set.
impl From<u32> for Card {
    fn from(ckc_number: u32) -> Self {
        let r: Result<CardNumber, CkcError> = ckc_number.try_into();
        match r {
            Ok(u) => Card(u as u32),
            _ => Card::BLANK,
        }
    }
}

impl FromStr for Card {
    type Err = CkcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.trim().chars();
        let rank: Rank = match chars.next() {
            None => return Err(CkcError::InvalidIndex),
            Some(r) => {
                let rank = Rank::from(r);
                if rank == Rank::BLANK {
                    return Err(CkcError::InvalidIndex);
                }
                rank
            },
        };
        let suit: Suit = match chars.next() {
            None => return Err(CkcError::InvalidIndex),
            Some(s) => {
                let suit = Suit::from(s);
                if suit == Suit::BLANK {
                    return Err(CkcError::InvalidIndex);
                }
                suit
            },
        };
        Ok(Card::new(rank, suit))
    }
}

#[cfg(feature = "serde")]
impl serde::ser::Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_newtype_struct("Card", &alloc::string::ToString::to_string(self))
    }
}

#[cfg(feature = "serde")]
fn deserialize_card_index<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let buf = <String as serde::Deserialize>::deserialize(deserializer)?;

    match Card::from_str(buf.as_str()) {
        Ok(card) => Ok(card.as_u32()),
        Err(_) => Ok(0),
    }
}

impl SuitShift for Card {
    fn shift_suit_down(&self) -> Self {
        Card::new(self.get_rank(), self.get_suit().shift_suit_down())
    }

    fn shift_suit_up(&self) -> Self {
        Card::new(self.get_rank(), self.get_suit().shift_suit_up())
    }

    fn opposite(&self) -> Self {
        Card::new(self.get_rank(), self.get_suit().opposite())
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn every_card_number_makes_a_card() {
        for cn in CardNumber::iter() {
            let card = Card::from(*cn as u32);
            assert_eq!(card.as_u32(), *cn as u32);
            assert_ne!(card, Card::BLANK);
        }
    }

    /// Strengthened per Task 6 brief item 3: composing every `Rank` x `Suit`
    /// pair must reproduce the exact matching `CardNumber` constant. If any
    /// of the 52 disagree, a `Rank`/`Suit` method or a `Card` const was
    /// transcribed wrong when porting from pkcore.
    #[test]
    fn new_composes_the_cactus_kev_number() {
        for rank in Rank::ALL {
            for suit in Suit::ALL {
                let composed = Card::new(rank, suit);
                assert!(
                    CardNumber::ALL.iter().any(|cn| *cn as u32 == composed.as_u32()),
                    "{rank:?} of {suit:?} composed to {:#010x}, which is not a CardNumber",
                    composed.as_u32()
                );
                let expected = Card::from(CardNumber::try_from(composed.as_u32()).unwrap() as u32);
                assert_eq!(composed, expected, "{rank:?} of {suit:?}");
            }
        }
        assert_eq!(Card::new(Rank::ACE, Suit::SPADES), Card::ACE_SPADES);
        assert_eq!(Card::new(Rank::DEUCE, Suit::CLUBS), Card::DEUCE_CLUBS);
    }

    #[test]
    fn new_with_blank_yields_blank() {
        assert_eq!(Card::TREY_CLUBS, Card::new(Rank::TREY, Suit::CLUBS));
        assert_eq!(Card::BLANK, Card::new(Rank::BLANK, Suit::CLUBS));
        assert_eq!(Card::BLANK, Card::new(Rank::TREY, Suit::BLANK));
        assert_eq!(Card::BLANK, Card::new(Rank::BLANK, Suit::BLANK));
    }

    #[test]
    fn filter_rejects_blank() {
        assert_eq!(Card::filter(Card::BLANK), Err(CkcError::BlankCard));
        assert_eq!(Card::filter(Card::NINE_CLUBS), Ok(Card::NINE_CLUBS));
    }

    #[test]
    fn rank_and_suit_round_trip() {
        for rank in Rank::iter() {
            for suit in Suit::iter() {
                let card = Card::new(*rank, *suit);
                assert_eq!(card.get_rank(), *rank);
                assert_eq!(card.get_suit(), *suit);
            }
        }
    }

    #[test]
    fn as_u32() {
        assert_eq!(CardNumber::AceSpades as u32, Card::ACE_SPADES.as_u32());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn binary_string() {
        let expected = "00000001 00000000 10001000 00010111";
        let card = Card::from_str("T♠").unwrap();

        assert_eq!(expected, card.bit_string());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn bit_string() {
        assert_eq!("00010000 00000000 10001100 00101001", Card::ACE_SPADES.bit_string());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn bit_string_guided() {
        assert_eq!(
            "xxxAKQJT 98765432 ♠♥♦♣rrrr xxpppppp\n00010000 00000000 10001100 00101001",
            Card::ACE_SPADES.bit_string_guided()
        );
    }

    #[test]
    fn frequency_paired() {
        let weighted = Card::TREY_CLUBS.frequency_paired();

        assert!(weighted.is_flagged(Card::FREQUENCY_PAIRED_MASK));
        assert_eq!(0b00000000_00000010_00000000_00000000, weighted.get_rank_flag());
        assert_eq!(0b00000000_00000000_00010000_00000000, weighted.get_suit_flag());
        assert_eq!(Card::TREY_CLUBS.get_rank(), weighted.get_rank());
        assert_eq!(Card::TREY_CLUBS.get_suit(), weighted.get_suit());
    }

    #[test]
    fn frequency_tripped() {
        let weighted = Card::TREY_DIAMONDS.frequency_tripped();

        assert!(weighted.is_flagged(Card::FREQUENCY_TRIPPED_MASK));
        assert_eq!(0b00000000_00000010_00000000_00000000, weighted.get_rank_flag());
        assert_eq!(0b00000000_00000000_00100000_00000000, weighted.get_suit_flag());
        assert_eq!(Card::TREY_DIAMONDS.get_rank(), weighted.get_rank());
        assert_eq!(Card::TREY_DIAMONDS.get_suit(), weighted.get_suit());
    }

    #[test]
    fn frequency_quaded() {
        let weighted = Card::TREY_HEARTS.frequency_quaded();

        assert!(weighted.is_flagged(Card::FREQUENCY_QUADED_MASK));
        assert_eq!(0b00000000_00000010_00000000_00000000, weighted.get_rank_flag());
        assert_eq!(0b00000000_00000000_01000000_00000000, weighted.get_suit_flag());
        assert_eq!(Card::TREY_HEARTS.get_rank(), weighted.get_rank());
        assert_eq!(Card::TREY_HEARTS.get_suit(), weighted.get_suit());
    }

    #[test]
    fn get_rank() {
        let card = Card::ACE_CLUBS;
        assert_eq!(0b00010000_00000000_00000000_00000000, card.get_rank_flag());
        assert_eq!(Rank::ACE, card.get_rank());
        assert_eq!(Rank::ACE.prime(), card.get_rank_prime());
        let card = Card::KING_DIAMONDS;
        assert_eq!(0b00001000_00000000_00000000_00000000, card.get_rank_flag());
        assert_eq!(Rank::KING, card.get_rank());
        assert_eq!(Rank::KING.prime(), card.get_rank_prime());
        let card = Card::QUEEN_SPADES;
        assert_eq!(0b00000100_00000000_00000000_00000000, card.get_rank_flag());
        assert_eq!(Rank::QUEEN, card.get_rank());
        assert_eq!(Rank::QUEEN.prime(), card.get_rank_prime());
        let card = Card::JACK_HEARTS;
        assert_eq!(0b00000010_00000000_00000000_00000000, card.get_rank_flag());
        assert_eq!(Rank::JACK, card.get_rank());
        assert_eq!(Rank::JACK.prime(), card.get_rank_prime());
        let card = Card::TEN_SPADES;
        assert_eq!(0b00000001_00000000_00000000_00000000, card.get_rank_flag());
        assert_eq!(Rank::TEN, card.get_rank());
        assert_eq!(Rank::TEN.prime(), card.get_rank_prime());
        let card = Card::NINE_HEARTS;
        assert_eq!(0b00000000_10000000_00000000_00000000, card.get_rank_flag());
        assert_eq!(Rank::NINE, card.get_rank());
        assert_eq!(Rank::NINE.prime(), card.get_rank_prime());
        let card = Card::EIGHT_DIAMONDS;
        assert_eq!(0b00000000_01000000_00000000_00000000, card.get_rank_flag());
        assert_eq!(Rank::EIGHT, card.get_rank());
        assert_eq!(Rank::EIGHT.prime(), card.get_rank_prime());
        let card = Card::SEVEN_CLUBS;
        assert_eq!(0b00000000_00100000_00000000_00000000, card.get_rank_flag());
        assert_eq!(Rank::SEVEN, card.get_rank());
        assert_eq!(Rank::SEVEN.prime(), card.get_rank_prime());
        let card = Card::SIX_SPADES;
        assert_eq!(0b00000000_00010000_00000000_00000000, card.get_rank_flag());
        assert_eq!(Rank::SIX, card.get_rank());
        assert_eq!(Rank::SIX.prime(), card.get_rank_prime());
        let card = Card::FIVE_HEARTS;
        assert_eq!(0b00000000_00001000_00000000_00000000, card.get_rank_flag());
        assert_eq!(Rank::FIVE, card.get_rank());
        assert_eq!(Rank::FIVE.prime(), card.get_rank_prime());
        let card = Card::FOUR_DIAMONDS;
        assert_eq!(0b00000000_00000100_00000000_00000000, card.get_rank_flag());
        assert_eq!(Rank::FOUR, card.get_rank());
        assert_eq!(Rank::FOUR.prime(), card.get_rank_prime());
        let card = Card::TREY_CLUBS;
        assert_eq!(0b00000000_00000010_00000000_00000000, card.get_rank_flag());
        assert_eq!(Rank::TREY, card.get_rank());
        assert_eq!(Rank::TREY.prime(), card.get_rank_prime());
        let card = Card::DEUCE_SPADES;
        assert_eq!(0b00000000_00000001_00000000_00000000, card.get_rank_flag());
        assert_eq!(Rank::DEUCE, card.get_rank());
        assert_eq!(Rank::DEUCE.prime(), card.get_rank_prime());
    }

    #[test]
    fn get_rank_flag_frequency_weighted() {
        let card = Card::TREY_CLUBS;

        let weighted = card.frequency_paired();

        assert_eq!(0b00000000_00000010_00000000_00000000, weighted.get_rank_flag());
        assert_eq!(card.get_rank(), weighted.get_rank());
    }

    #[test]
    fn suit_shift_down() {
        assert_eq!(Card::ACE_HEARTS, Card::ACE_SPADES.shift_suit_down());
        assert_eq!(Card::SIX_DIAMONDS, Card::SIX_HEARTS.shift_suit_down());
        assert_eq!(Card::QUEEN_CLUBS, Card::QUEEN_DIAMONDS.shift_suit_down());
        assert_eq!(Card::FIVE_SPADES, Card::FIVE_CLUBS.shift_suit_down());
        assert_eq!(Card::BLANK, Card::BLANK.shift_suit_down());
    }

    #[test]
    fn suit_shift_up() {
        assert_eq!(Card::NINE_SPADES, Card::NINE_HEARTS.shift_suit_up());
        assert_eq!(Card::EIGHT_HEARTS, Card::EIGHT_DIAMONDS.shift_suit_up());
        assert_eq!(Card::TEN_DIAMONDS, Card::TEN_CLUBS.shift_suit_up());
        assert_eq!(Card::DEUCE_CLUBS, Card::DEUCE_SPADES.shift_suit_up());
        assert_eq!(Card::BLANK, Card::BLANK.shift_suit_up());
    }

    #[test]
    fn suit_shift_opposite() {
        assert_eq!(Card::NINE_CLUBS, Card::NINE_HEARTS.opposite());
        assert_eq!(Card::EIGHT_SPADES, Card::EIGHT_DIAMONDS.opposite());
        assert_eq!(Card::TEN_HEARTS, Card::TEN_CLUBS.opposite());
        assert_eq!(Card::DEUCE_DIAMONDS, Card::DEUCE_SPADES.opposite());
        assert_eq!(Card::BLANK, Card::BLANK.opposite());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn display() {
        assert_eq!("A♠", alloc::string::ToString::to_string(&Card::ACE_SPADES));
        assert_eq!("A♥", alloc::string::ToString::to_string(&Card::ACE_HEARTS));
        assert_eq!("A♦", alloc::string::ToString::to_string(&Card::ACE_DIAMONDS));
        assert_eq!("A♣", alloc::string::ToString::to_string(&Card::ACE_CLUBS));
        assert_eq!("__", alloc::string::ToString::to_string(&Card::BLANK));
    }

    //region card_consts tests
    /// REFACTORING NOTES
    /// <https://github.com/ContractBridge/pkcore/commit/c3b1a7a425b1ef0394c3719ae34156e685397965>
    /// Original version doesn't test for value, just for internal logic
    /// The goal of testing is to validate how the code is expected to act
    /// and insulate us from breaking things when we change the code later on.
    ///
    /// Fail to validate value: change one u32 for a `CardNumber` and the test should fail.
    /// MORAL: Test for value!
    #[test]
    fn from_u32() {
        let cases: [(Card, Rank, Suit); 52] = [
            (Card::from(CardNumber::AceSpades as u32), Rank::ACE, Suit::SPADES),
            (Card::from(CardNumber::KingSpades as u32), Rank::KING, Suit::SPADES),
            (Card::from(CardNumber::QueenSpades as u32), Rank::QUEEN, Suit::SPADES),
            (Card::from(CardNumber::JackSpades as u32), Rank::JACK, Suit::SPADES),
            (Card::from(CardNumber::TenSpades as u32), Rank::TEN, Suit::SPADES),
            (Card::from(CardNumber::NineSpades as u32), Rank::NINE, Suit::SPADES),
            (Card::from(CardNumber::EightSpades as u32), Rank::EIGHT, Suit::SPADES),
            (Card::from(CardNumber::SevenSpades as u32), Rank::SEVEN, Suit::SPADES),
            (Card::from(CardNumber::SixSpades as u32), Rank::SIX, Suit::SPADES),
            (Card::from(CardNumber::FiveSpades as u32), Rank::FIVE, Suit::SPADES),
            (Card::from(CardNumber::FourSpades as u32), Rank::FOUR, Suit::SPADES),
            (Card::from(CardNumber::TreySpades as u32), Rank::TREY, Suit::SPADES),
            (Card::from(CardNumber::DeuceSpades as u32), Rank::DEUCE, Suit::SPADES),
            (Card::from(CardNumber::AceHearts as u32), Rank::ACE, Suit::HEARTS),
            (Card::from(CardNumber::KingHearts as u32), Rank::KING, Suit::HEARTS),
            (Card::from(CardNumber::QueenHearts as u32), Rank::QUEEN, Suit::HEARTS),
            (Card::from(CardNumber::JackHearts as u32), Rank::JACK, Suit::HEARTS),
            (Card::from(CardNumber::TenHearts as u32), Rank::TEN, Suit::HEARTS),
            (Card::from(CardNumber::NineHearts as u32), Rank::NINE, Suit::HEARTS),
            (Card::from(CardNumber::EightHearts as u32), Rank::EIGHT, Suit::HEARTS),
            (Card::from(CardNumber::SevenHearts as u32), Rank::SEVEN, Suit::HEARTS),
            (Card::from(CardNumber::SixHearts as u32), Rank::SIX, Suit::HEARTS),
            (Card::from(CardNumber::FiveHearts as u32), Rank::FIVE, Suit::HEARTS),
            (Card::from(CardNumber::FourHearts as u32), Rank::FOUR, Suit::HEARTS),
            (Card::from(CardNumber::TreyHearts as u32), Rank::TREY, Suit::HEARTS),
            (Card::from(CardNumber::DeuceHearts as u32), Rank::DEUCE, Suit::HEARTS),
            (Card::from(CardNumber::AceDiamonds as u32), Rank::ACE, Suit::DIAMONDS),
            (Card::from(CardNumber::KingDiamonds as u32), Rank::KING, Suit::DIAMONDS),
            (
                Card::from(CardNumber::QueenDiamonds as u32),
                Rank::QUEEN,
                Suit::DIAMONDS,
            ),
            (Card::from(CardNumber::JackDiamonds as u32), Rank::JACK, Suit::DIAMONDS),
            (Card::from(CardNumber::TenDiamonds as u32), Rank::TEN, Suit::DIAMONDS),
            (Card::from(CardNumber::NineDiamonds as u32), Rank::NINE, Suit::DIAMONDS),
            (
                Card::from(CardNumber::EightDiamonds as u32),
                Rank::EIGHT,
                Suit::DIAMONDS,
            ),
            (
                Card::from(CardNumber::SevenDiamonds as u32),
                Rank::SEVEN,
                Suit::DIAMONDS,
            ),
            (Card::from(CardNumber::SixDiamonds as u32), Rank::SIX, Suit::DIAMONDS),
            (Card::from(CardNumber::FiveDiamonds as u32), Rank::FIVE, Suit::DIAMONDS),
            (Card::from(CardNumber::FourDiamonds as u32), Rank::FOUR, Suit::DIAMONDS),
            (Card::from(CardNumber::TreyDiamonds as u32), Rank::TREY, Suit::DIAMONDS),
            (
                Card::from(CardNumber::DeuceDiamonds as u32),
                Rank::DEUCE,
                Suit::DIAMONDS,
            ),
            (Card::from(CardNumber::AceClubs as u32), Rank::ACE, Suit::CLUBS),
            (Card::from(CardNumber::KingClubs as u32), Rank::KING, Suit::CLUBS),
            (Card::from(CardNumber::QueenClubs as u32), Rank::QUEEN, Suit::CLUBS),
            (Card::from(CardNumber::JackClubs as u32), Rank::JACK, Suit::CLUBS),
            (Card::from(CardNumber::TenClubs as u32), Rank::TEN, Suit::CLUBS),
            (Card::from(CardNumber::NineClubs as u32), Rank::NINE, Suit::CLUBS),
            (Card::from(CardNumber::EightClubs as u32), Rank::EIGHT, Suit::CLUBS),
            (Card::from(CardNumber::SevenClubs as u32), Rank::SEVEN, Suit::CLUBS),
            (Card::from(CardNumber::SixClubs as u32), Rank::SIX, Suit::CLUBS),
            (Card::from(CardNumber::FiveClubs as u32), Rank::FIVE, Suit::CLUBS),
            (Card::from(CardNumber::FourClubs as u32), Rank::FOUR, Suit::CLUBS),
            (Card::from(CardNumber::TreyClubs as u32), Rank::TREY, Suit::CLUBS),
            (Card::from(CardNumber::DeuceClubs as u32), Rank::DEUCE, Suit::CLUBS),
        ];
        for (actual, rank, suit) in cases {
            assert_eq!(actual.get_rank(), rank);
            assert_eq!(actual.get_suit(), suit);
        }
        assert_eq!(Card::default().get_rank(), Rank::BLANK);
        assert_eq!(Card::default().get_suit(), Suit::BLANK);
    }
    //endregion tests

    #[test]
    fn from_str() {
        assert_eq!(Card::ACE_HEARTS, Card::from_str("AH").unwrap());
        assert_eq!(Card::KING_DIAMONDS, Card::from_str("  K♢   ").unwrap());
        assert_eq!(CkcError::InvalidIndex, Card::from_str("  ").unwrap_err());
        assert_eq!(CkcError::InvalidIndex, Card::from_str("QQ").unwrap_err());
    }

    /// By default, cards will sort themselves from lowest, to highest, which means
    /// that A♠ A♣ K♠ will sort to K♠ A♣ A♠
    #[cfg(feature = "alloc")]
    #[test]
    fn sort() {
        let mut v = alloc::vec![Card::ACE_SPADES, Card::ACE_CLUBS, Card::KING_SPADES];

        v.sort();

        assert_eq!(v, alloc::vec![Card::KING_SPADES, Card::ACE_CLUBS, Card::ACE_SPADES]);
    }

    /// Ported verbatim from pkcore's own `clean` test (`pkcore/src/card.rs:626-631`).
    #[test]
    fn clean() {
        assert_eq!(Card::TREY_CLUBS, Card::TREY_CLUBS.frequency_paired().clean());
        assert_eq!(Card::TREY_CLUBS, Card::TREY_CLUBS.frequency_tripped().clean());
        assert_eq!(Card::TREY_CLUBS, Card::TREY_CLUBS.frequency_quaded().clean());
        assert_eq!(Card::TREY_CLUBS, Card::TREY_CLUBS.clean());
    }

    #[test]
    fn frequency_tripped_idempotent() {
        let once = Card::TREY_DIAMONDS.frequency_tripped();
        let twice = once.frequency_tripped();
        assert_eq!(once, twice);
    }

    #[test]
    fn frequency_quaded_idempotent() {
        let once = Card::TREY_HEARTS.frequency_quaded();
        let twice = once.frequency_quaded();
        assert_eq!(once, twice);
    }
}
