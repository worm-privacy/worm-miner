use alloy::primitives::Address;
use anyhow::anyhow;
use clap::ValueEnum;
use std::{env, fmt::Display, str::FromStr};

#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    ValueEnum,
    serde::Serialize,
    serde::Deserialize,
    Default,
)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    Anvil,
    Sepolia,
    #[default]
    Mainnet,
}

impl TryFrom<&str> for Network {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "anvil" => Network::Anvil,
            "sepolia" => Network::Sepolia,
            "mainnet" => Network::Mainnet,
            _ => return Err(anyhow!("Invalid network: '{value}' ")),
        })
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Network::Anvil => f.write_str("anvil"),
            Network::Sepolia => f.write_str("sepolia"),
            Network::Mainnet => f.write_str("mainnet"),
        }
    }
}

impl Network {
    pub fn url(&self) -> &'static str {
        match self {
            Network::Anvil => "http://127.0.0.1:8545",
            Network::Sepolia => "https://sepolia.drpc.org",
            Network::Mainnet => "https://eth.drpc.org",
        }
    }

    pub fn beth_address(&self) -> Result<Address, anyhow::Error> {
        let address_str = match self {
            Network::Anvil => {
                env::var("ANVIL_BETH_ADDRESS").expect("provide ANVIL_BETH_ADDRESS env variable")
            }
            Network::Sepolia => "0x98B9b8879EC255dfcfA55dF19d9FFc0987d68064".to_string(),
            Network::Mainnet => "0x5624344235607940d4d4EE76Bf8817d403EB9Cf8".to_string(),
        };
        Address::from_str(&address_str)
            .map_err(|e| anyhow!("invalid beth contract address {}, msg: {}", address_str, e))
    }

    //TODO
    pub fn worm_address(&self) -> Result<Address, anyhow::Error> {
        let address_str = match self {
            Network::Anvil => {
                env::var("ANVIL_WORM_ADDRESS").expect("provide ANVIL_WORM_ADDRESS env variable")
            }
            Network::Sepolia => "0x0eD61b3696F0dafFaE01E7EEA22711E7860b1118".to_string(),
            Network::Mainnet => "todo".to_string(),
        };
        Address::from_str(&address_str)
            .map_err(|e| anyhow!("invalid worm contract address {}, msg: {}", address_str, e))
    }

    //TODO
    pub fn staking_address(&self) -> Result<Address, anyhow::Error> {
        let address_str = match self {
            Network::Anvil => env::var("ANVIL_STAKING_ADDRESS")
                .expect("provide ANVIL_STAKING_ADDRESS env variable"),
            Network::Sepolia => "0x0116E4bDc0282419e58Af45dB79233Fb7cF02663".to_string(),
            Network::Mainnet => "todo".to_string(),
        };
        Address::from_str(&address_str).map_err(|e| {
            anyhow!(
                "invalid staking contract address {}, msg: {}",
                address_str,
                e
            )
        })
    }

    pub fn beth_to_eth_address(&self) -> Result<Address, anyhow::Error> {
        let address_str = match self {
            Network::Anvil => env::var("ANVIL_BETH_TO_ETH_ADDRESS")
                .expect("provide ANVIL_BETH_TO_ETH_ADDRESS env variable"),
            Network::Sepolia => "0xB41bD692C004672aCaDbD7162c84b4381A58cFeC".to_string(),
            Network::Mainnet => "0xbA5A285806c343AaD955a40FE4b6e5e607B752b6".to_string(),
        };

        Address::from_str(&address_str).map_err(|e| {
            anyhow!(
                "invalid BETHToETH contract address {}, msg: {}",
                address_str,
                e
            )
        })
    }
}
