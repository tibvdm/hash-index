//! TODO: Serialized Functional data table

use crate::serialization::Serialize;
use crate::serialization::uniprot_id::UniprotId;

// TODO

// TODO: Serialize uniprot ids to bytestring
// TODO: Write this string to a file 

// TODO: eerst zonder serialization
// OF: byte als character naar csv

/// TODO
pub struct FunctionalTable {
    /// TODO
    pub entry_counter: u32
}

impl FunctionalTable {
    /// TODO
    pub fn new() -> FunctionalTable {
        FunctionalTable {
            entry_counter: 0
        }
    }

    /// TODO: -> Result
    pub fn insert(&mut self, data: &Vec<UniprotId>) -> u32 {
        let mut serialized_vec: Vec<u8> = Vec::new();

        // TODO: add LCA

        for uid in data {
            let serialized = uid.serialize();

            for byte in serialized {
                serialized_vec.push(byte);
            }
        }
        
        // Store data in struct or write directly to file

        self.entry_counter += 1;

        return self.entry_counter - 1;
    }
}
