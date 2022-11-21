extern crate log;
extern crate pretty_env_logger;
extern crate rand;

use std::env;
use std::str::FromStr;

use fuel_tx::{Address};
use fuels::prelude::{Bech32ContractId, Contract, launch_custom_provider_and_get_wallets, Provider, TxParameters, WalletsConfig, WalletUnlocked};
use fuels::signers::fuel_crypto::SecretKey;
use fuels_abigen_macro::abigen;
use fuels_core::Identity;
use fuels_core::Identity::ContractId;
use fuels_core::parameters::StorageConfiguration;
use dotenv::dotenv;

abigen!(
    NFT,
    "../nft/out/debug/nft-abi.json"
);

async fn setup_provider_and_wallet() -> (WalletUnlocked, Bech32ContractId) {
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
        .await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::deploy(
        "../nft/out/debug/nft.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "../nft/out/debug/nft-storage_slots.json".to_string(),
        )),
    )
        .await
        .unwrap();

    (wallet.clone(), id)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let (wallet, contract_id) = setup_provider_and_wallet().await;
    println!("Using contract at {}", contract_id.to_string());

    let contract = NFT::new(contract_id.clone(), wallet.clone());
    let receiver = Identity::Address(Address::from(wallet.address()));

    let _ = contract.methods().mint(2, receiver.clone()).call().await;
    let total_supply = contract.methods().total_supply().call().await;
    let max_supply = contract.methods().max_supply().call().await;
    let balance = contract.methods().balance_of(receiver.clone()).call().await;

    println!("Total supply for the contract: {}", total_supply.unwrap().value);
    println!("Max supply for the contract: {}", max_supply.unwrap().value);
    println!("Balance of the receiver is: {}", balance.unwrap().value);

    Ok(())
}
