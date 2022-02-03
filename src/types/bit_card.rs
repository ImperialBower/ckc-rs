use crate::{CardRank, CardSuit, PokerCard};
use bitvec::field::BitField;
use bitvec::prelude::{BitArray, BitSlice, BitVec, Msb0};

/// `BitCard` is an experiment with using the
/// [Alexander Payne](https://myrrlyn.net/)'s wonderful
/// [types](https://github.com/bitvecto-rs/bitvec) library to represent
/// [Cactus Kev's](https://suffe.cool/poker/evaluator.html) binary representation
/// of a Poker card.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BitCard(BitArray<Msb0, [u8; 4]>);

impl BitCard {
    // Constructors
    #[must_use]
    pub fn new(b: BitArray<Msb0, [u8; 4]>) -> BitCard {
        BitCard(b)
    }

    // Struct methods

    #[must_use]
    pub fn as_bitarray(&self) -> BitArray<Msb0, [u8; 4]> {
        self.0.copy()
    }

    #[must_use]
    pub fn as_bitslice(&self) -> &BitSlice<Msb0, u8> {
        self.0.copy().as_bitslice()
    }

    #[must_use]
    pub fn get_rank(&self) -> CardRank {
        match self.get_rank_bitslice().trailing_zeros() {
            12 => CardRank::ACE,
            11 => CardRank::KING,
            10 => CardRank::QUEEN,
            9 => CardRank::JACK,
            8 => CardRank::TEN,
            7 => CardRank::NINE,
            6 => CardRank::EIGHT,
            5 => CardRank::SEVEN,
            4 => CardRank::SIX,
            3 => CardRank::FIVE,
            2 => CardRank::FOUR,
            1 => CardRank::THREE,
            0 => CardRank::TWO,
            _ => CardRank::Blank,
        }
    }

    #[must_use]
    pub fn get_rank_bitslice(&self) -> &BitSlice<Msb0, u8> {
        &self.0.copy()[..16]
    }

    #[must_use]
    pub fn get_suit(&self) -> CardSuit {
        match self.get_suit_bitslice().load_le::<u8>() {
            8 => CardSuit::SPADES,
            4 => CardSuit::HEARTS,
            2 => CardSuit::DIAMONDS,
            1 => CardSuit::CLUBS,
            _ => CardSuit::Blank,
        }
    }

    /// Returns a `BitSlice` of the `Suit` section of the `CactusKev` `BitArray`.
    #[must_use]
    pub fn get_suit_bitslice(&self) -> &BitSlice<Msb0, u8> {
        &self.0.copy()[16..20]
    }

    #[must_use]
    pub fn get_suit_binary_signature(&self) -> u32 {
        let s = self.get_suit_bitslice().load_be::<u32>();
        s << 12
    }

    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.0.count_zeros() == 32
    }

    #[must_use]
    pub fn and(&self, bc: &BitSlice<Msb0, u8>) -> BitVec<Msb0, u8> {
        self.as_bitslice().to_bitvec() | bc.to_bitvec()
    }

    #[must_use]
    pub fn or(&self, bc: &BitSlice<Msb0, u8>) -> BitVec<Msb0, u8> {
        self.as_bitslice().to_bitvec() | bc.to_bitvec()
    }

    #[must_use]
    pub fn or_rank_bitslice(&self, bc: &BitSlice<Msb0, u8>) -> BitVec<Msb0, u8> {
        self.get_rank_bitslice().to_bitvec() | bc.to_bitvec()
    }

    #[must_use]
    pub fn and_suit_bitslice(&self, bc: &BitSlice<Msb0, u8>) -> BitVec<Msb0, u8> {
        self.get_suit_bitslice().to_bitvec() & bc.to_bitvec()
    }

    #[must_use]
    pub fn or_suit_bitslice(&self, bc: &BitSlice<Msb0, u8>) -> BitVec<Msb0, u8> {
        self.get_suit_bitslice().to_bitvec() | bc.to_bitvec()
    }

    #[must_use]
    pub fn to_bitvec(&self) -> BitVec<Msb0, u8> {
        self.0.copy().to_bitvec()
    }

    #[must_use]
    pub fn to_poker_card(&self) -> PokerCard {
        self.as_bitslice().load_be::<u32>()
    }

    // Private methods

    fn set_rank(&mut self, rank_weight: u32) {
        self.0[20..24].store_be(rank_weight);
    }

    fn set_rank_flag(&mut self, rank_weight: u32) {
        match rank_weight {
            12 => self.0.set(3, true), // Ace
            11 => self.0.set(4, true), // King
            10 => self.0.set(5, true), // Queen
            9 => self.0.set(6, true),  // Jack
            8 => self.0.set(7, true),  // Ten
            7 => self.0.set(8, true),  // Nine
            6 => self.0.set(9, true),  // Eight
            5 => self.0.set(10, true), // Seven
            4 => self.0.set(11, true), // Six
            3 => self.0.set(12, true), // Five
            2 => self.0.set(13, true), // Four
            1 => self.0.set(14, true), // Three
            0 => self.0.set(15, true), // Two
            _ => (),
        }
    }

    fn set_rank_prime(&mut self, rank_prime: u32) {
        self.0[26..32].store_be(rank_prime);
    }

    fn set_suit(&mut self, suit_weight: u32) {
        match suit_weight {
            4 => self.0.set(16, true), // Spades
            3 => self.0.set(17, true), // Hearts
            2 => self.0.set(18, true), // Diamonds
            1 => self.0.set(19, true), // Clubs
            _ => (),
        }
    }
}

