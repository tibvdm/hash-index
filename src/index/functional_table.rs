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
    pub fn new(amount_of_kmers: usize) -> FunctionalTable {
        FunctionalTable {
            serialized_entries: vec![vec![]; amount_of_kmers]
        }
    }

    /// TODO: -> Result
    pub fn insert(&mut self, i: usize, lca: u32, functional_data: &Vec<UniprotId>) {
        // Store LCA in the first 32 bits of the serialized information stream
        self.serialized_entries[i].push((lca >> 24) as u8);
        self.serialized_entries[i].push((lca >> 16) as u8);
        self.serialized_entries[i].push((lca >> 8)  as u8);
        self.serialized_entries[i].push( lca        as u8);

        // Store all functional components
        for uid in functional_data {
            for byte in uid.serialize() {
                self.serialized_entries[i].push(byte);
            }
        }
    }

    /// TODO: error handling
    pub fn get(&self, functional_pointer: usize) -> &Vec<u8> {
        &self.serialized_entries[functional_pointer]
    }
}
