//! TODO: Serialized Functional data table

use crate::serialization::Serialize;

// TODO

// TODO: SErialize uniprot ids to bytestring
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
    pub fn insert(&mut self/*, data: &dyn Serialize<Output=[u8]>*/) {
        // TODO: serialize data
        
        // Store data in struct or write directly to file

        println!("{}", self.entry_counter);

        self.entry_counter += 1;
    }
}
