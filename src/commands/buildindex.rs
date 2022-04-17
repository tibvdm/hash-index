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
    /// TODO
    #[structopt(short = "b", long = "buckets", default_value = "1")]
    pub amount_of_buckets: usize,
}

/// Implements the buildindex command
pub fn buildindex(args: BuildIndexArgs) -> Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(io::stdin());

    let mut conflict_table = ConflictTable::new(args.amount_of_buckets);

    // Build the conflict table first
    for record in reader.deserialize() {
        let (kmer, _, _): (String, u32, String) = record?;

        conflict_table.insert(&Kmer::from(kmer.clone()));
    }

    println!("first for loop finished");

    conflict_table.finish();

    println!("conflict table finished");

    // Functional table has the same amount as entries, as the conflict stack
    let mut ftable = FunctionalTable::new(conflict_table.get_amount_of_kmers());

    let mut reader2 = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path("data/in.csv")?;

    // Then build the functional table using the ids of the conflict table
    for record in reader2.deserialize() {
        let (kmer, lca, uids): (String, u32, String) = record?;

        // println!("{}", lca);

        let uids_vec: Vec<UniprotId> = uids
            .split(';')
            .map(|s| UniprotId::new(s.trim().parse().unwrap()))
            .collect();

        let fpointer = match conflict_table.get(&Kmer::from(kmer.clone())) {
            Ok(i) => i as usize,
            Err(_) => {
                println!("{}", kmer.clone());
                bail!("sdfs")
            }
        };

        ftable.insert(fpointer, lca, &uids_vec);
    }

    println!("second for loop finished");

    ftable.to_csv("test.out.csv".to_string());

    println!("to csv finished");

    // match conflict_table.get(&Kmer::from("AAAAAAAAA")) {
    //     Ok(id) => println!("{:?}", ftable.get(id as usize)),
    //     Err(_) => bail!("LOL")
    // }

    Ok(())
}
