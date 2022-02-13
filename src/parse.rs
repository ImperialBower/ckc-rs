use crate::{CKCNumber, CardRank, CardSuit, PokerCard};

#[must_use]
pub fn five_from_index(index: &str) -> Option<[CKCNumber; 5]> {
    let mut esses = index.split_whitespace();

    let first = CKCNumber::from_index(esses.next()?);
    let second = CKCNumber::from_index(esses.next()?);
    let third = CKCNumber::from_index(esses.next()?);
    let forth = CKCNumber::from_index(esses.next()?);
    let fifth = CKCNumber::from_index(esses.next()?);
    let hand: [CKCNumber; 5] = [first, second, third, forth, fifth];
    Some(hand)
}

#[must_use]
pub fn get_rank_and_suit(index: &str) -> (CardRank, CardSuit) {
    let mut chars = index.chars();
    let rank: CardRank = match chars.next() {
        None => return (CardRank::BLANK, CardSuit::BLANK),
        Some(r) => CardRank::from_char(r),
    };
    let suit: CardSuit = match chars.next() {
        None => return (CardRank::BLANK, CardSuit::BLANK),
        Some(s) => CardSuit::from_char(s),
    };
    (rank, suit)
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
}
