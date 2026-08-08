use crate::CkcError;
use core::fmt;
use core::str::FromStr;

#[cfg(feature = "alloc")]
use alloc::{format, string::String, vec::Vec};

/// TODO THEME I am an artist, and I paint with code. The pallet I am using to paint is the domain
/// of the area I am coding for, in this case the traditional 52 card French Deck.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    ACE = 14,
    KING = 13,
    QUEEN = 12,
    JACK = 11,
    TEN = 10,
    NINE = 9,
    EIGHT = 8,
    SEVEN = 7,
    SIX = 6,
    FIVE = 5,
    FOUR = 4,
    TREY = 3,
    DEUCE = 2,
    #[default]
    BLANK = 0,
}

impl Rank {
    /// The thirteen real ranks, ace-high to deuce. Excludes `BLANK`.
    pub const ALL: [Rank; 13] = [
        Rank::ACE,
        Rank::KING,
        Rank::QUEEN,
        Rank::JACK,
        Rank::TEN,
        Rank::NINE,
        Rank::EIGHT,
        Rank::SEVEN,
        Rank::SIX,
        Rank::FIVE,
        Rank::FOUR,
        Rank::TREY,
        Rank::DEUCE,
    ];

    pub fn iter() -> core::slice::Iter<'static, Rank> {
        Self::ALL.iter()
    }

    pub const EIGHT_OR_BETTER_LO_BIT_ACE: u8 = 0b0000_0001;
    pub const EIGHT_OR_BETTER_LO_BIT_DEUCE: u8 = 0b0000_0010;
    pub const EIGHT_OR_BETTER_LO_BIT_TREY: u8 = 0b0000_0100;
    pub const EIGHT_OR_BETTER_LO_BIT_FOUR: u8 = 0b0000_1000;
    pub const EIGHT_OR_BETTER_LO_BIT_FIVE: u8 = 0b0001_0000;
    pub const EIGHT_OR_BETTER_LO_BIT_SIX: u8 = 0b0010_0000;
    pub const EIGHT_OR_BETTER_LO_BIT_SEVEN: u8 = 0b0100_0000;
    pub const EIGHT_OR_BETTER_LO_BIT_EIGHT: u8 = 0b1000_0000;

    #[must_use]
    pub fn bits(self) -> u32 {
        1 << (16 + self.number())
    }

    #[must_use]
    pub fn number(self) -> u32 {
        match self {
            Rank::ACE => 12,
            Rank::KING => 11,
            Rank::QUEEN => 10,
            Rank::JACK => 9,
            Rank::TEN => 8,
            Rank::NINE => 7,
            Rank::EIGHT => 6,
            Rank::SEVEN => 5,
            Rank::SIX => 4,
            Rank::FIVE => 3,
            Rank::FOUR => 2,
            Rank::TREY => 1,
            _ => 0,
        }
    }

    #[must_use]
    pub fn prime(self) -> u32 {
        match self {
            Rank::ACE => 41,
            Rank::KING => 37,
            Rank::QUEEN => 31,
            Rank::JACK => 29,
            Rank::TEN => 23,
            Rank::NINE => 19,
            Rank::EIGHT => 17,
            Rank::SEVEN => 13,
            Rank::SIX => 11,
            Rank::FIVE => 7,
            Rank::FOUR => 5,
            Rank::TREY => 3,
            Rank::DEUCE => 2,
            Rank::BLANK => 0,
        }
    }

    #[must_use]
    pub fn shift8(self) -> u32 {
        self.number() << 8
    }

    #[must_use]
    pub fn to_char(self) -> char {
        // TODO NOTE: I wonder if there is a better way to go back and forth from chars?
        match self {
            Rank::ACE => 'A',
            Rank::KING => 'K',
            Rank::QUEEN => 'Q',
            Rank::JACK => 'J',
            Rank::TEN => 'T',
            Rank::NINE => '9',
            Rank::EIGHT => '8',
            Rank::SEVEN => '7',
            Rank::SIX => '6',
            Rank::FIVE => '5',
            Rank::FOUR => '4',
            Rank::TREY => '3',
            Rank::DEUCE => '2',
            Rank::BLANK => '_',
        }
    }

    #[must_use]
    pub fn to_eight_or_better_lo_bit(self) -> u8 {
        match self {
            Rank::ACE => Rank::EIGHT_OR_BETTER_LO_BIT_ACE,
            Rank::DEUCE => Rank::EIGHT_OR_BETTER_LO_BIT_DEUCE,
            Rank::TREY => Rank::EIGHT_OR_BETTER_LO_BIT_TREY,
            Rank::FOUR => Rank::EIGHT_OR_BETTER_LO_BIT_FOUR,
            Rank::FIVE => Rank::EIGHT_OR_BETTER_LO_BIT_FIVE,
            Rank::SIX => Rank::EIGHT_OR_BETTER_LO_BIT_SIX,
            Rank::SEVEN => Rank::EIGHT_OR_BETTER_LO_BIT_SEVEN,
            Rank::EIGHT => Rank::EIGHT_OR_BETTER_LO_BIT_EIGHT,
            _ => 0,
        }
    }

    // region rank bit flags
    pub const RANK_BIT_FLAG_A: u16 = 0b1_0000_0000_0000;
    pub const RANK_BIT_FLAG_K: u16 = 0b0_1000_0000_0000;
    pub const RANK_BIT_FLAG_Q: u16 = 0b0_0100_0000_0000;
    pub const RANK_BIT_FLAG_J: u16 = 0b0_0010_0000_0000;
    pub const RANK_BIT_FLAG_T: u16 = 0b0_0001_0000_0000;
    pub const RANK_BIT_FLAG_9: u16 = 0b0_0000_1000_0000;
    pub const RANK_BIT_FLAG_8: u16 = 0b0_0000_0100_0000;
    pub const RANK_BIT_FLAG_7: u16 = 0b0_0000_0010_0000;
    pub const RANK_BIT_FLAG_6: u16 = 0b0_0000_0001_0000;
    pub const RANK_BIT_FLAG_5: u16 = 0b0_0000_0000_1000;
    pub const RANK_BIT_FLAG_4: u16 = 0b0_0000_0000_0100;
    pub const RANK_BIT_FLAG_3: u16 = 0b0_0000_0000_0010;
    pub const RANK_BIT_FLAG_2: u16 = 0b0_0000_0000_0001;

    #[must_use]
    pub fn from_rank_bit_flag(rank: u16) -> Rank {
        match rank {
            Rank::RANK_BIT_FLAG_A => Rank::ACE,
            Rank::RANK_BIT_FLAG_K => Rank::KING,
            Rank::RANK_BIT_FLAG_Q => Rank::QUEEN,
            Rank::RANK_BIT_FLAG_J => Rank::JACK,
            Rank::RANK_BIT_FLAG_T => Rank::TEN,
            Rank::RANK_BIT_FLAG_9 => Rank::NINE,
            Rank::RANK_BIT_FLAG_8 => Rank::EIGHT,
            Rank::RANK_BIT_FLAG_7 => Rank::SEVEN,
            Rank::RANK_BIT_FLAG_6 => Rank::SIX,
            Rank::RANK_BIT_FLAG_5 => Rank::FIVE,
            Rank::RANK_BIT_FLAG_4 => Rank::FOUR,
            Rank::RANK_BIT_FLAG_3 => Rank::TREY,
            Rank::RANK_BIT_FLAG_2 => Rank::DEUCE,
            _ => Rank::BLANK,
        }
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self == &Rank::BLANK
    }

    #[must_use]
    pub fn rank_bit_flag(&self) -> u16 {
        match self {
            Rank::ACE => Rank::RANK_BIT_FLAG_A,
            Rank::KING => Rank::RANK_BIT_FLAG_K,
            Rank::QUEEN => Rank::RANK_BIT_FLAG_Q,
            Rank::JACK => Rank::RANK_BIT_FLAG_J,
            Rank::TEN => Rank::RANK_BIT_FLAG_T,
            Rank::NINE => Rank::RANK_BIT_FLAG_9,
            Rank::EIGHT => Rank::RANK_BIT_FLAG_8,
            Rank::SEVEN => Rank::RANK_BIT_FLAG_7,
            Rank::SIX => Rank::RANK_BIT_FLAG_6,
            Rank::FIVE => Rank::RANK_BIT_FLAG_5,
            Rank::FOUR => Rank::RANK_BIT_FLAG_4,
            Rank::TREY => Rank::RANK_BIT_FLAG_3,
            Rank::DEUCE => Rank::RANK_BIT_FLAG_2,
            Rank::BLANK => 0,
        }
    }

    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn rank_bit_flags_pretty_format(bits: u16) -> String {
        let bin = format!("{bits:13}");
        bin.chars()
            .rev()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("_")
            .chars()
            .rev()
            .collect()
    }
    // endregion rank bit flags
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl From<char> for Rank {
    fn from(char: char) -> Self {
        match char {
            'A' | 'a' => Rank::ACE,
            'K' | 'k' => Rank::KING,
            'Q' | 'q' => Rank::QUEEN,
            'J' | 'j' => Rank::JACK,
            'T' | 't' | '0' => Rank::TEN,
            '9' => Rank::NINE,
            '8' => Rank::EIGHT,
            '7' => Rank::SEVEN,
            '6' => Rank::SIX,
            '5' => Rank::FIVE,
            '4' => Rank::FOUR,
            '3' => Rank::TREY,
            '2' => Rank::DEUCE,
            _ => Rank::BLANK,
        }
    }
}

