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

    #[must_use]
    fn sort(&self) -> Self;

    fn sort_in_place(&mut self);

    fn contain_blank(&self) -> bool {
        self.iter().any(|c| c == &CardNumber::BLANK)
    }

    fn is_valid(&self) -> bool {
        self.are_unique() && !self.contain_blank()
    }

    fn iter(&self) -> Iter<'_, CKCNumber>;
}
