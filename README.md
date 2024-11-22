# pump Pump.fun contract
This is the Rust/Anchor Smart contract for building pump project forking from Pump.fun.
The trigger to move to raydium and meteora is not marketcap, just period.

# Contracts

- Localnet - ``
- Devnet - ``
- Mainnet - ``

- Check version `solana-install update`

- `solana config set --keypair <path_to_your_keypair>`
- Build: `anchor build`
- Deploy: `solana program deploy target/deploy/pump.so --with-compute-unit-price 20000 --keypair ./keypair.json`
- Close: `solana program close <Contract_Address> --keypair <Keypair_Path> --bypass-warning`
- Clean: `anchor clean`

# Init Contract

- `anchor run test`
- make sure you change config
- add the idl meta address
- `anchor run test -- --priority-fee 9000 --provider.cluster `

# Build

- anchor build
- Find address `solana address -k target/deploy/pump-keypair.json`
- Change contract
- anchor build
- deploy

# Balance

solana balance --keypair ./keypair.json

# Reclaim SOL

solana program show --buffers --keypair ./keypair.json
solana program close --buffers --keypair ./keypair.json

# Upgrading Contracts

- Build
- Copy keypair of ENV
- Build & Change Anchor.toml and lib.rs
- Deploy

