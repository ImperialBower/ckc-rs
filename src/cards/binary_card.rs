use crate::cards::five::Five;
use crate::cards::four::Four;
use crate::cards::seven::Seven;
use crate::cards::six::Six;
use crate::cards::three::Three;
use crate::cards::two::Two;
use crate::cards::HandValidator;
use crate::{CKCNumber, CardNumber, PokerCard};

pub type BinaryCard = u64;

#[allow(dead_code)]
#[rustfmt::skip]
pub trait BC64 {
    //region Cards
    const ACE_SPADES:     u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const KING_SPADES:    u64 = 0b0100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const QUEEN_SPADES:   u64 = 0b0010_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const JACK_SPADES:    u64 = 0b0001_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const TEN_SPADES:     u64 = 0b0000_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const NINE_SPADES:    u64 = 0b0000_0100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const EIGHT_SPADES:   u64 = 0b0000_0010_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const SEVEN_SPADES:   u64 = 0b0000_0001_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const SIX_SPADES:     u64 = 0b0000_0000_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const FIVE_SPADES:    u64 = 0b0000_0000_0100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const FOUR_SPADES:    u64 = 0b0000_0000_0010_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const TREY_SPADES:    u64 = 0b0000_0000_0001_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const DEUCE_SPADES:   u64 = 0b0000_0000_0000_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const ACE_HEARTS:     u64 = 0b0000_0000_0000_0100_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const KING_HEARTS:    u64 = 0b0000_0000_0000_0010_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const QUEEN_HEARTS:   u64 = 0b0000_0000_0000_0001_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    const JACK_HEARTS:    u64 = 0b0000_0000_0000_0000_1000_0000_0000_0000_0000_0000_0000_0000_0000;
    const TEN_HEARTS:     u64 = 0b0000_0000_0000_0000_0100_0000_0000_0000_0000_0000_0000_0000_0000;
    const NINE_HEARTS:    u64 = 0b0000_0000_0000_0000_0010_0000_0000_0000_0000_0000_0000_0000_0000;
    const EIGHT_HEARTS:   u64 = 0b0000_0000_0000_0000_0001_0000_0000_0000_0000_0000_0000_0000_0000;
    const SEVEN_HEARTS:   u64 = 0b0000_0000_0000_0000_0000_1000_0000_0000_0000_0000_0000_0000_0000;
    const SIX_HEARTS:     u64 = 0b0000_0000_0000_0000_0000_0100_0000_0000_0000_0000_0000_0000_0000;
    const FIVE_HEARTS:    u64 = 0b0000_0000_0000_0000_0000_0010_0000_0000_0000_0000_0000_0000_0000;
    const FOUR_HEARTS:    u64 = 0b0000_0000_0000_0000_0000_0001_0000_0000_0000_0000_0000_0000_0000;
    const TREY_HEARTS:    u64 = 0b0000_0000_0000_0000_0000_0000_1000_0000_0000_0000_0000_0000_0000;
    const DEUCE_HEARTS:   u64 = 0b0000_0000_0000_0000_0000_0000_0100_0000_0000_0000_0000_0000_0000;
    const ACE_DIAMONDS:   u64 = 0b0000_0000_0000_0000_0000_0000_0010_0000_0000_0000_0000_0000_0000;
    const KING_DIAMONDS:  u64 = 0b0000_0000_0000_0000_0000_0000_0001_0000_0000_0000_0000_0000_0000;
    const QUEEN_DIAMONDS: u64 = 0b0000_0000_0000_0000_0000_0000_0000_1000_0000_0000_0000_0000_0000;
    const JACK_DIAMONDS:  u64 = 0b0000_0000_0000_0000_0000_0000_0000_0100_0000_0000_0000_0000_0000;
    const TEN_DIAMONDS:   u64 = 0b0000_0000_0000_0000_0000_0000_0000_0010_0000_0000_0000_0000_0000;
    const NINE_DIAMONDS:  u64 = 0b0000_0000_0000_0000_0000_0000_0000_0001_0000_0000_0000_0000_0000;
    const EIGHT_DIAMONDS: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_1000_0000_0000_0000_0000;
    const SEVEN_DIAMONDS: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0100_0000_0000_0000_0000;
    const SIX_DIAMONDS:   u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0010_0000_0000_0000_0000;
    const FIVE_DIAMONDS:  u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0001_0000_0000_0000_0000;
    const FOUR_DIAMONDS:  u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000_0000_0000;
    const TREY_DIAMONDS:  u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0100_0000_0000_0000;
    const DEUCE_DIAMONDS: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0010_0000_0000_0000;
    const ACE_CLUBS:      u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000_0000_0000;
    const KING_CLUBS:     u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000_0000;
    const QUEEN_CLUBS:    u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100_0000_0000;
    const JACK_CLUBS:     u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010_0000_0000;
    const TEN_CLUBS:      u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000_0000;
    const NINE_CLUBS:     u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000;
    const EIGHT_CLUBS:    u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100_0000;
    const SEVEN_CLUBS:    u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010_0000;
    const SIX_CLUBS:      u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000;
    const FIVE_CLUBS:     u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000;
    const FOUR_CLUBS:     u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100;
    const TREY_CLUBS:     u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
    const DEUCE_CLUBS:    u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001;
    const BLANK:          u64 = 0;

