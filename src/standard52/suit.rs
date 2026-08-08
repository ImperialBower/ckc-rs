use crate::CkcError;
use core::fmt;
use core::str::FromStr;

/// Spades to Hearts to Diamonds to Clubs.
pub trait SuitShift {
    #[must_use]
    fn shift_suit_down(&self) -> Self;

    #[must_use]
    fn shift_suit_up(&self) -> Self;

    #[must_use]
    fn opposite(&self) -> Self;
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Suit {
    SPADES = 4,
    HEARTS = 3,
    DIAMONDS = 2,
    CLUBS = 1,
    BLANK = 0,
}

impl Suit {
    /// The four real suits, high to low. Excludes `BLANK` — this is the
    /// `no_std` replacement for the old `all() -> HashSet<Suit>`.
    pub const ALL: [Suit; 4] = [Suit::SPADES, Suit::HEARTS, Suit::DIAMONDS, Suit::CLUBS];

    pub fn iter() -> core::slice::Iter<'static, Suit> {
        Self::ALL.iter()
    }

    #[must_use]
    pub fn binary_signature(&self) -> u32 {
        match self {
            Suit::SPADES => 0x8000,
            Suit::HEARTS => 0x4000,
            Suit::DIAMONDS => 0x2000,
            Suit::CLUBS => 0x1000,
            Suit::BLANK => 0,
        }
    }

    #[must_use]
    pub fn to_char_letter(self) -> char {
        match self {
            Suit::SPADES => 'S',
            Suit::HEARTS => 'H',
            Suit::DIAMONDS => 'D',
            Suit::CLUBS => 'C',
            Suit::BLANK => '_',
        }
    }

    #[must_use]
    pub fn to_char_symbol(self) -> char {
        match self {
            Suit::SPADES => '♠',
            Suit::HEARTS => '♥',
            Suit::DIAMONDS => '♦',
            Suit::CLUBS => '♣',
            Suit::BLANK => '_',
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char_symbol())
    }
}

impl From<char> for Suit {
    fn from(char: char) -> Self {
        match char {
            '♤' | '♠' | 'S' | 's' => Suit::SPADES,
            '♡' | '♥' | 'H' | 'h' => Suit::HEARTS,
            '♢' | '♦' | 'D' | 'd' => Suit::DIAMONDS,
            '♧' | '♣' | 'C' | 'c' => Suit::CLUBS,
            _ => Suit::BLANK,
        }
    }
}

impl FromStr for Suit {
    type Err = CkcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut chars = s.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Ok(Suit::from(c)),
            _ => Err(CkcError::InvalidIndex),
        }
    }
}

impl SuitShift for Suit {
    fn shift_suit_down(&self) -> Self {
        match self {
            Suit::SPADES => Suit::HEARTS,
            Suit::HEARTS => Suit::DIAMONDS,
            Suit::DIAMONDS => Suit::CLUBS,
            Suit::CLUBS => Suit::SPADES,
            Suit::BLANK => Suit::BLANK,
        }
    }

    fn shift_suit_up(&self) -> Self {
        match self {
            Suit::SPADES => Suit::CLUBS,
            Suit::HEARTS => Suit::SPADES,
            Suit::DIAMONDS => Suit::HEARTS,
            Suit::CLUBS => Suit::DIAMONDS,
            Suit::BLANK => Suit::BLANK,
        }
    }

    fn opposite(&self) -> Self {
        self.shift_suit_down().shift_suit_down()
    }
}

#[cfg(test)]
mod suit_tests {
    use super::*;

    #[test]
    fn all_excludes_blank_and_is_complete() {
        assert_eq!(Suit::ALL.len(), 4);
        assert!(!Suit::ALL.contains(&Suit::BLANK));
    }

    #[test]
    fn binary_signatures_are_distinct_single_bits() {
        let mut seen = 0u32;
        for suit in Suit::iter() {
            let sig = suit.binary_signature();
            assert_eq!(sig.count_ones(), 1, "{suit:?} signature must be one bit");
            assert_eq!(seen & sig, 0, "{suit:?} signature collides");
            seen |= sig;
        }
        assert_eq!(seen, 0xF000);
    }

