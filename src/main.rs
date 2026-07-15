use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};

use crate::{descriptors::generate_descriptors_from_mnemonic, mnemonic::generate_mnemonic};

mod descriptors;
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
    Descriptors,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Mnemonic { words } => {
            generate_mnemonic(words)?;
        },
        ref other => {
            dotenvy::dotenv()?;

            let recovery_phrase = match std::env::var("RECOVERY_PHRASE") {
                Ok(mnemonic) => mnemonic,
                Err(_) => return Err(anyhow!("RECOVERY_PHRASE deve estar definido"))
            };

            let descriptors= generate_descriptors_from_mnemonic(&recovery_phrase)?;

            match other {
                Command::Descriptors => {
                    println!("tpub external descriptor: {}", descriptors.tpub_ext);
                    println!("tpub internal descriptor: {}", descriptors.tpub_int);
                    println!("tprv external descriptor: {}", descriptors.tpub_ext.to_string_with_secret(&descriptors.ext_keymap));
                    println!("tprv internal descriptor: {}", descriptors.tpub_int.to_string_with_secret(&descriptors.int_keymap));
                },
                Command::Mnemonic { .. } => unreachable!("Command::Mnemonic definido anteriormente.")
            }
        }
    }

    Ok(())
}
