pub type BinaryCard = u64;

#[allow(dead_code)]
#[rustfmt::skip]
pub trait BC64 {
    //region cards

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

    //endregion

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

    #[must_use]
    fn is_single_card(&self) -> bool {
        self.as_u84().count_ones() == 1
    }

    #[must_use]
    fn number_of_cards(&self) -> u32 {
        self.as_u84().count_ones()
    }

    fn as_u84(&self) -> u64;
}

impl BC64 for BinaryCard {
    fn as_u84(&self) -> u64 {
        *self
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod alt__bit_card {
    use super::*;

    #[test]
    fn is_single_card() {
        assert!(BinaryCard::ACE_SPADES.is_single_card());
        assert!(!BinaryCard::ACES.is_single_card());
    }

    #[test]
    fn number_of_cards() {
        assert_eq!(4, BinaryCard::ACES.number_of_cards());
    }

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
}
