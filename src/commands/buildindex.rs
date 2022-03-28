//! Command to build the index structure

use crate::kmer::Kmer;

use crate::errors::Result;

/// Arguments to build an index
#[derive(Debug, StructOpt)]
pub struct BuildIndexArgs {
    
}

/// Implements the buildindex command
pub fn buildindex(_args: BuildIndexArgs) -> Result<()> {
    let test: Kmer = Kmer::from("ACDFFYADD");

    let s: String = Kmer::into(test);

    println!("{}", s);

    Ok(())
}
