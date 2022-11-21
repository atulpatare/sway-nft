library interface;

use std::{identity::Identity, option::Option};

pub struct TransferEvent {
    from: Identity,
    to: Identity,
    token_id: u64,
}

pub struct MintEvent {
    owner: Identity,
    token_id_start: u64,
    total_tokens: u64,
}

abi NFT {
    #[storage(read)]
    fn admin() -> Identity;

    #[storage(read)]
    fn balance_of(owner: Identity) -> u64;

    #[storage(read, write)]
    fn constructor();

    #[storage(read)]
    fn max_supply() -> u64;

    #[storage(read, write)]
    fn mint(amount: u64, to: Identity);

    #[storage(read)]
    fn total_supply() -> u64;

    #[storage(read, write)]
    fn transfer_from(from: Identity, to: Identity, token_id: u64);
}