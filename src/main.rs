use error_chain::quick_main;

use structopt::StructOpt;

use hash_index::commands;
use hash_index::errors::Result;

quick_main!(|| -> Result<()> {
    match Opt::from_args() {
        Opt::BuildIndex(args) => commands::buildindex::buildindex(args)
    }
});

#[rustfmt::skip]
#[derive(Debug, StructOpt)]
pub enum Opt {
    #[structopt(name = "buildindex")] BuildIndex(commands::buildindex::BuildIndexArgs)
}