impl FromStr for Rank {
    type Err = CkcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut chars = s.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Ok(Rank::from(c)),
            _ => Err(CkcError::InvalidIndex),
        }
    }
}

#[cfg(test)]
mod rank_tests {
    use super::*;

    #[test]
    fn all_excludes_blank_and_is_complete() {
        assert_eq!(Rank::ALL.len(), 13);
        assert!(!Rank::ALL.contains(&Rank::BLANK));
    }

    #[test]
    fn primes_are_the_first_thirteen() {
        let expected = [41u32, 37, 31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2];
        for (rank, want) in Rank::ALL.iter().zip(expected) {
            assert_eq!(rank.prime(), want, "{rank:?} prime");
        }
    }

    #[test]
    fn from_char() {
        let cases = [
            ('A', Rank::ACE),
            ('a', Rank::ACE),
            ('K', Rank::KING),
            ('k', Rank::KING),
            ('Q', Rank::QUEEN),
            ('q', Rank::QUEEN),
            ('J', Rank::JACK),
            ('j', Rank::JACK),
            ('T', Rank::TEN),
            ('t', Rank::TEN),
            ('0', Rank::TEN),
            ('9', Rank::NINE),
            ('8', Rank::EIGHT),
            ('7', Rank::SEVEN),
            ('6', Rank::SIX),
            ('5', Rank::FIVE),
            ('4', Rank::FOUR),
            ('3', Rank::TREY),
            ('2', Rank::DEUCE),
            ('_', Rank::BLANK),
            (' ', Rank::BLANK),
        ];
        for (input, expected) in cases {
            assert_eq!(expected, Rank::from(input), "char {input:?}");
        }
    }

