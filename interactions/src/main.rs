extern crate log;
extern crate pretty_env_logger;
extern crate rand;

use std::env;
use std::str::FromStr;

use fuel_tx::{Address, ContractId};
use fuels::prelude::{
    Bech32ContractId, Contract, Provider, TxParameters, WalletUnlocked,
};
use fuels::signers::fuel_crypto::SecretKey;
use fuels_abigen_macro::abigen;
use fuels_core::Identity;
use fuels_core::parameters::StorageConfiguration;
use dotenv::dotenv;

abigen!(
    NFT,
    "../nft/out/debug/nft-abi.json"
);

async fn get_contract_id(wallet: &WalletUnlocked) -> Bech32ContractId {
    let bin_path = "../nft/out/debug/nft.bin".to_string();
    let contract_id = Contract::deploy(
        &bin_path,
        wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "../nft/out/debug/nft-storage_slots.json".to_string()
        )),
    )
        .await
        .unwrap();

    contract_id
}

async fn setup_provider_and_wallet() -> (Provider, WalletUnlocked) {
    // let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let address = "127.0.0.1:4000";
    let provider = Provider::connect(&address).await.unwrap();

    let primary_private_key = env::var("PRIVATE_KEY").unwrap();
    let secret = SecretKey::from_str(&primary_private_key).unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret, Some(provider.clone()));

    (provider, wallet)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let (provider, wallet) = setup_provider_and_wallet().await;
    let contract_id: Bech32ContractId = get_contract_id(&wallet).await;
    let contract: NFT = NFT::new(contract_id.clone(), wallet.clone());
    println!("Using contract at {}", contract_id.to_string());

    let random_wallet = WalletUnlocked::new_random(Some(provider));
    let receiver = Identity::Address(Address::from(random_wallet.address()));
    let owner = Identity::Address(Address::from(wallet.address()));
    println!("Address of the owner: {}", wallet.address().to_string());
    println!("Address of the receiver: {}", random_wallet.address().to_string());

    let _ = contract.methods().constructor().call().await;
    let _ = contract.methods().mint(10, receiver.clone()).call().await;

    let balance = contract.methods().balance_of(receiver.clone()).call().await;
    let owner_balance = contract.methods().balance_of(owner.clone()).call().await;
    println!("Balance of the receiver is: {}", balance.unwrap().value);
    println!("Balance of the owner is : {}", owner_balance.unwrap().value);

    let _ = contract.methods().transfer_from(receiver.clone(), owner.clone(), 1).call().await;
    let updated_balance = contract.methods().balance_of(receiver.clone()).call().await;
    let owner_updated_balance = contract.methods().balance_of(owner.clone()).call().await;

    println!("Balance of the receiver after transfer is: {}", updated_balance.unwrap().value);
    println!("Balance of the owner is now : {}", owner_updated_balance.unwrap().value);

    Ok(())
}
