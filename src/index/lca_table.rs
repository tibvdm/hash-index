//! TODO: Serialized Functional data table

use std::fs;
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};

use crate::errors::Result;
use crate::serialization::Serializer;
use crate::serialization::uniprot_id::UniprotId;

/// TODO
#[derive(Serialize, Deserialize)]
pub struct LcaTable {
    /// TODO
    pub entries: Vec<u32>
}

impl LcaTable {
    /// TODO
    pub fn new(amount_of_kmers: usize) -> LcaTable {
        LcaTable {
            entries: vec![0; amount_of_kmers]
        }
    }

    /// TODO: -> Result
    pub fn insert(&mut self, i: usize, lca: u32) {
        self.entries[i] = lca;
    }

    /// TODO: error handling
    pub fn get(&self, lca_pointer: usize) -> u32 {
        self.entries[lca_pointer]
    }

    /// TODO: saving the functional table
    pub fn to_bin(&self, file_path: String) -> Result<()> {
        let mut file = File::create(file_path)?;

        let encoded: Vec<u8> = bincode::serialize(self).unwrap();

        file.write_all(&encoded);

        Ok(())
    }

    /// TODO
    pub fn from_bin(file_path: String) -> LcaTable {
        let encoded = fs::read(file_path).expect("Unable to read file");

        bincode::deserialize(&encoded[..]).unwrap()
    }
}
