contract;

dep errors;
dep interface;

use errors::{AccessError, InitError, InputError};
use interface::{MintEvent, NFT, TransferEvent};
use std::{
    auth::msg_sender,
    identity::Identity,
    logging::log,
    option::Option,
    result::Result,
    revert::require,
    storage::StorageMap,
};

storage {
    access_control: bool = false,
    admin: Option<Identity> = Option::None,
    balances: StorageMap<Identity, u64> = StorageMap {},
    max_supply: u64 = 0,
    owners: StorageMap<u64, Option<Identity>> = StorageMap {},
    total_supply: u64 = 0,
}

impl NFT for Contract {
    #[storage(read)]
    fn admin() -> Identity {
        let admin = storage.admin;
        require(admin.is_some(), InputError::AdminDoesNotExist);
        admin.unwrap()
    }

    #[storage(read)]
    fn balance_of(owner: Identity) -> u64 {
        storage.balances.get(owner)
    }

    #[storage(read, write)]
    fn constructor() {
        let admin = Option::Some(msg_sender().unwrap());
        require(storage.max_supply == 0, InitError::CannotReinitialize);
        storage.access_control = true;
        storage.admin = admin;
        storage.max_supply = 10000;
    }

    #[storage(read)]
    fn max_supply() -> u64 {
        storage.max_supply
    }

    #[storage(read, write)]
    fn mint(to: Identity) {
        let total_supply = storage.total_supply;
        let token_id = total_supply + 1;
        require(storage.max_supply >= token_id, InputError::NotEnoughTokensToMint);

        let admin = storage.admin;
        require(!storage.access_control || (admin.is_some() && msg_sender().unwrap() == admin.unwrap()), AccessError::SenderNotAdmin);

        storage.owners.insert(token_id, Option::Some(to));
        storage.balances.insert(to, storage.balances.get(to) + 1);
        storage.total_supply += 1;

        log(MintEvent {
            owner: to,
            token_id,
        });
    }

    #[storage(read)]
    fn total_supply() -> u64 {
        storage.total_supply
    }

    #[storage(read, write)]
    fn transfer_from(from: Identity, to: Identity, token_id: u64) {
        let token_owner = storage.owners.get(token_id);
        require(token_owner.is_some(), InputError::TokenDoesNotExist);
        let token_owner = token_owner.unwrap();

        storage.owners.insert(token_id, Option::Some(to));
        storage.balances.insert(from, storage.balances.get(from) - 1);
        storage.balances.insert(to, storage.balances.get(to) + 1);

        log(TransferEvent {
            from,
            to,
            token_id,
        });
    }
}