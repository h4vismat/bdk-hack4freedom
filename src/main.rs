use anyhow::{Result, anyhow};
use bdk_electrum::{BdkElectrumClient, electrum_client};
use bdk_wallet::rusqlite;
use clap::{Parser, Subcommand};

use crate::{descriptors::generate_descriptors_from_mnemonic, mnemonic::generate_mnemonic, send::sign_transaction};

static DB_PATH: &'static str = "wallet.sqlite";

mod descriptors;
mod mnemonic;
mod send;
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
    Balance,
    Descriptors,
    Send {
        address: String,
        satoshi: u64
    }
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

            let mut conn = rusqlite::Connection::open(DB_PATH)?;
            let mut wallet = wallet::load_wallet(&mut conn, &descriptors)?;

            let client = BdkElectrumClient::new(electrum_client::Client::new(&std::env::var("ELECTRUM_URL")?)?);
            client.populate_tx_cache(wallet.tx_graph().full_txs().map(|tx_node| tx_node.tx));

            let update = client.full_scan(wallet.start_full_scan(), 50, 5, false)?;
            wallet.apply_update(update)?;
            wallet.persist(&mut conn)?;

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
                Command::Balance => println!("Balance: {}", wallet.balance().total()),
                Command::Send { address, satoshi } => {
                    let mut psbt = send::prepare_transaction(&mut wallet, address, *satoshi)?;
                    sign_transaction(&wallet, &descriptors, &mut psbt)?;

                    let tx = psbt.extract_tx()?;
                    let txid = client.transaction_broadcast(&tx)?;

                    println!("Txid: {}", txid)
                }
                Command::Mnemonic { .. } => unreachable!("Command::Mnemonic definido anteriormente.")
            }
        }
    }

    Ok(())
}
