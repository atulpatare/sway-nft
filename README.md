# NFT 
NFT contract built on fuel as per ERC721 standard

## Directory Structure
```
/interactions : dir contains script to execute functions on contract
/nft : dir containing contracts in sway
```

## Execution
- Build the contracts
```
cd nft # root dir for contracts 
forc build
```

- Run the scripts
```
# run the local fuel node
fuel-core run --db-type in-memory

# run the scripts
cargo run -p interactions
```
