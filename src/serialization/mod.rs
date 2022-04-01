//! TODO

trait Serialize {
    fn serialize(&self) -> [u8];
    
    fn deserialize(&[u8]) -> [self];
}

pub mod ec_number;
pub mod go_term;
pub mod interpro_entry;
