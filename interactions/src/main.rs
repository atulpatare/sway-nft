extern crate log;
extern crate pretty_env_logger;
extern crate rand;

use std::env;
use std::str::FromStr;

use dotenv::dotenv;
use fuel_tx::Address;
use fuels::prelude::{Bech32ContractId, Contract, ContractId, Provider, TxParameters, WalletUnlocked};
use fuels::signers::fuel_crypto::SecretKey;
use fuels_abigen_macro::abigen;
use fuels_core::Identity;
use fuels_core::parameters::StorageConfiguration;

abigen!(
    NFT,
    "../nft/out/debug/nft-abi.json"
);

async fn get_contract_id(wallet: &WalletUnlocked) -> Bech32ContractId {
    let contract_address = env::var("CONTRACT_ADDRESS");
    if !contract_address.is_err() {
        println!("Using contract address from .env");
        return Bech32ContractId::from(ContractId::from_str(&*contract_address.unwrap()).unwrap());
    }
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
    let address = match env::var("NODE_URL") {
        Ok(val) => val,
        Err(_) => "127.0.0.1:4000".to_string(),
    };
    println!("Connected to node on url : {}", address);
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

    let random_wallet = WalletUnlocked::new_random(Some(provider));
    let receiver = Identity::Address(Address::from(random_wallet.address()));
    let owner = Identity::Address(Address::from(wallet.address()));

    println!("Contract address: 0x{}", contract_id.hash);
    println!("Address of the owner: 0x{}", wallet.address().hash);
    println!("Address of the receiver: 0x{}", random_wallet.address().hash);

    let mut tx_params = TxParameters::default();
    tx_params.gas_price = 1;

    let contract_methods = contract.methods();

    let constructor_result = contract_methods.constructor().tx_params(tx_params).call().await;
    constructor_result.as_ref().unwrap();
    assert_eq!(constructor_result.is_err(), false, "Constructor failed");

    let mint_result = contract_methods.mint(receiver.clone()).tx_params(tx_params).call().await;
    assert_eq!(mint_result.is_err(), false, "Mint function call failed");

    let transfer_result = contract_methods.transfer_from(receiver.clone(), owner.clone(), 1).tx_params(tx_params).call().await;
    assert_eq!(transfer_result.is_err(), false, "Token transfer failed");

    let receiver_balance = contract_methods.balance_of(receiver.clone()).tx_params(tx_params).call().await;
    let owner_balance = contract_methods.balance_of(owner.clone()).tx_params(tx_params).call().await;
    println!("Balance of the receiver {}", receiver_balance.unwrap().value);
    println!("Balance of the owner    {}", owner_balance.unwrap().value);

    Ok(())
}
