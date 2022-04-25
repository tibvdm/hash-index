//! Command to build the index structure

use std::io;
use std::fs::File;
use std::io::Write;

use crate::index::functional_table::FunctionalTable;
use crate::index::lca_table::LcaTable;
use crate::index::conflict_table::ConflictTable;
use crate::serialization::uniprot_id::UniprotId;

use crate::kmer;
use crate::kmer::Kmer;

use crate::errors::Result;

// TODO: ftable stuff should move to index.rs file

/// Arguments to build an index
#[derive(Debug, StructOpt)]
pub struct LoadIndexArgs {
    /// TODO
    #[structopt(short = "b", long = "buckets", default_value = "1")]
    pub amount_of_buckets: usize,
}

/// Implements the buildindex command
pub fn loadindex(args: LoadIndexArgs) -> Result<()> {
    let mut conflict_table: ConflictTable = ConflictTable::new(1_000);

    let kmers: Vec<Kmer> = kmer::generate_kmers(1_000_000);

    for kmer in kmers.iter() {
        conflict_table.insert(&kmer);
    }

    conflict_table.finish();

    for kmer in kmers.iter() {
        conflict_table.get(&kmer);
    }


//    let conflict_table: ConflictTable = ConflictTable::from_bin("results/hash.bin".to_string());
//
//    println!("conflict table loaded");
//
//    let lca_table: LcaTable = LcaTable::from_bin("results/lca.bin".to_string());
//
//    println!("lca table loaded");
//
//    let function_table: FunctionalTable = FunctionalTable::from_bin("results/function.bin".to_string());
//
//    println!("function table loaded");
//
//    match conflict_table.get(&Kmer::from("AAAAAAAAA")) {
//        Ok(id) => println!("lca: {:?}, functions: {:?}", lca_table.get(id as usize), function_table.get(id as usize)),
//        Err(_) => println!("LOL")
//    }
//
//    match conflict_table.get(&Kmer::from("AAAAAAAAC")) {
//        Ok(id) => println!("lca: {:?}, functions: {:?}", lca_table.get(id as usize), function_table.get(id as usize)),
//        Err(_) => println!("LOL")
//    }
//
//    match conflict_table.get(&Kmer::from("AAAAAAAAD")) {
//        Ok(id) => println!("lca: {:?}, functions: {:?}", lca_table.get(id as usize), function_table.get(id as usize)),
//        Err(_) => println!("LOL")
//    }
//
//    match conflict_table.get(&Kmer::from("AAAAAAAAE")) {
//        Ok(id) => println!("lca: {:?}, functions: {:?}", lca_table.get(id as usize), function_table.get(id as usize)),
//        Err(_) => println!("LOL")
//    }
//
//    match conflict_table.get(&Kmer::from("AAAAAAAAF")) {
//        Ok(id) => println!("lca: {:?}, functions: {:?}", lca_table.get(id as usize), function_table.get(id as usize)),
//        Err(_) => println!("LOL")
//    }
//
//    match conflict_table.get(&Kmer::from("AAAAAAACA")) {
//        Ok(id) => println!("lca: {:?}, functions: {:?}", lca_table.get(id as usize), function_table.get(id as usize)),
//        Err(_) => println!("LOL")
//    }
//
//    match conflict_table.get(&Kmer::from("AAAAAAAFA")) {
//        Ok(id) => println!("lca: {:?}, functions: {:?}", lca_table.get(id as usize), function_table.get(id as usize)),
//        Err(_) => println!("LOL")
//    }
//
//    Ok(())

    Ok(())
}