    const ALL:            u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    /// Used to check for values that wouldn't be a valid for a `BinaryCard`.
    const OVERFLOW:       u64 = 0b1111_1111_1111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

    //endregion

    //region Ranks
    const ACES: u64 = BinaryCard::ACE_SPADES | BinaryCard::ACE_HEARTS | BinaryCard::ACE_DIAMONDS | BinaryCard::ACE_CLUBS;
    const KINGS: u64 = BinaryCard::KING_SPADES | BinaryCard::KING_HEARTS | BinaryCard::KING_DIAMONDS | BinaryCard::KING_CLUBS;
    const QUEENS: u64 = BinaryCard::QUEEN_SPADES | BinaryCard::QUEEN_HEARTS | BinaryCard::QUEEN_DIAMONDS | BinaryCard::QUEEN_CLUBS;
    const JACKS: u64 = BinaryCard::JACK_SPADES | BinaryCard::JACK_HEARTS | BinaryCard::JACK_DIAMONDS | BinaryCard::JACK_CLUBS;
    const TENS: u64 = BinaryCard::TEN_SPADES | BinaryCard::TEN_HEARTS | BinaryCard::TEN_DIAMONDS | BinaryCard::TEN_CLUBS;
    const NINES: u64 = BinaryCard::NINE_SPADES | BinaryCard::NINE_HEARTS | BinaryCard::NINE_DIAMONDS | BinaryCard::NINE_CLUBS;
    const EIGHTS: u64 = BinaryCard::EIGHT_SPADES | BinaryCard::EIGHT_HEARTS | BinaryCard::EIGHT_DIAMONDS | BinaryCard::EIGHT_CLUBS;
    const SEVENS: u64 = BinaryCard::SEVEN_SPADES | BinaryCard::SEVEN_HEARTS | BinaryCard::SEVEN_DIAMONDS | BinaryCard::SEVEN_CLUBS;
    const SIXES: u64 = BinaryCard::SIX_SPADES | BinaryCard::SIX_HEARTS | BinaryCard::SIX_DIAMONDS | BinaryCard::SIX_CLUBS;
    const FIVES: u64 = BinaryCard::FIVE_SPADES | BinaryCard::FIVE_HEARTS | BinaryCard::FIVE_DIAMONDS | BinaryCard::FIVE_CLUBS;
    const FOURS: u64 = BinaryCard::FOUR_SPADES | BinaryCard::FOUR_HEARTS | BinaryCard::FOUR_DIAMONDS | BinaryCard::FOUR_CLUBS;
    const TREYS: u64 = BinaryCard::TREY_SPADES | BinaryCard::TREY_HEARTS | BinaryCard::TREY_DIAMONDS | BinaryCard::TREY_CLUBS;
    const DEUCES: u64 = BinaryCard::DEUCE_SPADES | BinaryCard::DEUCE_HEARTS | BinaryCard::DEUCE_DIAMONDS | BinaryCard::DEUCE_CLUBS;
    //endregion Ranks

    const DECK: [BinaryCard; 52] = [
        BinaryCard::ACE_SPADES,
        BinaryCard::KING_SPADES,
        BinaryCard::QUEEN_SPADES,
        BinaryCard::JACK_SPADES,
        BinaryCard::TEN_SPADES,
        BinaryCard::NINE_SPADES,
        BinaryCard::EIGHT_SPADES,
        BinaryCard::SEVEN_SPADES,
        BinaryCard::SIX_SPADES,
        BinaryCard::FIVE_SPADES,
        BinaryCard::FOUR_SPADES,
        BinaryCard::TREY_SPADES,
        BinaryCard::DEUCE_SPADES,
        BinaryCard::ACE_HEARTS,
        BinaryCard::KING_HEARTS,
        BinaryCard::QUEEN_HEARTS,
        BinaryCard::JACK_HEARTS,
        BinaryCard::TEN_HEARTS,
        BinaryCard::NINE_HEARTS,
        BinaryCard::EIGHT_HEARTS,
        BinaryCard::SEVEN_HEARTS,
        BinaryCard::SIX_HEARTS,
        BinaryCard::FIVE_HEARTS,
        BinaryCard::FOUR_HEARTS,
        BinaryCard::TREY_HEARTS,
        BinaryCard::DEUCE_HEARTS,
        BinaryCard::ACE_DIAMONDS,
        BinaryCard::KING_DIAMONDS,
        BinaryCard::QUEEN_DIAMONDS,
        BinaryCard::JACK_DIAMONDS,
        BinaryCard::TEN_DIAMONDS,
        BinaryCard::NINE_DIAMONDS,
        BinaryCard::EIGHT_DIAMONDS,
        BinaryCard::SEVEN_DIAMONDS,
        BinaryCard::SIX_DIAMONDS,
        BinaryCard::FIVE_DIAMONDS,
        BinaryCard::FOUR_DIAMONDS,
        BinaryCard::TREY_DIAMONDS,
        BinaryCard::DEUCE_DIAMONDS,
        BinaryCard::ACE_CLUBS,
        BinaryCard::KING_CLUBS,
        BinaryCard::QUEEN_CLUBS,
        BinaryCard::JACK_CLUBS,
        BinaryCard::TEN_CLUBS,
        BinaryCard::NINE_CLUBS,
        BinaryCard::EIGHT_CLUBS,
        BinaryCard::SEVEN_CLUBS,
        BinaryCard::SIX_CLUBS,
        BinaryCard::FIVE_CLUBS,
        BinaryCard::FOUR_CLUBS,
        BinaryCard::TREY_CLUBS,
        BinaryCard::DEUCE_CLUBS,
    ];

