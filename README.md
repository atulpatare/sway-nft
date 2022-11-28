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

- Deployment Details

```
Deployed from 
Fuel Wallet : fuel17qtutwm8cvf0jq0v2peesusfxkuc9hmr9cpt80v3ul46spxjnp5qafqfth
Wallet Hash : 0xf017c5bb67c312f901ec507398720935b982df632e02b3bd91e7eba804d29868

Result
Contract Id : 0x8ebf2b98f443137a9920930180af22fef99043fa8005f93c34b4fee2805a8672
```

```
1. Contract Create

Transaction Hash  : 0x6c6ddb1a017b971f12ccc03f8a3d77b2764f824abd712e0ef120ea83815e1616
Block Id          : 0x1edf40920d50563f16129c99a61af2de43ef4c9de45709603bf23187edc7a9fb
Block Height      : 190876
Time              : 4611686020097029825

2. Constructor Call

Transaction Hash  : 0x409138afe0b7e2b5b5e433e860086ce0310fd7bf9cf1302879d784aebfa1d895
Block Id          : 0xd99df96ec8f59f5e144f3d34556932d6b9b1145ea48360ca6e448b8abfdb191e
Block Height      : 190999
Time              : 4611686020097030188

3. Mint Call

Transaction Hash  : 0x2b4fe45d3e50ebfd796d903c76b569ce77f7ff9101fa71bb9a86c29984760cbb
Block Id          : 0x01bcb0847eb11292aa0c3a8578e1bcf10cefce9c84f5e7151ec7824f482df332
Block Height      : 191004
Time              : 4611686020097030194

4. Transfer Call

Transaction Hash  : 0xfab8d892e97cf61f509f3f5c8546d61f4a874cc10196398432f313a00fade79b
Block Id          : 0xff312394ea12ce7ee9b42f5542cf19de6d0d408efb7408595f619c50a69a2c8a
Block Height      : 191007
Time              : 4611686020097030208
```
