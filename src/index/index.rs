//! TODO: Combination of the funnel table and the functional table.

// TODO

/// TODO
pub struct BowtieIndex {
    /// TODO
    hash_table: HashTable
    /// TODO
    functional_table: FunctionalTable
}

// TODO: Is opsplitsen echt nodig?
// Buildindex voor het bouwen van de indexstructuur
// Ander command voor het bevragen van de structuur.

impl BowtieIndex {
    /// TODO
    pub fn new() -> BowtieIndex {
        // TODO: Build hashtable
        hash_table: HashTable = HashTable::new();

        // TODO: Build functional table
        functional_table: FunctionalTable = FunctionalTable::new();
        
        BowtieIndex {
            hash_table: hash_table,
            functional_table: functional_table
        }
    }

    /// TODO
    pub fn search(&self, kmer: &Kmer) -> u32 {
        // Search the k-mer in the hash table
        let fpointer: u32 = self.hash_table.get(kmer);
        // TODO: determine wheter we want to ask the csv for all the information?
    }
}
