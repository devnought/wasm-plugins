use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Wasm component plugin path
    #[clap(short, long)]
    pub path: PathBuf,

    /// Sleep interval in milliseconds between plugin invocations
    #[clap(short, long, default_value_t = 1000)]
    pub sleep: u64,
}
