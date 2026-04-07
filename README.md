# WORM Miner

This repository contains two tools:
1. `cli` a command-line tool that lets you interact with WORM by running different subcommands. You can create proofs on your own machine...
2. `server` helps you run a WORM miner server that other people can use to generate proof and submit proof and you will get fees.

# How to Build from source:

First you need rust toolchain:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Compile CLI:
```
cargo build --bin cli
```
You can find you binary in `target/release/cli`

### Compile Server:
```
cargo build --bin server
```
You can find you binary in `target/release/server`

Note: If you want to deploy with **Docker**, check out [this](https://github.com/orgs/worm-privacy/packages?repo_name=worm-miner)

# CLI sub commands

## Burn

`worm-miner burn --network sepolia/anvil/mainnet --private-key 0x1234... --amount 1.0 --reveal 0.5 --broadcaster-fee 0.1 --sell-for-eth 0.1 --receiver-address 0x.. --prover-fee 0.01`

- `--network`: Optional (Default: `mainnet`)
- `--private-key`: Required (the private key of the account performing the burn)
- `--amount`: Required (The amount we want to send to the burn-address)
- `--reveal`: Optional (Default: maximum, same as `--amount`) (You can partially reveal the burned amount as BETH and encrypt the rest in a note file)
- `--broadcaster-fee`: Optional (Default: 0)
- `--sell-for-eth`: Optional (Default: 0) Part of the reveal amount can be sold in exchange of ETH
- `--receiver_address`: Required (Address) user will get BETH on this address
- `--prover-fee`: Default is `0` in case you want to prove it yourself

The burn info (Burn-key, amount etc.) is stored in `burn.json` in case of failure.
The remaining coin (`--amount` - `--reveal`) will be saved as a note in a JSON file.

## Mint

Minting BETH

`worm-miner mint --broadcaster 0x1234... burn.json`

- `--broadcaster`: Required (can be a HTTP endpoint `https://prover-1.worm.cx` (If we want someone else to broadcast for us, see the Relay section) or a private key `0x...` (If we want to broadcast ourself with another private key))

## Participate

`worm-miner participate --private-key 0x1234... --num-epochs 10 --amount-per-epoch 0.1`

- `--num-epochs`: Participate in N next epochs
- `--amount-per-epoch`: Put X BETH per epochs
- `--network`: Optional (Default: `mainnet`)

Creates a participation file: `participate_10_0.1.json`

## Claim

`worm-miner claim --private-key 0x1234... participate_10_0.1.json`

Claim input participation.


Note: to use anvil network you should provide `ANVIL_BETH_ADDRESS` `ANVIL_WORM_ADDRESS` and `ANVIL_STAKING_ADDRESS` env variables if needed

## Server

Spins up a HTTP server, generates proofs and broadcasts them on behalf of others.

- GET `/proof` returns minimum proving fee of the relayer.
- POST `/proof` accepts proof-of-burn zk circuit inputs and starts proof generation
- GET `/proof/{nullifier}` gets cached proof for the given nullifier.
- GET `/relay` returns minimum broadcasting fee of the relayer
- POST `/relay` gets inputs of a `mintCoin()` transaction and submits on behalf of you.

Note: The first time you run `server` binary it will create a configuration file at `~/.worm-miner/server_config.json`:
```json
{
  "min_prover_fee": "0.001",
  "min_broadcaster_fee": "0.001",
  "private_key": "0x12345678...",
  "port": 8080
}
```
This private key is used to sign proof submission transactions and will collect all fees (prover-fee and broadcaster-fee)