use crate::{CardNumber, CardRank, CardSuit, CKCNumber};

// #[must_use]
// fn generate(rank: CardRank, suit: CardSuit) -> CardNumber {}
#[must_use]
pub fn get_rank_and_suit(index: &str) -> (CardRank, CardSuit) {
    let mut chars = index.chars();
    let rank: CardRank;
    let suit: CardSuit;
    match chars.next() {
        None => {
            return (CardRank::BLANK, CardSuit::BLANK)
        }
        Some(r) => {
            rank = CardRank::from_char(r);
        }
    }
    match chars.next() {
        None => {
            return (CardRank::BLANK, CardSuit::BLANK)
        }
        Some(s) => {
            suit = CardSuit::from_char(s);
        }
    }
    (rank, suit)
}

pub fn from_index(_index: &str) -> CKCNumber {
    let (rank, suit) = get_rank_and_suit(index);
    let suit_sig: u32 = suit.binary_signature();
    CardNumber::BLANK
}

#[cfg(test)]
mod parse_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("A♠", CardRank::ACE, CardSuit::SPADES)]
    #[case("a♠", CardRank::ACE, CardSuit::SPADES)]
    #[case("AS", CardRank::ACE, CardSuit::SPADES)]
    #[case("As", CardRank::ACE, CardSuit::SPADES)]
    fn get_rank_and_suits(#[case] index: &str, #[case] rank: CardRank, #[case] suit: CardSuit) {
        let (actual_rank, actual_suit) = get_rank_and_suit(index);
        assert_eq!(rank, actual_rank);
        assert_eq!(suit, actual_suit);
    }

    #[test]
    fn test_from_index() {
        assert_eq!(from_index(""), CardNumber::ACE_SPADES);
    }
}