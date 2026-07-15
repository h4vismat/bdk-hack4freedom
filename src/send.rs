// src/send.rs

use anyhow::Result;
use bdk_wallet::{KeychainKind, SignOptions, bitcoin::{Address, Amount, FeeRate, Network, Psbt, key::Secp256k1}, signer::SignersContainer};
use std::str::FromStr;

use crate::descriptors::Descriptors;

pub fn prepare_transaction(wallet: &mut bdk_wallet::Wallet, address: &str, satoshi: u64) -> Result<Psbt> {
    let address = Address::from_str(address)?.require_network(Network::Regtest)?;

    let mut builder = wallet.build_tx();
    builder
        .fee_rate(FeeRate::from_sat_per_vb(4).ok_or(anyhow::anyhow!("Failed to set FeeRate"))?)
        .add_recipient(address.script_pubkey(), Amount::from_sat(satoshi));

    let psbt = builder.finish()?;

    println!("Inputs: {}", psbt.inputs.len());
    println!("Outputs: {}", psbt.outputs.len());
    println!("Fee: {}", psbt.fee()?);

    Ok(psbt)
}

pub fn sign_transaction(wallet: &bdk_wallet::Wallet, descriptors: &Descriptors, psbt: &mut Psbt) -> Result<()> {
    let secp = Secp256k1::new();
    let ext_signers_container = SignersContainer::build(descriptors.ext_keymap.clone(), wallet.public_descriptor(KeychainKind::External), &secp);
    let int_signers_container = SignersContainer::build(descriptors.int_keymap.clone(), wallet.public_descriptor(KeychainKind::Internal), &secp);
    let signers: &[&SignersContainer; 2] = &[&ext_signers_container, &int_signers_container];

    let psbt_finalized = wallet.sign_with_signers(psbt, signers, SignOptions::default())?;
    assert!(psbt_finalized);

    Ok(())
}