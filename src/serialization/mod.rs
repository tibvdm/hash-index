//! TODO

/// TODO
pub trait Serialize {
    /// TODO
    type Output;

    /// TODO
    fn serialize(&self) -> Self::Output;
}

// pub mod ec_number;
// pub mod go_term;
// pub mod interpro_entry;
pub mod uniprot_id;
