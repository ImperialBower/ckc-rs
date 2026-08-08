use crate::CkcError;

//region BasicCards
const CKC_AS: u32 = 0b010000000000001000110000101001;
const CKC_KS: u32 = 0b001000000000001000101100100101;
const CKC_QS: u32 = 0b000100000000001000101000011111;
const CKC_JS: u32 = 0b000010000000001000100100011101;
const CKC_TS: u32 = 0b000001000000001000100000010111;
const CKC_9S: u32 = 0b000000100000001000011100010011;
const CKC_8S: u32 = 0b000000010000001000011000010001;
const CKC_7S: u32 = 0b000000001000001000010100001101;
const CKC_6S: u32 = 0b000000000100001000010000001011;
const CKC_5S: u32 = 0b000000000010001000001100000111;
const CKC_4S: u32 = 0b000000000001001000001000000101;
const CKC_3S: u32 = 0b000000000000101000000100000011;
const CKC_2S: u32 = 0b000000000000011000000000000010;

const CKC_AH: u32 = 0b010000000000000100110000101001;
const CKC_KH: u32 = 0b001000000000000100101100100101;
const CKC_QH: u32 = 0b000100000000000100101000011111;
const CKC_JH: u32 = 0b000010000000000100100100011101;
const CKC_TH: u32 = 0b000001000000000100100000010111;
const CKC_9H: u32 = 0b000000100000000100011100010011;
const CKC_8H: u32 = 0b000000010000000100011000010001;
const CKC_7H: u32 = 0b000000001000000100010100001101;
const CKC_6H: u32 = 0b000000000100000100010000001011;
const CKC_5H: u32 = 0b000000000010000100001100000111;
const CKC_4H: u32 = 0b000000000001000100001000000101;
const CKC_3H: u32 = 0b000000000000100100000100000011;
const CKC_2H: u32 = 0b000000000000010100000000000010;

const CKC_AD: u32 = 0b010000000000000010110000101001;
const CKC_KD: u32 = 0b001000000000000010101100100101;
const CKC_QD: u32 = 0b000100000000000010101000011111;
const CKC_JD: u32 = 0b000010000000000010100100011101;
const CKC_TD: u32 = 0b000001000000000010100000010111;
const CKC_9D: u32 = 0b000000100000000010011100010011;
const CKC_8D: u32 = 0b000000010000000010011000010001;
const CKC_7D: u32 = 0b000000001000000010010100001101;
const CKC_6D: u32 = 0b000000000100000010010000001011;
const CKC_5D: u32 = 0b000000000010000010001100000111;
const CKC_4D: u32 = 0b000000000001000010001000000101;
const CKC_3D: u32 = 0b000000000000100010000100000011;
const CKC_2D: u32 = 0b000000000000010010000000000010;

