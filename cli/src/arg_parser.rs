use alloy::{
    primitives::{utils::parse_ether, *},
    signers::local::PrivateKeySigner,
};
use clap::{Parser, Subcommand};
use common::{burn::broadcaster::Broadcaster, contracts::network::Network};
use std::{path::PathBuf, str::FromStr};

/// Worm CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Burn ETH
    Burn {
        /// Account that preforms burn
        #[arg(long, value_parser = private_key_parser)]
        private_key: PrivateKeySigner,

        /// The amount we want to send to burn address
        #[arg(long, value_parser = eth_amount_parser)]
        amount: U256,

        /// Default is amount (maximum)
        #[arg(long, value_parser = eth_amount_parser)]
        reveal: Option<U256>,

        #[arg(long, value_parser = eth_amount_parser, default_value_t = { U256::from(0) })]
        broadcaster_fee: U256,

        // Amount of tokens you want to swap for ETH
        #[arg(long, value_parser = eth_amount_parser, default_value_t = { U256::from(0) })]
        sell_for_eth: U256,

        /// User will get BETH on this address 0x...
        #[arg(long, value_parser = eth_address_parser)]
        receiver_address: Address,

        /// Default is `0` in case you want to prove it yourself
        #[arg(long, value_parser = eth_amount_parser, default_value_t = { U256::from(0) })]
        prover_fee: U256,

        /// output file
        #[arg(long)]
        out: Option<PathBuf>,

        #[arg(long, default_value_t = Network::Mainnet, value_enum)]
        network: Network,
    },

    /// In case the proving/minting fails along the way, you can recover
    Mint {
        /// Json file (ex: burn.json)
        #[arg(long)]
        file: String,

        /// Json file (ex: burn.json)
        #[arg(long, value_parser = broadcaster_parser)]
        broadcaster: Broadcaster,
    },

    /// Creates a new note file for the remaining amount (E.g note2.json)
    Spend {
        /// Note file (ex: note.json)
        #[arg(long)]
        note: PathBuf,

        #[arg(long, value_parser = eth_amount_parser)]
        amount: U256,
    },

    /// Put BETH to epochs to get Worm later,
    /// Creates `participate.json` to use in `claim` command
    Participate {
        /// signer
        #[arg(long, value_parser = private_key_parser)]
        private_key: PrivateKeySigner,

        /// Number of epochs you want to participate
        #[arg(long)]
        num_epochs: u64,

        /// How much you want to put in each epoch
        #[arg(long, value_parser = eth_amount_parser)]
        amount_per_epoch: U256,

        #[arg(long, default_value_t = Network::Mainnet, value_enum)]
        network: Network,
    },

    /// Claim Worm form finished epoch
    Claim {
        /// signer
        #[arg(long, value_parser = private_key_parser)]
        private_key: PrivateKeySigner,

        /// participate.json file that created by Participate command
        participate_file: PathBuf,
    },
}

fn eth_amount_parser(s: &str) -> Result<U256, &'static str> {
    parse_ether(s).map_err(|_| "invalid eth amount")
}

fn eth_address_parser(s: &str) -> Result<Address, &'static str> {
    Address::from_str(s).map_err(|_| "invalid address")
}

fn broadcaster_parser(s: &str) -> Result<Broadcaster, &'static str> {
    Broadcaster::try_from(s).map_err(|_| "invalid broadcaster")
}

fn private_key_parser(s: &str) -> Result<PrivateKeySigner, &'static str> {
    PrivateKeySigner::from_str(s).map_err(|_| "invalid private key")
}
