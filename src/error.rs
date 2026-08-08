use core::fmt::{self, Display, Formatter};

/// Every way a card or hand can be malformed. Carved from pkcore's 52-variant
/// `PKError`; pkcore adds `impl From<CkcError> for PKError` on its side.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CkcError {
    BlankCard,
    DuplicateCard,
    Incomplete,
    InvalidBinaryFormat,
    InvalidCard,
    InvalidCardNumber,
    InvalidCardCount,
    InvalidIndex,
}

impl Display for CkcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            CkcError::BlankCard => "blank card",
            CkcError::DuplicateCard => "duplicate card",
            CkcError::Incomplete => "incomplete hand",
            CkcError::InvalidBinaryFormat => "invalid binary format",
            CkcError::InvalidCard => "invalid card",
            CkcError::InvalidCardNumber => "invalid card number",
            CkcError::InvalidCardCount => "invalid card count",
            CkcError::InvalidIndex => "invalid index",
        };
        write!(f, "{s}")
    }
}

impl core::error::Error for CkcError {}
