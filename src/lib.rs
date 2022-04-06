//! Contains all modules

#![deny(missing_docs)]
#![recursion_limit = "128"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate structopt;

pub mod benches;
pub mod commands;
pub mod hash;
pub mod index;
pub mod serialization;
pub mod errors;
pub mod kmer;
