// src/mnemonic.rs

use anyhow::{Result, anyhow};
use bdk_wallet::{bip39::{Language, Mnemonic}, keys::{GeneratableKey, GeneratedKey, bip39::WordCount}, miniscript::Tap};

pub fn generate_mnemonic(words: usize) -> Result<()> {
    let word_count = match words {
        12 => Ok(WordCount::Words12),
        15 => Ok(WordCount::Words15),
        18 => Ok(WordCount::Words18),
        21 => Ok(WordCount::Words21),
        24 => Ok(WordCount::Words24),
        _ => Err(anyhow!("Invalid word count"))
    }?;

    let mnemonic: GeneratedKey<_, Tap> = Mnemonic::generate((word_count, Language::English))
        .map_err(|_| anyhow!("Failed to generate mnemonic."))?;
    println!("Mnemonic: {}", *mnemonic);

    Ok(())
}