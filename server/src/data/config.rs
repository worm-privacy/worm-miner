use alloy::primitives::utils::format_ether;
use alloy::primitives::{Address, B256};
use alloy::primitives::{U256, utils::parse_ether};
use alloy::signers::local::PrivateKeySigner;
use anyhow::anyhow;
use common::utils::ether_amount_serializer;
use common::utils::worm_home;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Config {
    // prover-fee = max(min_prover_fee, (burnAmount / prover_fee_share_inv))
    pub prover_fee_share_inv: u64,
    #[serde(with = "ether_amount_serializer")]
    pub min_prover_fee: U256,
    #[serde(with = "ether_amount_serializer")]
    pub min_broadcaster_fee: U256,
    pub private_key: B256,
    pub port: u16,
}

impl Config {
    pub fn load() -> Result<Config, anyhow::Error> {
        let path = worm_home::get_config()?;
        if !path.exists() {
            println!("server config not exist!");
            Self::create_default_file()?;
            println!("server config created at '{}'", path.to_str().unwrap());
        }
        let config: Config = serde_json::from_str(&fs::read_to_string(&path)?)?;

        // prevent user accidentally start server with no address for safety
        if config.private_key == B256::ZERO {
            return Err(anyhow!(
                "private_key is ZERO,\n change it in -> {}",
                path.to_str().unwrap()
            ));
        }

        Ok(config)
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        fs::write(worm_home::get_config()?, self.to_json()?)?;
        Ok(())
    }

    pub fn create_default_file() -> Result<(), anyhow::Error> {
        Config::default().save()
    }

    pub fn to_json(&self) -> Result<String, anyhow::Error> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn print(&self) {
        // this prevent user confusion by specifying units (ETH)
        println!(
            "\n--- Loading config from `{}` ",
            worm_home::get_config().unwrap().to_str().unwrap()
        );
        println!("prover_fee_share_inv = {} ", self.prover_fee_share_inv);
        println!(
            "min_prover_fee       = {} BETH",
            format_ether(self.min_prover_fee)
                .trim_end_matches('0')
                .trim_end_matches('.')
        );
        println!(
            "min_broadcaster_fee  = {} BETH",
            format_ether(self.min_broadcaster_fee)
                .trim_end_matches('0')
                .trim_end_matches('.')
        );
        println!("private_key          = {}", self.private_key);
        println!("address              = {}", self.address());
        println!("port                 = {}", self.port);
        println!()
    }

    pub fn signer(&self) -> PrivateKeySigner {
        PrivateKeySigner::from_bytes(&self.private_key).unwrap()
    }

    pub fn address(&self) -> Address {
        self.signer().address()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prover_fee_share_inv: 20, // 5%
            min_prover_fee: parse_ether("0.001").unwrap(),
            min_broadcaster_fee: parse_ether("0.001").unwrap(),
            port: 8080,
            private_key: B256::ZERO,
        }
    }
}
