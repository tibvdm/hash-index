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
    pub serialized_entries: Vec<Vec<u8>>
}

impl FunctionalTable {
    /// TODO
    pub fn new() -> FunctionalTable {
        FunctionalTable {
            serialized_entries: vec![]
        }
    }

    /// TODO: -> Result
    pub fn insert(&mut self, lca: u32, functional_data: &Vec<UniprotId>) -> u32 {
        let mut serialized_vec: Vec<u8> = Vec::new();

        // Store LCA in the first 32 bits of the serialized information stream
        serialized_vec.push((lca >> 24) as u8);
        serialized_vec.push((lca >> 16) as u8);
        serialized_vec.push((lca >> 8)  as u8);
        serialized_vec.push( lca        as u8);

        for uid in functional_data {
            for byte in uid.serialize() {
                serialized_vec.push(byte);
            }
        }
        
        // Store the functional information
        self.serialized_entries.push(serialized_vec);

        return self.serialized_entries.len() as u32 - 1;
    }

    /// TODO: error handling
    pub fn get(&self, functional_pointer: usize) -> &Vec<u8> {
        &self.serialized_entries[functional_pointer]
    }

    /// TODO
    pub fn finish() {
        // TODO
    }
}