    #[must_use]
    fn from_ckc(ckc: CKCNumber) -> BinaryCard {
        match ckc {
            CardNumber::ACE_SPADES => BinaryCard::ACE_SPADES,
            CardNumber::KING_SPADES => BinaryCard::KING_SPADES,
            CardNumber::QUEEN_SPADES => BinaryCard::QUEEN_SPADES,
            CardNumber::JACK_SPADES => BinaryCard::JACK_SPADES,
            CardNumber::TEN_SPADES => BinaryCard::TEN_SPADES,
            CardNumber::NINE_SPADES => BinaryCard::NINE_SPADES,
            CardNumber::EIGHT_SPADES => BinaryCard::EIGHT_SPADES,
            CardNumber::SEVEN_SPADES => BinaryCard::SEVEN_SPADES,
            CardNumber::SIX_SPADES => BinaryCard::SIX_SPADES,
            CardNumber::FIVE_SPADES => BinaryCard::FIVE_SPADES,
            CardNumber::FOUR_SPADES => BinaryCard::FOUR_SPADES,
            CardNumber::TREY_SPADES => BinaryCard::TREY_SPADES,
            CardNumber::DEUCE_SPADES => BinaryCard::DEUCE_SPADES,
            CardNumber::ACE_HEARTS => BinaryCard::ACE_HEARTS,
            CardNumber::KING_HEARTS => BinaryCard::KING_HEARTS,
            CardNumber::QUEEN_HEARTS => BinaryCard::QUEEN_HEARTS,
            CardNumber::JACK_HEARTS => BinaryCard::JACK_HEARTS,
            CardNumber::TEN_HEARTS => BinaryCard::TEN_HEARTS,
            CardNumber::NINE_HEARTS => BinaryCard::NINE_HEARTS,
            CardNumber::EIGHT_HEARTS => BinaryCard::EIGHT_HEARTS,
            CardNumber::SEVEN_HEARTS => BinaryCard::SEVEN_HEARTS,
            CardNumber::SIX_HEARTS => BinaryCard::SIX_HEARTS,
            CardNumber::FIVE_HEARTS => BinaryCard::FIVE_HEARTS,
            CardNumber::FOUR_HEARTS => BinaryCard::FOUR_HEARTS,
            CardNumber::TREY_HEARTS => BinaryCard::TREY_HEARTS,
            CardNumber::DEUCE_HEARTS => BinaryCard::DEUCE_HEARTS,
            CardNumber::ACE_DIAMONDS => BinaryCard::ACE_DIAMONDS,
            CardNumber::KING_DIAMONDS => BinaryCard::KING_DIAMONDS,
            CardNumber::QUEEN_DIAMONDS => BinaryCard::QUEEN_DIAMONDS,
            CardNumber::JACK_DIAMONDS => BinaryCard::JACK_DIAMONDS,
            CardNumber::TEN_DIAMONDS => BinaryCard::TEN_DIAMONDS,
            CardNumber::NINE_DIAMONDS => BinaryCard::NINE_DIAMONDS,
            CardNumber::EIGHT_DIAMONDS => BinaryCard::EIGHT_DIAMONDS,
            CardNumber::SEVEN_DIAMONDS => BinaryCard::SEVEN_DIAMONDS,
            CardNumber::SIX_DIAMONDS => BinaryCard::SIX_DIAMONDS,
            CardNumber::FIVE_DIAMONDS => BinaryCard::FIVE_DIAMONDS,
            CardNumber::FOUR_DIAMONDS => BinaryCard::FOUR_DIAMONDS,
            CardNumber::TREY_DIAMONDS => BinaryCard::TREY_DIAMONDS,
            CardNumber::DEUCE_DIAMONDS => BinaryCard::DEUCE_DIAMONDS,
            CardNumber::ACE_CLUBS => BinaryCard::ACE_CLUBS,
            CardNumber::KING_CLUBS => BinaryCard::KING_CLUBS,
            CardNumber::QUEEN_CLUBS => BinaryCard::QUEEN_CLUBS,
            CardNumber::JACK_CLUBS => BinaryCard::JACK_CLUBS,
            CardNumber::TEN_CLUBS => BinaryCard::TEN_CLUBS,
            CardNumber::NINE_CLUBS => BinaryCard::NINE_CLUBS,
            CardNumber::EIGHT_CLUBS => BinaryCard::EIGHT_CLUBS,
            CardNumber::SEVEN_CLUBS => BinaryCard::SEVEN_CLUBS,
            CardNumber::SIX_CLUBS => BinaryCard::SIX_CLUBS,
            CardNumber::FIVE_CLUBS => BinaryCard::FIVE_CLUBS,
            CardNumber::FOUR_CLUBS => BinaryCard::FOUR_CLUBS,
            CardNumber::TREY_CLUBS => BinaryCard::TREY_CLUBS,
            CardNumber::DEUCE_CLUBS => BinaryCard::DEUCE_CLUBS,
            _ => BinaryCard::BLANK,
        }
    }

