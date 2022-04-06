//! Define all possible errors

error_chain! {
    foreign_links {
        Csv(csv::Error) #[doc = "CSV"];
    }
}
