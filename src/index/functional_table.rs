//! TODO: Serialized Functional data table

use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};

use crate::errors::Result;
use crate::serialization::Serializer;
use crate::serialization::uniprot_id::UniprotId;

// TODO

// TODO: Serialize uniprot ids to bytestring
// TODO: Write this string to a file 

// TODO: eerst zonder serialization
// OF: byte als character naar csv

/// TODO
#[derive(Serialize, Deserialize)]
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
    pub fn insert(&mut self, i: usize, functional_data: &Vec<UniprotId>) {
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

    /// TODO: saving the functional table
    pub fn to_bin(&self, file_path: String) -> Result<()> {
        let mut file = File::create(file_path)?;

        let encoded: Vec<u8> = bincode::serialize(self).unwrap();

        file.write_all(&encoded);

        Ok(())
    }

//    /// TODO
//    pub fn from_bin(&self, file_path: String) -> FunctionalTable {
//        
//    }
}
