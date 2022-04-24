use crate::{CKCNumber, CardNumber};
use core::slice::Iter;

pub mod five;
pub mod four;
pub mod seven;
pub mod six;
pub mod three;
pub mod two;

pub trait HandValidator {
    fn are_unique(&self) -> bool;

    fn first(&self) -> CKCNumber;

    #[must_use]
    fn sort(&self) -> Self;

    fn sort_in_place(&mut self);

    fn contain_blank(&self) -> bool {
        self.iter().any(|c| c == &CardNumber::BLANK)
    }

    /// A corrupt hand is one where any of the values in the array doesn't correspond to any
    /// recognized `CardNumber` or is blank.
    fn is_corrupt(&self) -> bool {
        self.iter().any(|c| CardNumber::filter(*c) == CardNumber::BLANK)
    }

    fn is_valid(&self) -> bool {
        self.are_unique() && !self.is_corrupt()
    }

    fn iter(&self) -> Iter<'_, CKCNumber>;
}
