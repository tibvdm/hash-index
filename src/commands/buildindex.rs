//! Command to build the index structure

use crate::index::functional_table::FunctionalTable;

use crate::kmer::Kmer;

use crate::errors::Result;

/// Arguments to build an index
#[derive(Debug, StructOpt)]
pub struct BuildIndexArgs {
    
}

/// Implements the buildindex command
pub fn buildindex(_args: BuildIndexArgs) -> Result<()> {
    let mut ftable = FunctionalTable::new();

    ftable.insert();
    ftable.insert();

    Ok(())
}