impl Default for BitCard {
    fn default() -> BitCard {
        BitCard::new(BitArray::zeroed())
    }
}

impl From<PokerCard> for BitCard {
    fn from(number: PokerCard) -> Self {
        let mut bc: BitCard = BitCard::default();
        if number == 0_u32 {
            return bc;
        }
        bc.0[..32].store_be(number);
        bc
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod bit_card_tests {
    use super::*;
    use crate::types::deck::{Deck, POKER_DECK};
    use crate::CardNumber;
    use rstest::rstest;

    #[test]
    fn len() {
        assert_eq!(BitCard::default().0.len(), 32);
    }

    #[test]
    fn from_poker_card() {
        let actual = BitCard::from(CardNumber::ACE_SPADES);

        assert_eq!(actual.to_poker_card(), CardNumber::ACE_SPADES);
    }

    #[test]
    fn from_to() {
        for i in 0..Deck::len() {
            let card = Deck::get(i);

            let actual = BitCard::from(card);

            assert_eq!(actual.to_poker_card(), card);
        }
    }

    #[test]
    fn get_rank() {
        assert_eq!(
            BitCard::from(CardNumber::ACE_SPADES).get_rank(),
            cardpack::Rank::new(cardpack::ACE)
        );
        assert_eq!(
            BitCard::from(CardNumber::KING_SPADES).get_rank(),
            cardpack::Rank::new(cardpack::KING)
        );
        assert_eq!(
            BitCard::from(CardNumber::QUEEN_SPADES).get_rank(),
            cardpack::Rank::new(cardpack::QUEEN)
        );
        assert_eq!(
            BitCard::from(CardNumber::JACK_SPADES).get_rank(),
            cardpack::Rank::new(cardpack::JACK)
        );
        assert_eq!(
            BitCard::from(CardNumber::TEN_SPADES).get_rank(),
            cardpack::Rank::new(cardpack::TEN)
        );
        assert_eq!(
            BitCard::from(CardNumber::NINE_SPADES).get_rank(),
            cardpack::Rank::new(cardpack::NINE)
        );
        assert_eq!(
            BitCard::from(CardNumber::EIGHT_DIAMONDS).get_rank(),
            cardpack::Rank::new(cardpack::EIGHT)
        );
        assert_eq!(
            BitCard::from(CardNumber::SEVEN_HEARTS).get_rank(),
            cardpack::Rank::new(cardpack::SEVEN)
        );
        assert_eq!(
            BitCard::from(CardNumber::SIX_SPADES).get_rank(),
            cardpack::Rank::new(cardpack::SIX)
        );
        assert_eq!(
            BitCard::from(CardNumber::FIVE_CLUBS).get_rank(),
            cardpack::Rank::new(cardpack::FIVE)
        );
        assert_eq!(
            BitCard::from(CardNumber::FOUR_DIAMONDS).get_rank(),
            cardpack::Rank::new(cardpack::FOUR)
        );
        assert_eq!(
            BitCard::from(CardNumber::TREY_HEARTS).get_rank(),
            cardpack::Rank::new(cardpack::THREE)
        );
        assert_eq!(
            BitCard::from(CardNumber::DEUCE_CLUBS).get_rank(),
            cardpack::Rank::new(cardpack::TWO)
        );
    }

    #[test]
    fn get_rank_bitslice() {
        let card: BitCard = BitCard::from(CardNumber::KING_HEARTS);
        assert_eq!(0b0000100000000000, card.get_rank_bitslice());
    }

    #[test]
    fn get_suit() {
        assert_eq!(
            BitCard::from(CardNumber::TEN_SPADES).get_suit(),
            cardpack::Suit::new(cardpack::SPADES)
        );

        assert_eq!(
            BitCard::from(CardNumber::NINE_HEARTS).get_suit(),
            cardpack::Suit::new(cardpack::HEARTS)
        );

        assert_eq!(
            BitCard::from(CardNumber::FIVE_DIAMONDS).get_suit(),
            cardpack::Suit::new(cardpack::DIAMONDS)
        );

        assert_eq!(
            BitCard::from(CardNumber::NINE_CLUBS).get_suit(),
            cardpack::Suit::new(cardpack::CLUBS)
        )
    }

    #[test]
    fn get_suit_bitslice() {
        let card: BitCard = BitCard::from(CardNumber::SEVEN_SPADES);
        assert_eq!(0b1000, card.get_suit_bitslice());

        let card: BitCard = BitCard::from(CardNumber::QUEEN_HEARTS);
        assert_eq!(0b0100, card.get_suit_bitslice());

        let card: BitCard = BitCard::from(CardNumber::QUEEN_DIAMONDS);
        assert_eq!(0b0010, card.get_suit_bitslice());

        let card: BitCard = BitCard::from(CardNumber::JACK_CLUBS);
        assert_eq!(0b0001, card.get_suit_bitslice());
    }

    #[rstest]
    #[case(CardNumber::DEUCE_CLUBS, 4096)]
    #[case(CardNumber::DEUCE_DIAMONDS, 8192)]
    #[case(CardNumber::DEUCE_HEARTS, 16384)]
    #[case(CardNumber::DEUCE_SPADES, 32768)]
    fn get_suit_binary_signature(#[case] card_number: PokerCard, #[case] expected: u32) {
        let bit_card: BitCard = BitCard::from(card_number);

        assert_eq!(bit_card.get_suit_binary_signature(), expected);
        assert_eq!(
            bit_card.get_suit_binary_signature(),
            PlayingCard::suit_binary_signature(&card)
        );
    }

    #[test]
    fn is_blank() {
        assert!(BitCard::default().is_blank());
    }

    #[test]
    fn is_blank__false() {
        assert!(!BitCard::from(CardNumber::FOUR_HEARTS).is_blank());
    }

    #[test]
    fn or_rank_bitslice() {
        let ace_spades = BitCard::from(CardNumber::ACE_SPADES);
        let king_spades = BitCard::from(CardNumber::KING_SPADES);
        let result = ace_spades.or_rank_bitslice(&king_spades.get_rank_bitslice());

        assert_eq!(result, 0b0001100000000000);
    }

    #[test]
    fn and_suit_bitslice() {
        let king_spade = BitCard::from(CardNumber::KING_SPADES);
        let queen_spades = BitCard::from(CardNumber::QUEEN_SPADES);

        let actual = king_spades.or_suit_bitslice(&queen_spades.get_suit_bitslice());

        assert_eq!(0b1000, actual);
    }

    #[test]
    fn or_suit_bitslice() {
        let king_spade = BitCard::from(CardNumber::KING_SPADES);
        let king_hearts = BitCard::from(CardNumber::KING_HEARTS);
        let king_diamonds = BitCard::from(CardNumber::KING_DIAMONDS);
        let king_clubs = BitCard::from(CardNumber::KING_CLUBS);

        let actual = king_spades.or_suit_bitslice(&king_hearts.get_suit_bitslice());
        assert_eq!(0b1100, actual);

        let actual = king_diamonds.or_suit_bitslice(&actual);
        assert_eq!(0b1110, actual);

        let actual = king_clubs.or_suit_bitslice(&actual);
        assert_eq!(0b1111, actual);
    }

    #[test]
    fn set_rank() {
        let mut bit_card: BitCard = BitCard::default();
        let card = cardpack::Standard52::card_from_index("K♦");

        bit_card.set_rank(card.rank.weight);
        assert_eq!(0b00000000000000000000101100000000, bit_card.to_poker_card());
    }

    #[test]
    fn set_rank_flag() {
        let mut bit_card: BitCard = BitCard::default();
        let card = cardpack::Standard52::card_from_index("K♦");

        bit_card.set_rank_flag(card.rank.weight);
        assert_eq!(0b00001000000000000000000000000000, bit_card.to_poker_card());
    }

    #[test]
    fn set_rank_prime() {
        let mut bit_card: BitCard = BitCard::default();
        let card = cardpack::Standard52::card_from_index("K♦");

        bit_card.set_rank_prime(card.rank.prime);
        assert_eq!(0b00000000000000000000000000100101, bit_card.to_poker_card());
    }

    #[test]
    fn set_suit() {
        let mut bit_card: BitCard = BitCard::default();

        let card = cardpack::Standard52::card_from_index("KS");
        bit_card.set_suit(card.suit.weight);
        assert_eq!(0b00000000000000001000000000000000, bit_card.to_poker_card());

        let card = cardpack::Standard52::card_from_index("KH");
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_suit(card.suit.weight);
        assert_eq!(0b00000000000000000100000000000000, bit_card.to_poker_card());

        let card = cardpack::Standard52::card_from_index("K♦");
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_suit(card.suit.weight);
        assert_eq!(0b00000000000000000010000000000000, bit_card.to_poker_card());

        let card = cardpack::Standard52::card_from_index("KC");
        let mut bit_card: BitCard = BitCard::default();
        bit_card.set_suit(card.suit.weight);
        assert_eq!(
            "00000000 00000000 00010000 00000000",
            bit_card.display(true)
        );
    }
}
