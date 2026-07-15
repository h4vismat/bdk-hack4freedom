use anyhow::{Result, anyhow};
use bdk_wallet::{Wallet, bitcoin::Network, rusqlite};
use clap::{Parser, Subcommand};

use crate::{descriptors::generate_descriptors_from_mnemonic, mnemonic::generate_mnemonic};

static DB_PATH: &'static str = "wallet.sqlite";

mod descriptors;
mod mnemonic;
mod wallet;

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
    Address,
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
                Err(_) => return Err(anyhow!("MNEMONIC deve estar definido"))
            };

            let descriptors= generate_descriptors_from_mnemonic(&recovery_phrase)?;

            let mut conn = rusqlite::Connection::open(DB_PATH)?;
            let mut wallet = wallet::load_wallet(&mut conn, &descriptors)?;

            match other {
                Command::Descriptors => {
                    println!("tpub external descriptor: {}", descriptors.tpub_ext);
                    println!("tpub internal descriptor: {}", descriptors.tpub_int);
                    println!("tprv external descriptor: {}", descriptors.tpub_ext.to_string_with_secret(&descriptors.ext_keymap));
                    println!("tprv internal descriptor: {}", descriptors.tpub_int.to_string_with_secret(&descriptors.int_keymap));
                },
                Command::Address => { 
                    let addr = wallet.reveal_next_address(bdk_wallet::KeychainKind::External);
                    println!("{}", addr);
                    wallet.persist(&mut conn)?;
                },
                Command::Mnemonic { .. } => unreachable!("Command::Mnemonic definido anteriormente.")
            }
        }
    }

    Ok(())
}