    #[must_use]
    fn from_two(two: Two) -> BinaryCard {
        BinaryCard::from_ckc(two.first()) | BinaryCard::from_ckc(two.second())
    }

    #[must_use]
    fn from_three(three: Three) -> BinaryCard {
        BinaryCard::from_ckc(three.first())
            | BinaryCard::from_ckc(three.second())
            | BinaryCard::from_ckc(three.third())
    }

    #[must_use]
    fn from_four(four: Four) -> BinaryCard {
        BinaryCard::from_ckc(four.first())
            | BinaryCard::from_ckc(four.second())
            | BinaryCard::from_ckc(four.third())
            | BinaryCard::from_ckc(four.forth())
    }

    #[must_use]
    fn from_five(five: Five) -> BinaryCard {
        BinaryCard::from_ckc(five.first())
            | BinaryCard::from_ckc(five.second())
            | BinaryCard::from_ckc(five.third())
            | BinaryCard::from_ckc(five.forth())
            | BinaryCard::from_ckc(five.fifth())
    }

    #[must_use]
    fn from_six(six: Six) -> BinaryCard {
        BinaryCard::from_ckc(six.first())
            | BinaryCard::from_ckc(six.second())
            | BinaryCard::from_ckc(six.third())
            | BinaryCard::from_ckc(six.forth())
            | BinaryCard::from_ckc(six.fifth())
            | BinaryCard::from_ckc(six.sixth())
    }

    #[must_use]
    fn from_seven(seven: Seven) -> BinaryCard {
        BinaryCard::from_ckc(seven.first())
            | BinaryCard::from_ckc(seven.second())
            | BinaryCard::from_ckc(seven.third())
            | BinaryCard::from_ckc(seven.forth())
            | BinaryCard::from_ckc(seven.fifth())
            | BinaryCard::from_ckc(seven.sixth())
            | BinaryCard::from_ckc(seven.seventh())
    }

    #[must_use]
    fn fold_in(&self, bc: BinaryCard) -> BinaryCard {
        self.as_u64() | bc
    }

    #[must_use]
    fn from_index(index: &str) -> BinaryCard {
        let mut bc = BinaryCard::BLANK;

        for s in index.split_whitespace() {
            bc = bc.fold_in(BinaryCard::from_ckc(CKCNumber::from_index(s)));
        }

        bc
    }

    #[must_use]
    fn has(&self, card: u64) -> bool {
        self.as_u64() & card == card
    }

    #[must_use]
    fn is_single_card(&self) -> bool {
        self.number_of_cards() == 1
    }

    #[must_use]
    fn is_valid(&self) -> bool {
        (self.as_u64() != BinaryCard::BLANK) && ((self.as_u64() & BinaryCard::OVERFLOW).number_of_cards()) < 1
    }

    #[must_use]
    fn number_of_cards(&self) -> u32 {
        self.as_u64().count_ones()
    }

    fn peel(&mut self) -> BinaryCard;

    fn as_u64(&self) -> u64;
}

impl BC64 for BinaryCard {
    fn peel(&mut self) -> BinaryCard {
        for bc in BinaryCard::DECK {
            if *self & bc == bc {
                *self ^= bc;
                return bc;
            }
        }
        BinaryCard::BLANK
    }

