// src/descriptors.rs

use std::collections::BTreeMap;
use std::str::FromStr;

use anyhow::Result;
use bdk_wallet::bip39::Mnemonic;
use bdk_wallet::bitcoin::NetworkKind;
use bdk_wallet::bitcoin::bip32::DerivationPath;
use bdk_wallet::bitcoin::key::Secp256k1;
use bdk_wallet::descriptor;
use bdk_wallet::descriptor::IntoWalletDescriptor;
use bdk_wallet::keys::{DescriptorPublicKey, DescriptorSecretKey};
use bdk_wallet::miniscript::Descriptor;

static EXTERNAL_PATH: &'static str = "m/86h/1h/0h/0";
static INTERNAL_PATH: &'static str = "m/86h/1h/0h/1";

#[derive(Debug)]
pub struct Descriptors {
    pub tpub_ext: Descriptor<DescriptorPublicKey>,
    pub tpub_int: Descriptor<DescriptorPublicKey>,
    pub ext_keymap: BTreeMap<DescriptorPublicKey, DescriptorSecretKey>,
    pub int_keymap: BTreeMap<DescriptorPublicKey, DescriptorSecretKey>
}

pub fn generate_descriptors_from_mnemonic(mnemonic: &str) -> Result<Descriptors> {
    let secp = Secp256k1::new();

    let mnemonic = Mnemonic::from_str(mnemonic)?;
    let mnemonic_with_passphrase = (mnemonic, None);

    let external_path = DerivationPath::from_str(EXTERNAL_PATH)?;
    let internal_path = DerivationPath::from_str(INTERNAL_PATH)?;

    let (tpub_ext, ext_keymap) = descriptor!(tr((mnemonic_with_passphrase.clone(), external_path)))?.into_wallet_descriptor(&secp, NetworkKind::Test)?;
    let (tpub_int, int_keymap) = descriptor!(tr((mnemonic_with_passphrase.clone(), internal_path)))?.into_wallet_descriptor(&secp, NetworkKind::Test)?;

    let descriptors = Descriptors {
        tpub_ext,
        tpub_int,
        ext_keymap,
        int_keymap
    };

    Ok(descriptors)
} 