// src/main.rs
/*
 * Main executable for EvolveTrade
 */

use clap::Parser;
use evolvetrade::{Result, run};

#[derive(Parser)]
#[command(version, about = "EvolveTrade - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