    #[test]
    fn from_str_roundtrips_every_rank() {
        let cases = [
            ("A", Rank::ACE),
            ("a", Rank::ACE),
            ("K", Rank::KING),
            ("k", Rank::KING),
            ("Q", Rank::QUEEN),
            ("q", Rank::QUEEN),
            ("J", Rank::JACK),
            ("j", Rank::JACK),
            ("T", Rank::TEN),
            ("t", Rank::TEN),
            ("0", Rank::TEN),
            ("9", Rank::NINE),
            ("8", Rank::EIGHT),
            ("7", Rank::SEVEN),
            ("6", Rank::SIX),
            ("5", Rank::FIVE),
            ("4", Rank::FOUR),
            ("3", Rank::TREY),
            ("2", Rank::DEUCE),
            ("_", Rank::BLANK),
        ];
        for (input, expected) in cases {
            assert_eq!(expected, Rank::from_str(input).unwrap(), "str {input:?}");
        }
    }

    #[test]
    fn from_str_invalid() {
        assert_eq!(CkcError::InvalidIndex, Rank::from_str("").unwrap_err());
        assert_eq!(CkcError::InvalidIndex, Rank::from_str(" ").unwrap_err());
        assert_eq!(CkcError::InvalidIndex, Rank::from_str("AK").unwrap_err());
    }
}
