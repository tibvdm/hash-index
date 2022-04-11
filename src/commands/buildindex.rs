//! Command to build the index structure

use std::io;

use crate::index::functional_table::FunctionalTable;
use crate::index::conflict_table::ConflictTable;
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

    let mut conflict_table = ConflictTable::new(2);

    // Build the conflict table first
    for record in reader.deserialize() {
        let (kmer, _, _): (String, u32, String) = record?;

        conflict_table.insert(&Kmer::from(kmer));
    }

    conflict_table.finish();

    // Functional table has the same amount as entries, as the conflict stack
    let mut ftable = FunctionalTable::new(conflict_table.get_amount_of_kmers());

    let mut reader2 = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path("data/test.csv")?;

    // Then build the functional table using the ids of the conflict table
    for record in reader2.deserialize() {
        let (kmer, lca, uids): (String, u32, String) = record?;

        let uids_vec: Vec<UniprotId> = uids
            .split(';')
            .map(|s| UniprotId::new(s.trim().parse().unwrap()))
            .collect();

        let fpointer = conflict_table.get(&Kmer::from(kmer))? as usize;

        ftable.insert(fpointer, lca, &uids_vec);
    }

    match conflict_table.get(&Kmer::from("AAAAAAAFA")) {
        Ok(id) => println!("{:?}", ftable.get(id as usize)),
        Err(_) => bail!("LOL")
    }

    Ok(())
}
