use crate::{burn::extra_commitment::ExtraCommitment, contracts::network::Network};
use alloy::primitives::{Bytes, U256};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct BurnOutput {
    // So we don't need to get it from user again on burn
    pub network: Network,

    // Burn_address 2nd param
    pub burn_key: U256, // Save as string because it's a pretty large number

    // Burn_address 3rd param
    pub burn_amount: U256, // Save as string because it's a pretty large number
    pub reveal_amount: U256, // Save as string because it's a pretty large number

    // Extra commitment content
    pub extra_commitment: ExtraCommitment,

    pub receiver_hook: Bytes,
}

impl BurnOutput {
    pub fn new(
        network: Network,
        burn_key: U256,
        burn_amount: U256,
        reveal_amount: U256,
        extra_commitment: ExtraCommitment,
        receiver_hook: Bytes,
    ) -> Self {
        Self {
            network,
            burn_key,
            burn_amount,
            reveal_amount,
            extra_commitment,
            receiver_hook,
        }
    }

    pub fn from_json(s: &str) -> Result<Self, anyhow::Error> {
        Ok(serde_json::from_str(s)?)
    }

    pub fn to_json(&self) -> Result<String, anyhow::Error> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}