    fn as_u64(&self) -> u64 {
        *self
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod alt__bit_card {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("A♠", BinaryCard::ACE_SPADES)]
    #[case("ks", BinaryCard::KING_SPADES)]
    #[case("QS", BinaryCard::QUEEN_SPADES)]
    #[case("J♠", BinaryCard::JACK_SPADES)]
    #[case("TS", BinaryCard::TEN_SPADES)]
    #[case("9s", BinaryCard::NINE_SPADES)]
    #[case("8♠", BinaryCard::EIGHT_SPADES)]
    #[case("7S", BinaryCard::SEVEN_SPADES)]
    #[case("6♠", BinaryCard::SIX_SPADES)]
    #[case("5S", BinaryCard::FIVE_SPADES)]
    #[case("4♠", BinaryCard::FOUR_SPADES)]
    #[case("3s", BinaryCard::TREY_SPADES)]
    #[case("2S", BinaryCard::DEUCE_SPADES)]
    #[case("A♥", BinaryCard::ACE_HEARTS)]
    #[case("k♥", BinaryCard::KING_HEARTS)]
    #[case("QH", BinaryCard::QUEEN_HEARTS)]
    #[case("jh", BinaryCard::JACK_HEARTS)]
    #[case("T♥", BinaryCard::TEN_HEARTS)]
    #[case("9♥", BinaryCard::NINE_HEARTS)]
    #[case("8h", BinaryCard::EIGHT_HEARTS)]
    #[case("7H", BinaryCard::SEVEN_HEARTS)]
    #[case("6h", BinaryCard::SIX_HEARTS)]
    #[case("5H", BinaryCard::FIVE_HEARTS)]
    #[case("4♥", BinaryCard::FOUR_HEARTS)]
    #[case("3♥", BinaryCard::TREY_HEARTS)]
    #[case("2h", BinaryCard::DEUCE_HEARTS)]
    #[case("A♦", BinaryCard::ACE_DIAMONDS)]
    #[case("k♦", BinaryCard::KING_DIAMONDS)]
    #[case("Q♦", BinaryCard::QUEEN_DIAMONDS)]
    #[case("Jd", BinaryCard::JACK_DIAMONDS)]
    #[case("tD", BinaryCard::TEN_DIAMONDS)]
    #[case("9♦", BinaryCard::NINE_DIAMONDS)]
    #[case("8D", BinaryCard::EIGHT_DIAMONDS)]
    #[case("7♦", BinaryCard::SEVEN_DIAMONDS)]
    #[case("6D", BinaryCard::SIX_DIAMONDS)]
    #[case("5D", BinaryCard::FIVE_DIAMONDS)]
    #[case("4♦", BinaryCard::FOUR_DIAMONDS)]
    #[case("3♦", BinaryCard::TREY_DIAMONDS)]
    #[case("2d", BinaryCard::DEUCE_DIAMONDS)]
    #[case("a♣", BinaryCard::ACE_CLUBS)]
    #[case("k♣", BinaryCard::KING_CLUBS)]
    #[case("QC", BinaryCard::QUEEN_CLUBS)]
    #[case("jc", BinaryCard::JACK_CLUBS)]
    #[case("tC", BinaryCard::TEN_CLUBS)]
    #[case("9♣", BinaryCard::NINE_CLUBS)]
    #[case("8♣", BinaryCard::EIGHT_CLUBS)]
    #[case("7c", BinaryCard::SEVEN_CLUBS)]
    #[case("6♣", BinaryCard::SIX_CLUBS)]
    #[case("5C", BinaryCard::FIVE_CLUBS)]
    #[case("4c", BinaryCard::FOUR_CLUBS)]
    #[case("3C", BinaryCard::TREY_CLUBS)]
    #[case("2C", BinaryCard::DEUCE_CLUBS)]
    #[case("AS AH AD AC", BinaryCard::ACES)]
    #[case("   KC  KS      KH  KD ", BinaryCard::KINGS)]
    #[case("XX", BinaryCard::BLANK)]
    fn from_index(#[case] index: &str, #[case] expected: BinaryCard) {
        assert_eq!(BinaryCard::from_index(index), expected);
    }

    #[test]
    fn from_two() {
        let bc = BinaryCard::from_two(Two::new(CardNumber::JACK_SPADES, CardNumber::TEN_SPADES));

        assert!(bc.is_valid());
        assert!(!bc.is_single_card());
        assert_eq!(2, bc.number_of_cards());
        assert!(bc.has(BinaryCard::JACK_SPADES));
        assert!(bc.has(BinaryCard::TEN_SPADES));
    }

    #[test]
    fn from_three() {
        let three = Three::try_from("KC QD A♠").unwrap();
        let bc = BinaryCard::from_three(three);

        assert!(bc.is_valid());
        assert!(!bc.is_single_card());
        assert_eq!(3, bc.number_of_cards());
        assert!(bc.has(BinaryCard::ACE_SPADES));
        assert!(bc.has(BinaryCard::KING_CLUBS));
        assert!(bc.has(BinaryCard::QUEEN_DIAMONDS));
    }

    #[test]
    fn from_four() {
        let four = Four::try_from("KC QD A♠ JH").unwrap();
        let bc = BinaryCard::from_four(four);

        assert!(bc.is_valid());
        assert!(!bc.is_single_card());
        assert_eq!(4, bc.number_of_cards());
        assert!(bc.has(BinaryCard::ACE_SPADES));
        assert!(bc.has(BinaryCard::KING_CLUBS));
        assert!(bc.has(BinaryCard::QUEEN_DIAMONDS));
        assert!(bc.has(BinaryCard::JACK_HEARTS));
    }

    #[test]
    fn from_five() {
        let five = Five::try_from("TD KC QD A♠ JH").unwrap();
        let bc = BinaryCard::from_five(five);

        assert!(bc.is_valid());
        assert!(!bc.is_single_card());
        assert_eq!(5, bc.number_of_cards());
        assert!(bc.has(BinaryCard::ACE_SPADES));
        assert!(bc.has(BinaryCard::KING_CLUBS));
        assert!(bc.has(BinaryCard::QUEEN_DIAMONDS));
        assert!(bc.has(BinaryCard::JACK_HEARTS));
        assert!(bc.has(BinaryCard::TEN_DIAMONDS));
    }

    #[test]
    fn from_six() {
        let six = Six::try_from("TS TD KC QD A♠ JH").unwrap();
        let bc = BinaryCard::from_six(six);

        assert!(bc.is_valid());
        assert!(!bc.is_single_card());
        assert_eq!(6, bc.number_of_cards());
        assert!(bc.has(BinaryCard::ACE_SPADES));
        assert!(bc.has(BinaryCard::KING_CLUBS));
        assert!(bc.has(BinaryCard::QUEEN_DIAMONDS));
        assert!(bc.has(BinaryCard::JACK_HEARTS));
        assert!(bc.has(BinaryCard::TEN_DIAMONDS));
        assert!(bc.has(BinaryCard::TEN_SPADES));
    }

    #[test]
    fn from_seven() {
        let seven = Seven::try_from("2D TS TD KC QD A♠ JH").unwrap();
        let bc = BinaryCard::from_seven(seven);

        assert!(bc.is_valid());
        assert!(!bc.is_single_card());
        assert_eq!(7, bc.number_of_cards());
        assert!(bc.has(BinaryCard::ACE_SPADES));
        assert!(bc.has(BinaryCard::KING_CLUBS));
        assert!(bc.has(BinaryCard::QUEEN_DIAMONDS));
        assert!(bc.has(BinaryCard::JACK_HEARTS));
        assert!(bc.has(BinaryCard::TEN_DIAMONDS));
        assert!(bc.has(BinaryCard::TEN_SPADES));
        assert!(bc.has(BinaryCard::DEUCE_DIAMONDS));
    }

    #[test]
    fn fold_in() {
        let aces = BinaryCard::ACE_SPADES.fold_in(BinaryCard::ACE_DIAMONDS);
        assert!(aces.has(BinaryCard::ACE_SPADES));
        assert!(aces.has(BinaryCard::ACE_DIAMONDS));
        assert_eq!(2, aces.number_of_cards());
    }

    #[test]
    fn has() {
        assert!(BinaryCard::ACES.has(BinaryCard::ACE_SPADES));
        assert!(BinaryCard::ACES.has(BinaryCard::ACE_DIAMONDS));
        assert!(!BinaryCard::ACES.has(BinaryCard::KING_DIAMONDS));
    }

    #[test]
    fn is_single_card() {
        assert!(BinaryCard::ACE_SPADES.is_single_card());
        assert!(!BinaryCard::ACES.is_single_card());
    }

    #[test]
    fn is_valid() {
        assert!(BinaryCard::ALL.is_valid());
        assert!(BinaryCard::ACES.is_valid());
        assert!(!BinaryCard::BLANK.is_valid());
        assert!(!BinaryCard::OVERFLOW.is_valid());
    }

    #[test]
    fn number_of_cards() {
        assert_eq!(4, BinaryCard::ACES.number_of_cards());
    }

    #[test]
    fn peel() {
        let mut cards = BinaryCard::ACES;
        assert!(cards.has(BinaryCard::ACE_SPADES));

        let ace_spades = cards.peel();

        assert!(!cards.has(BinaryCard::ACE_SPADES));
        assert!(cards.has(BinaryCard::ACE_HEARTS));
        assert!(cards.has(BinaryCard::ACE_DIAMONDS));
        assert!(cards.has(BinaryCard::ACE_CLUBS));
        assert_eq!(BinaryCard::ACE_SPADES, ace_spades);
        assert_eq!(3, cards.number_of_cards());

        let ace_hearts = cards.peel();
        assert!(!cards.has(BinaryCard::ACE_SPADES));
        assert!(!cards.has(BinaryCard::ACE_HEARTS));
        assert!(cards.has(BinaryCard::ACE_DIAMONDS));
        assert!(cards.has(BinaryCard::ACE_CLUBS));
        assert_eq!(BinaryCard::ACE_HEARTS, ace_hearts);
        assert_eq!(2, cards.number_of_cards());

        let ace_diamonds = cards.peel();
        assert!(!cards.has(BinaryCard::ACE_SPADES));
        assert!(!cards.has(BinaryCard::ACE_HEARTS));
        assert!(!cards.has(BinaryCard::ACE_DIAMONDS));
        assert!(cards.has(BinaryCard::ACE_CLUBS));
        assert_eq!(BinaryCard::ACE_DIAMONDS, ace_diamonds);
        assert_eq!(1, cards.number_of_cards());

        let ace_clubs = cards.peel();
        assert!(!cards.has(BinaryCard::ACE_SPADES));
        assert!(!cards.has(BinaryCard::ACE_HEARTS));
        assert!(!cards.has(BinaryCard::ACE_DIAMONDS));
        assert!(!cards.has(BinaryCard::ACE_CLUBS));
        assert_eq!(BinaryCard::ACE_CLUBS, ace_clubs);
        assert_eq!(0, cards.number_of_cards());
        assert_eq!(0, cards);
    }

    //region Cards
    #[test]
    fn aces() {
        assert_eq!(BinaryCard::ACE_SPADES, BinaryCard::ACE_SPADES & BinaryCard::ACES);
        assert_eq!(BinaryCard::ACE_HEARTS, BinaryCard::ACE_HEARTS & BinaryCard::ACES);
        assert_eq!(BinaryCard::ACE_DIAMONDS, BinaryCard::ACE_DIAMONDS & BinaryCard::ACES);
        assert_eq!(BinaryCard::ACE_CLUBS, BinaryCard::ACE_CLUBS & BinaryCard::ACES);
        assert_eq!(BinaryCard::BLANK, BinaryCard::KING_DIAMONDS & BinaryCard::ACES);
    }

    #[test]
    fn kings() {
        assert_eq!(BinaryCard::KING_SPADES, BinaryCard::KING_SPADES & BinaryCard::KINGS);
        assert_eq!(BinaryCard::KING_HEARTS, BinaryCard::KING_HEARTS & BinaryCard::KINGS);
        assert_eq!(BinaryCard::KING_DIAMONDS, BinaryCard::KING_DIAMONDS & BinaryCard::KINGS);
        assert_eq!(BinaryCard::KING_CLUBS, BinaryCard::KING_CLUBS & BinaryCard::KINGS);
        assert_eq!(BinaryCard::BLANK, BinaryCard::ACE_DIAMONDS & BinaryCard::KINGS);
    }

    #[test]
    fn queens() {
        assert_eq!(BinaryCard::QUEEN_SPADES, BinaryCard::QUEEN_SPADES & BinaryCard::QUEENS);
        assert_eq!(BinaryCard::QUEEN_HEARTS, BinaryCard::QUEEN_HEARTS & BinaryCard::QUEENS);
        assert_eq!(
            BinaryCard::QUEEN_DIAMONDS,
            BinaryCard::QUEEN_DIAMONDS & BinaryCard::QUEENS
        );
        assert_eq!(BinaryCard::QUEEN_CLUBS, BinaryCard::QUEEN_CLUBS & BinaryCard::QUEENS);
        assert_eq!(BinaryCard::BLANK, BinaryCard::ACE_DIAMONDS & BinaryCard::QUEENS);
    }

    #[test]
    fn jacks() {
        assert_eq!(BinaryCard::JACK_SPADES, BinaryCard::JACK_SPADES & BinaryCard::JACKS);
        assert_eq!(BinaryCard::JACK_HEARTS, BinaryCard::JACK_HEARTS & BinaryCard::JACKS);
        assert_eq!(BinaryCard::JACK_DIAMONDS, BinaryCard::JACK_DIAMONDS & BinaryCard::JACKS);
        assert_eq!(BinaryCard::JACK_CLUBS, BinaryCard::JACK_CLUBS & BinaryCard::JACKS);
        assert_eq!(BinaryCard::BLANK, BinaryCard::ACE_DIAMONDS & BinaryCard::JACKS);
    }

    #[test]
    fn tens() {
        assert_eq!(BinaryCard::TEN_SPADES, BinaryCard::TEN_SPADES & BinaryCard::TENS);
        assert_eq!(BinaryCard::TEN_HEARTS, BinaryCard::TEN_HEARTS & BinaryCard::TENS);
        assert_eq!(BinaryCard::TEN_DIAMONDS, BinaryCard::TEN_DIAMONDS & BinaryCard::TENS);
        assert_eq!(BinaryCard::TEN_CLUBS, BinaryCard::TEN_CLUBS & BinaryCard::TENS);
        assert_eq!(BinaryCard::BLANK, BinaryCard::ACE_DIAMONDS & BinaryCard::TENS);
    }

    #[test]
    fn nines() {
        assert_eq!(BinaryCard::NINE_SPADES, BinaryCard::NINE_SPADES & BinaryCard::NINES);
        assert_eq!(BinaryCard::NINE_HEARTS, BinaryCard::NINE_HEARTS & BinaryCard::NINES);
        assert_eq!(BinaryCard::NINE_DIAMONDS, BinaryCard::NINE_DIAMONDS & BinaryCard::NINES);
        assert_eq!(BinaryCard::NINE_CLUBS, BinaryCard::NINE_CLUBS & BinaryCard::NINES);
        assert_eq!(BinaryCard::BLANK, BinaryCard::ACE_DIAMONDS & BinaryCard::NINES);
    }

    #[test]
    fn eights() {
        assert_eq!(BinaryCard::EIGHT_SPADES, BinaryCard::EIGHT_SPADES & BinaryCard::EIGHTS);
        assert_eq!(BinaryCard::EIGHT_HEARTS, BinaryCard::EIGHT_HEARTS & BinaryCard::EIGHTS);
        assert_eq!(
            BinaryCard::EIGHT_DIAMONDS,
            BinaryCard::EIGHT_DIAMONDS & BinaryCard::EIGHTS
        );
        assert_eq!(BinaryCard::EIGHT_CLUBS, BinaryCard::EIGHT_CLUBS & BinaryCard::EIGHTS);
        assert_eq!(BinaryCard::BLANK, BinaryCard::ACE_DIAMONDS & BinaryCard::EIGHTS);
    }

    #[test]
    fn sevens() {
        assert_eq!(BinaryCard::SEVEN_SPADES, BinaryCard::SEVEN_SPADES & BinaryCard::SEVENS);
        assert_eq!(BinaryCard::SEVEN_HEARTS, BinaryCard::SEVEN_HEARTS & BinaryCard::SEVENS);
        assert_eq!(
            BinaryCard::SEVEN_DIAMONDS,
            BinaryCard::SEVEN_DIAMONDS & BinaryCard::SEVENS
        );
        assert_eq!(BinaryCard::SEVEN_CLUBS, BinaryCard::SEVEN_CLUBS & BinaryCard::SEVENS);
        assert_eq!(BinaryCard::BLANK, BinaryCard::ACE_DIAMONDS & BinaryCard::SEVENS);
    }

    #[test]
    fn sixes() {
        assert_eq!(BinaryCard::SIX_SPADES, BinaryCard::SIX_SPADES & BinaryCard::SIXES);
        assert_eq!(BinaryCard::SIX_HEARTS, BinaryCard::SIX_HEARTS & BinaryCard::SIXES);
        assert_eq!(BinaryCard::SIX_DIAMONDS, BinaryCard::SIX_DIAMONDS & BinaryCard::SIXES);
        assert_eq!(BinaryCard::SIX_CLUBS, BinaryCard::SIX_CLUBS & BinaryCard::SIXES);
        assert_eq!(BinaryCard::BLANK, BinaryCard::ACE_DIAMONDS & BinaryCard::SIXES);
    }

    #[test]
    fn fives() {
        assert_eq!(BinaryCard::FIVE_SPADES, BinaryCard::FIVE_SPADES & BinaryCard::FIVES);
        assert_eq!(BinaryCard::FIVE_HEARTS, BinaryCard::FIVE_HEARTS & BinaryCard::FIVES);
        assert_eq!(BinaryCard::FIVE_DIAMONDS, BinaryCard::FIVE_DIAMONDS & BinaryCard::FIVES);
        assert_eq!(BinaryCard::FIVE_CLUBS, BinaryCard::FIVE_CLUBS & BinaryCard::FIVES);
        assert_eq!(BinaryCard::BLANK, BinaryCard::ACE_DIAMONDS & BinaryCard::FIVES);
    }

    #[test]
    fn fours() {
        assert_eq!(BinaryCard::FOUR_SPADES, BinaryCard::FOUR_SPADES & BinaryCard::FOURS);
        assert_eq!(BinaryCard::FOUR_HEARTS, BinaryCard::FOUR_HEARTS & BinaryCard::FOURS);
        assert_eq!(BinaryCard::FOUR_DIAMONDS, BinaryCard::FOUR_DIAMONDS & BinaryCard::FOURS);
        assert_eq!(BinaryCard::FOUR_CLUBS, BinaryCard::FOUR_CLUBS & BinaryCard::FOURS);
        assert_eq!(BinaryCard::BLANK, BinaryCard::ACE_DIAMONDS & BinaryCard::FOURS);
    }

    #[test]
    fn treys() {
        assert_eq!(BinaryCard::TREY_SPADES, BinaryCard::TREY_SPADES & BinaryCard::TREYS);
        assert_eq!(BinaryCard::TREY_HEARTS, BinaryCard::TREY_HEARTS & BinaryCard::TREYS);
        assert_eq!(BinaryCard::TREY_DIAMONDS, BinaryCard::TREY_DIAMONDS & BinaryCard::TREYS);
        assert_eq!(BinaryCard::TREY_CLUBS, BinaryCard::TREY_CLUBS & BinaryCard::TREYS);
        assert_eq!(BinaryCard::BLANK, BinaryCard::ACE_DIAMONDS & BinaryCard::TREYS);
    }

    #[test]
    fn deuces() {
        assert_eq!(BinaryCard::DEUCE_SPADES, BinaryCard::DEUCE_SPADES & BinaryCard::DEUCES);
        assert_eq!(BinaryCard::DEUCE_HEARTS, BinaryCard::DEUCE_HEARTS & BinaryCard::DEUCES);
        assert_eq!(
            BinaryCard::DEUCE_DIAMONDS,
            BinaryCard::DEUCE_DIAMONDS & BinaryCard::DEUCES
        );
        assert_eq!(BinaryCard::DEUCE_CLUBS, BinaryCard::DEUCE_CLUBS & BinaryCard::DEUCES);
        assert_eq!(BinaryCard::BLANK, BinaryCard::ACE_DIAMONDS & BinaryCard::DEUCES);
    }
    //endregion Cards
}
