//! TODO

use crate::serialization::Serialize;

/// TODO
pub struct UniprotId {
    /// TODO
    pub id: u32
}

impl Serialize for UniprotId {
    type Output = u16;

    /// TODO
    fn serialize(&self) -> Self::Output {
        self.id as u16
    }
}
