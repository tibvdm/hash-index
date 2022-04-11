//! TODO: Combination of the funnel table and the functional table.

use crate::index::functional_table::FunctionalTable;
use crate::index::conflict_table::ConflictTable;

/// TODO
pub struct Index {
    /// TODO
    conflict_table: ConflictTable
    /// TODO
    functional_table: FunctionalTable
}

// TODO: Is opsplitsen echt nodig?
// Buildindex voor het bouwen van de indexstructuur
// Ander command voor het bevragen van de structuur.

impl Index {
    /// TODO
    pub fn new(amount_of_buckets: u32) -> Index {
        // TODO: Build hashtable
        hash_table: HashTable = HashTable::new(amount_of_buckets);

        // TODO: Build functional table
        functional_table: FunctionalTable = FunctionalTable::new();
        
        Index {
            hash_table: hash_table,
            functional_table: functional_table
        }
    }

    pub fn insert(&mut self, kmer: &Kmer) {
        self.hash_table.insert(kmer);

        u32 fpointer = self.functional_table.insert(kmer)

        // TODO: some reordening needs to happen for the functional component

        // TODO: after reordening, we can:
        //  1. Write the functional table to a file
        //  2. We can keep this edited part in the index, and perform the translation in 1 go.
    }

    /// TODO
    pub fn search(&self, kmer: &Kmer) -> u32 {
        // Search the k-mer in the hash table
        let fpointer: u32 = self.hash_table.get(kmer);
        // TODO: determine wheter we want to ask the csv for all the information?
    }
}
