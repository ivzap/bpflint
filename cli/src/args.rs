use std::path::PathBuf;

use clap::ArgAction;
use clap::Parser;


/// A command line interface for `bpflint`.
#[derive(Debug, Parser)]
#[command(version = env!("VERSION"))]
pub struct Args {
    /// The BPF C source files to lint.
    pub srcs: Vec<PathBuf>,
    /// Increase verbosity (can be supplied multiple times).
    #[arg(short = 'v', long = "verbose", global = true, action = ArgAction::Count)]
    pub verbosity: u8,
}