const CKC_AC: u32 = 0b010000000000000001110000101001;
const CKC_KC: u32 = 0b001000000000000001101100100101;
const CKC_QC: u32 = 0b000100000000000001101000011111;
const CKC_JC: u32 = 0b000010000000000001100100011101;
const CKC_TC: u32 = 0b000001000000000001100000010111;
const CKC_9C: u32 = 0b000000100000000001011100010011;
const CKC_8C: u32 = 0b000000010000000001011000010001;
const CKC_7C: u32 = 0b000000001000000001010100001101;
const CKC_6C: u32 = 0b000000000100000001010000001011;
const CKC_5C: u32 = 0b000000000010000001001100000111;
const CKC_4C: u32 = 0b000000000001000001001000000101;
const CKC_3C: u32 = 0b000000000000100001000100000011;
const CKC_2C: u32 = 0b000000000000010001000000000010;
//endregion

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum CardNumber {
    AceSpades = CKC_AS,
    KingSpades = CKC_KS,
    QueenSpades = CKC_QS,
    JackSpades = CKC_JS,
    TenSpades = CKC_TS,
    NineSpades = CKC_9S,
    EightSpades = CKC_8S,
    SevenSpades = CKC_7S,
    SixSpades = CKC_6S,
    FiveSpades = CKC_5S,
    FourSpades = CKC_4S,
    TreySpades = CKC_3S,
    DeuceSpades = CKC_2S,
    AceHearts = CKC_AH,
    KingHearts = CKC_KH,
    QueenHearts = CKC_QH,
    JackHearts = CKC_JH,
    TenHearts = CKC_TH,
    NineHearts = CKC_9H,
    EightHearts = CKC_8H,
    SevenHearts = CKC_7H,
    SixHearts = CKC_6H,
    FiveHearts = CKC_5H,
    FourHearts = CKC_4H,
    TreyHearts = CKC_3H,
    DeuceHearts = CKC_2H,
    AceDiamonds = CKC_AD,
    KingDiamonds = CKC_KD,
    QueenDiamonds = CKC_QD,
    JackDiamonds = CKC_JD,
    TenDiamonds = CKC_TD,
    NineDiamonds = CKC_9D,
    EightDiamonds = CKC_8D,
    SevenDiamonds = CKC_7D,
    SixDiamonds = CKC_6D,
    FiveDiamonds = CKC_5D,
    FourDiamonds = CKC_4D,
    TreyDiamonds = CKC_3D,
    DeuceDiamonds = CKC_2D,
    AceClubs = CKC_AC,
    KingClubs = CKC_KC,
    QueenClubs = CKC_QC,
    JackClubs = CKC_JC,
    TenClubs = CKC_TC,
    NineClubs = CKC_9C,
    EightClubs = CKC_8C,
    SevenClubs = CKC_7C,
    SixClubs = CKC_6C,
    FiveClubs = CKC_5C,
    FourClubs = CKC_4C,
    TreyClubs = CKC_3C,
    DeuceClubs = CKC_2C,
}

impl TryFrom<u32> for CardNumber {
    type Error = CkcError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            CKC_AS => Ok(CardNumber::AceSpades),
            CKC_KS => Ok(CardNumber::KingSpades),
            CKC_QS => Ok(CardNumber::QueenSpades),
            CKC_JS => Ok(CardNumber::JackSpades),
            CKC_TS => Ok(CardNumber::TenSpades),
            CKC_9S => Ok(CardNumber::NineSpades),
            CKC_8S => Ok(CardNumber::EightSpades),
            CKC_7S => Ok(CardNumber::SevenSpades),
            CKC_6S => Ok(CardNumber::SixSpades),
            CKC_5S => Ok(CardNumber::FiveSpades),
            CKC_4S => Ok(CardNumber::FourSpades),
            CKC_3S => Ok(CardNumber::TreySpades),
            CKC_2S => Ok(CardNumber::DeuceSpades),

            CKC_AH => Ok(CardNumber::AceHearts),
            CKC_KH => Ok(CardNumber::KingHearts),
            CKC_QH => Ok(CardNumber::QueenHearts),
            CKC_JH => Ok(CardNumber::JackHearts),
            CKC_TH => Ok(CardNumber::TenHearts),
            CKC_9H => Ok(CardNumber::NineHearts),
            CKC_8H => Ok(CardNumber::EightHearts),
            CKC_7H => Ok(CardNumber::SevenHearts),
            CKC_6H => Ok(CardNumber::SixHearts),
            CKC_5H => Ok(CardNumber::FiveHearts),
            CKC_4H => Ok(CardNumber::FourHearts),
            CKC_3H => Ok(CardNumber::TreyHearts),
            CKC_2H => Ok(CardNumber::DeuceHearts),

            CKC_AD => Ok(CardNumber::AceDiamonds),
            CKC_KD => Ok(CardNumber::KingDiamonds),
            CKC_QD => Ok(CardNumber::QueenDiamonds),
            CKC_JD => Ok(CardNumber::JackDiamonds),
            CKC_TD => Ok(CardNumber::TenDiamonds),
            CKC_9D => Ok(CardNumber::NineDiamonds),
            CKC_8D => Ok(CardNumber::EightDiamonds),
            CKC_7D => Ok(CardNumber::SevenDiamonds),
            CKC_6D => Ok(CardNumber::SixDiamonds),
            CKC_5D => Ok(CardNumber::FiveDiamonds),
            CKC_4D => Ok(CardNumber::FourDiamonds),
            CKC_3D => Ok(CardNumber::TreyDiamonds),
            CKC_2D => Ok(CardNumber::DeuceDiamonds),

