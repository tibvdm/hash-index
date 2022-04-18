//! TODO

use crate::serialization::Serializer;

/// TODO
#[derive(Clone, PartialEq, Debug)]
pub struct UniprotId {
    /// TODO
    pub id: u32
}

impl UniprotId {
    /// TODO
    pub fn new(id: u32) -> UniprotId {
        UniprotId {
            id: id
        }
    }
}

impl Serializer for UniprotId {
    type Output = Vec<u8>;

    /// TODO
    fn serialize(&self) -> Self::Output {
        vec![self.id as u8, (self.id >> 8) as u8, (self.id >> 16) as u8, (self.id >> 24) as u8]
    }
}
