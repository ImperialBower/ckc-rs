use crate::CKCNumber;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct Five(pub [CKCNumber; 5]);

impl From<[CKCNumber; 5]> for Five {
    fn from(array: [CKCNumber; 5]) -> Self {
        Five(array)
    }
}

impl TryFrom<&'static str> for Five {
    type Error = ();

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        todo!()
    }
}