            CKC_AC => Ok(CardNumber::AceClubs),
            CKC_KC => Ok(CardNumber::KingClubs),
            CKC_QC => Ok(CardNumber::QueenClubs),
            CKC_JC => Ok(CardNumber::JackClubs),
            CKC_TC => Ok(CardNumber::TenClubs),
            CKC_9C => Ok(CardNumber::NineClubs),
            CKC_8C => Ok(CardNumber::EightClubs),
            CKC_7C => Ok(CardNumber::SevenClubs),
            CKC_6C => Ok(CardNumber::SixClubs),
            CKC_5C => Ok(CardNumber::FiveClubs),
            CKC_4C => Ok(CardNumber::FourClubs),
            CKC_3C => Ok(CardNumber::TreyClubs),
            CKC_2C => Ok(CardNumber::DeuceClubs),

            _ => Err(CkcError::InvalidCardNumber),
        }
    }
}

impl CardNumber {
    /// Replaces strum's `EnumIter`. Deck order: spades, hearts, diamonds, clubs,
    /// each ace-high to deuce.
    pub const ALL: [CardNumber; 52] = [
        CardNumber::AceSpades,
        CardNumber::KingSpades,
        CardNumber::QueenSpades,
        CardNumber::JackSpades,
        CardNumber::TenSpades,
        CardNumber::NineSpades,
        CardNumber::EightSpades,
        CardNumber::SevenSpades,
        CardNumber::SixSpades,
        CardNumber::FiveSpades,
        CardNumber::FourSpades,
        CardNumber::TreySpades,
        CardNumber::DeuceSpades,
        CardNumber::AceHearts,
        CardNumber::KingHearts,
        CardNumber::QueenHearts,
        CardNumber::JackHearts,
        CardNumber::TenHearts,
        CardNumber::NineHearts,
        CardNumber::EightHearts,
        CardNumber::SevenHearts,
        CardNumber::SixHearts,
        CardNumber::FiveHearts,
        CardNumber::FourHearts,
        CardNumber::TreyHearts,
        CardNumber::DeuceHearts,
        CardNumber::AceDiamonds,
        CardNumber::KingDiamonds,
        CardNumber::QueenDiamonds,
        CardNumber::JackDiamonds,
        CardNumber::TenDiamonds,
        CardNumber::NineDiamonds,
        CardNumber::EightDiamonds,
        CardNumber::SevenDiamonds,
        CardNumber::SixDiamonds,
        CardNumber::FiveDiamonds,
        CardNumber::FourDiamonds,
        CardNumber::TreyDiamonds,
        CardNumber::DeuceDiamonds,
        CardNumber::AceClubs,
        CardNumber::KingClubs,
        CardNumber::QueenClubs,
        CardNumber::JackClubs,
        CardNumber::TenClubs,
        CardNumber::NineClubs,
        CardNumber::EightClubs,
        CardNumber::SevenClubs,
        CardNumber::SixClubs,
        CardNumber::FiveClubs,
        CardNumber::FourClubs,
        CardNumber::TreyClubs,
        CardNumber::DeuceClubs,
    ];

    pub fn iter() -> core::slice::Iter<'static, CardNumber> {
        Self::ALL.iter()
    }
}

#[cfg(test)]
mod card_number_tests {
    use super::*;

    #[test]
    fn all_is_a_complete_deck() {
        assert_eq!(CardNumber::ALL.len(), 52);
        // Every entry distinct.
        for (i, a) in CardNumber::ALL.iter().enumerate() {
            for b in &CardNumber::ALL[i + 1..] {
                assert_ne!(a, b, "duplicate entry in CardNumber::ALL");
            }
        }
    }

    #[test]
    fn try_from_roundtrips_every_card() {
        for card in CardNumber::iter() {
            let n = *card as u32;
            assert_eq!(CardNumber::try_from(n), Ok(*card));
        }
    }

    #[test]
    fn try_from_rejects_garbage() {
        assert_eq!(CardNumber::try_from(23), Err(CkcError::InvalidCardNumber));
        assert_eq!(CardNumber::try_from(0), Err(CkcError::InvalidCardNumber));
    }
}
