use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(short, long)]
    pub path: PathBuf,

    #[clap(short, long, default_value_t = 1000)]
    pub sleep: u64,
}
