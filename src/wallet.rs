// src/wallet.rs

use anyhow::Result;
use bdk_wallet::{PersistedWallet, Wallet, bitcoin::Network, rusqlite};

use crate::descriptors::Descriptors;

pub fn load_wallet(conn: &mut rusqlite::Connection, descriptors: &Descriptors) -> Result<PersistedWallet<rusqlite::Connection>> {
    let wallet = match Wallet::load()
        .descriptor(bdk_wallet::KeychainKind::External, Some(descriptors.tpub_ext.clone()))
        .descriptor(bdk_wallet::KeychainKind::Internal, Some(descriptors.tpub_int.clone()))
        .check_network(Network::Regtest)
        .load_wallet(conn)? 
    {
        Some(wallet) => wallet,
        None => Wallet::create(descriptors.tpub_ext.clone(), descriptors.tpub_int.clone())
            .network(Network::Regtest)
            .create_wallet(conn)?
    };

    Ok(wallet)
}