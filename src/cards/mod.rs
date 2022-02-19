pub mod five;
pub mod two;

pub trait HandValidator {
    fn are_unique(&self) -> bool;

    fn contain_blank(&self) -> bool;

    fn is_valid(&self) -> bool {
        self.are_unique() && !self.contain_blank()
    }
}