    #[test]
    fn from_char() {
        let cases = [
            ('♠', Suit::SPADES),
            ('♤', Suit::SPADES),
            ('S', Suit::SPADES),
            ('s', Suit::SPADES),
            ('♥', Suit::HEARTS),
            ('♡', Suit::HEARTS),
            ('H', Suit::HEARTS),
            ('h', Suit::HEARTS),
            ('♦', Suit::DIAMONDS),
            ('♢', Suit::DIAMONDS),
            ('D', Suit::DIAMONDS),
            ('d', Suit::DIAMONDS),
            ('♣', Suit::CLUBS),
            ('♧', Suit::CLUBS),
            ('C', Suit::CLUBS),
            ('c', Suit::CLUBS),
            (' ', Suit::BLANK),
            ('F', Suit::BLANK),
        ];
        for (input, expected) in cases {
            assert_eq!(expected, Suit::from(input), "char {input:?}");
        }
    }

    #[test]
    fn from_str_roundtrips_every_suit() {
        let cases = [
            ("♠", Suit::SPADES),
            ("♤", Suit::SPADES),
            ("S", Suit::SPADES),
            ("s", Suit::SPADES),
            ("♥", Suit::HEARTS),
            ("♡", Suit::HEARTS),
            ("H", Suit::HEARTS),
            ("h", Suit::HEARTS),
            ("♦", Suit::DIAMONDS),
            ("♢", Suit::DIAMONDS),
            ("D", Suit::DIAMONDS),
            ("d", Suit::DIAMONDS),
            ("♣", Suit::CLUBS),
            ("♧", Suit::CLUBS),
            ("C", Suit::CLUBS),
            ("c", Suit::CLUBS),
            ("F", Suit::BLANK),
            ("_", Suit::BLANK),
        ];
        for (input, expected) in cases {
            assert_eq!(expected, Suit::from_str(input).unwrap(), "str {input:?}");
        }
    }

    #[test]
    fn from_str_invalid() {
        assert_eq!(CkcError::InvalidIndex, Suit::from_str("").unwrap_err());
        assert_eq!(CkcError::InvalidIndex, Suit::from_str(" ").unwrap_err());
        assert_eq!(CkcError::InvalidIndex, Suit::from_str("AK").unwrap_err());
    }

    #[test]
    fn suit_shift_down() {
        assert_eq!(Suit::HEARTS, Suit::SPADES.shift_suit_down());
        assert_eq!(Suit::DIAMONDS, Suit::HEARTS.shift_suit_down());
        assert_eq!(Suit::CLUBS, Suit::DIAMONDS.shift_suit_down());
        assert_eq!(Suit::SPADES, Suit::CLUBS.shift_suit_down());
        assert_eq!(Suit::BLANK, Suit::BLANK.shift_suit_down());
    }

    #[test]
    fn suit_shift_up() {
        assert_eq!(Suit::SPADES, Suit::HEARTS.shift_suit_up());
        assert_eq!(Suit::HEARTS, Suit::DIAMONDS.shift_suit_up());
        assert_eq!(Suit::DIAMONDS, Suit::CLUBS.shift_suit_up());
        assert_eq!(Suit::CLUBS, Suit::SPADES.shift_suit_up());
        assert_eq!(Suit::BLANK, Suit::BLANK.shift_suit_up());
    }

    #[test]
    fn suit_shift_opposite() {
        assert_eq!(Suit::SPADES, Suit::DIAMONDS.opposite());
        assert_eq!(Suit::HEARTS, Suit::CLUBS.opposite());
        assert_eq!(Suit::DIAMONDS, Suit::SPADES.opposite());
        assert_eq!(Suit::CLUBS, Suit::HEARTS.opposite());
        assert_eq!(Suit::BLANK, Suit::BLANK.opposite());
    }
}
