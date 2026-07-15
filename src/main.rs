use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

use crate::mnemonic::generate_mnemonic;

mod mnemonic;

#[derive(Debug, Parser)]
#[command(
    name = "h4f-wallet", 
    version,
    about = "Uma carteira de Bitcoin CLI."
)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Debug, Subcommand)]
enum Command {
    Mnemonic {
        #[arg(long, default_value_t = 12)]
        words: usize
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Mnemonic { words } => {
            generate_mnemonic(words)
        }
    }
}
