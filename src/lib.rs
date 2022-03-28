//! Contains all modules

#![deny(missing_docs)]
#![recursion_limit = "128"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate structopt;

pub mod commands;
pub mod errors;
