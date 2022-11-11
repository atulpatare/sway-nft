contract;

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

