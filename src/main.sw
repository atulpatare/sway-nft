contract;

dep interface;

use interface::{NFT};

storage {
    // token name
    name: str[3] = "Gas",
    // token symbol
    symbol: str[3] = "GAS",
    // owners for token id
    owners: StorageMap<u64, Option<Identity>> = StorageMap {},
    // balances of the owner
    balances: StorageMap<Option<Identity>, u64> = StorageMap {},
    // approvals for token id
    token_approvals: StorageMap<u64, Option<Identity>> = StorageMap {},
    // owner to operator approvals
    operator_approvals: StorageMap<(Identity, Identity), bool> = StorageMap {}, 
}

impl NFT for Contract {
    #[storage(write)]
    fn constructor(name_: str[3], symbol_: str[3]) {
        storage.name = name_;
        storage.symbol = symbol_;
    }

    #[storage(read)]
    fn name() -> str[3] {
        storage.name
    }

    #[storage(read)]
    fn symbol() -> str[3] {
        storage.symbol
    }
}

