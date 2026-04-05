use alloy::primitives::U256;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ParticipateOutputJson {
    starting_epoch: U256,
    number_of_epochs: u64,
}

impl ParticipateOutputJson {
    pub fn new(starting_epoch: U256, number_of_epochs: u64) -> Self {
        Self {
            starting_epoch,
            number_of_epochs,
        }
    }

    pub fn to_json(&self) -> Result<String, anyhow::Error> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn starting_epoch(&self) -> U256 {
        self.starting_epoch
    }

    pub fn number_of_epochs(&self) -> u64 {
        self.number_of_epochs
    }
}
