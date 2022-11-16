## NFT 
An NFT contract on Fuel with sway.

### Following the erc721 standard
- [ERC721](https://ethereum.org/en/developers/docs/standards/tokens/erc-721/) 
- [EIP721](https://eips.ethereum.org/EIPS/eip-721)

### Build and test
- Make sure to use foll versions
```
forc 0.30.1
cargo 1.64.0
rustc 1.64.0
```

- Install fuel toolchain
```
curl --proto '=https' --tlsv1.2 -sSf \
    https://fuellabs.github.io/fuelup/fuelup-init.sh | sh
fuelup toolchain install latest
```

- Build the contracts
```
forc build
```

- Tests
```
cargo test

# to include prints
cargo test -- --nocapture
```
