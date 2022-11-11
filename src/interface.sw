library interface;

abi NFT {
    #[storage(write)]
    fn constructor(name_: str[3], symbol_: str[3]);

    #[storage(read)]
    fn name() -> str[3];

    #[storage(read)]
    fn symbol() -> str[3];

    #[storage(read)]
    fn balance_of(owner: Identity) -> u64;
}
