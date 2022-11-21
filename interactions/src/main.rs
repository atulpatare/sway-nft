#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate rand;

use std::path::Path;
use std::str::FromStr;
use std::env;
use dotenv::dotenv;

use fuel_tx::Address;
use fuels::prelude::{
    Bech32ContractId, Contract, Provider, TxParameters, WalletUnlocked,
};
use fuels::signers::fuel_crypto::SecretKey;
use fuels_abigen_macro::abigen;
use fuels_core::Identity;
use fuels_core::parameters::StorageConfiguration;
use fuels_types::*;
use fuels_types::bech32::Bech32Address;
use rand::Rng;
use serde_json::de::Read;

// 0.6.5

pub fn tx_params() -> TxParameters {
    let gas_price = 0;
    let gas_limit = 1_000_000;
    let byte_price = 0;
    TxParameters::new(Some(gas_price), Some(gas_limit), Some(byte_price))
}

abigen!(
    NFT,
    "../nft/out/debug/nft-abi.json"
);

async fn get_contract_id(wallet: &WalletUnlocked) -> Bech32ContractId {
    debug!("Creating new deployment for non-existent contract");

    let _compiled =
        Contract::load_contract("../nft/out/debug/nft.bin", &None).unwrap();

    let bin_path = "../nft/out/debug/nft.bin".to_string();
    let contract_id = Contract::deploy(
        &bin_path,
        wallet,
        tx_params(),
        StorageConfiguration::default(),
    )
        .await
        .unwrap();

    contract_id
}

async fn setup_provider_and_wallet(port: u16) -> (Provider, WalletUnlocked) {
    // let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let address = format!("127.0.0.1:{}", port);
    let provider = Provider::connect(&address).await.unwrap();

    let primary_private_key = env::var("PRIVATE_KEY").unwrap();
    let secret = SecretKey::from_str(&primary_private_key, ).unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret, Some(provider.clone()));

    (provider, wallet)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let (provider, wallet) = setup_provider_and_wallet(4000).await;
    let contract_id: Bech32ContractId = get_contract_id(&wallet).await;
    info!("Using contract at {}", contract_id.to_string());
    let contract: NFT = NFT::new(contract_id, wallet);

    let secondary_private_key = env::var("PRIVATE_KEY_SECONDARY").unwrap();
    let user = WalletUnlocked::new_from_private_key(
        SecretKey::from_str(&secondary_private_key).unwrap(),
        Some(provider)
    );
    let raw_address: Bech32Address = user.address().clone();
    let receiver_identity = Identity::Address(Address::from(raw_address));

    let _ = contract.methods().mint(1, receiver_identity).call().await;

    Ok(())
}
