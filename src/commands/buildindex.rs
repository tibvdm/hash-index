//! Command to build the index structure

use std::io;

use crate::index::functional_table::FunctionalTable;
use crate::index::hash_table::HashTable;
use crate::serialization::uniprot_id::UniprotId;

use crate::kmer::Kmer;

use crate::errors::Result;

// TODO: ftable stuff should move to index.rs file

/// Arguments to build an index
#[derive(Debug, StructOpt)]
pub struct BuildIndexArgs {
    
}

/// Implements the buildindex command
pub fn buildindex(_args: BuildIndexArgs) -> Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(io::stdin());

    let mut ftable = FunctionalTable::new();

    let mut hash_table = HashTable::new(1);

    for record in reader.deserialize() {
        let (kmer, lca, uids): (String, u32, String) = record?;

        let uids_vec: Vec<UniprotId> = uids
            .split(';')
            .map(|s| UniprotId::new(s.trim().parse().unwrap()))
            .collect();

        let ftable_index = ftable.insert(&uids_vec);

        // println!("{}", ftable_index);

        // TODO: lca is the taxonomic information
        // TODO: ftable_index is the function pointer
        // TODO: hash(kmer)

        hash_table.insert(&Kmer::from(kmer));
    }

    println!("{:?}", hash_table.get(&Kmer::from("AAAAAAAAA")));

    println!("{:?}", hash_table.get(&Kmer::from("AAAAAAACA")));

    println!("{:?}", hash_table.get(&Kmer::from("AAAAAAAGA")));

    Ok(())
}
