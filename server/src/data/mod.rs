pub mod config;

use crate::{data::config::Config, proof_queue_service::proof_job::ProofJob};
use alloy::{consensus::Header, primitives::U256};
use common::{mint::proof_generator::RapidsnarkOutput, utils::MultiNetworkProvider};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, mpsc::UnboundedSender};

pub struct AppState {
    pub config: Config,

    /// block header cache, prevents extra onChain calls
    pub header_cache: HashMap<u64, Header>,

    /// <nullifier, ProofResult>
    pub proof_cache: HashMap<U256, ProofResult>,
    pub job_channel: UnboundedSender<ProofJob>,

    pub nullifier_to_job_id: HashMap<U256, usize>,
    pub current_processing_job_id: Option<usize>,
    pub next_job_id: usize,
    pub provider: MultiNetworkProvider,
}

impl AppState {
    pub fn new(
        config: Config,
        job_channel: UnboundedSender<ProofJob>,
        provider: MultiNetworkProvider,
    ) -> Arc<RwLock<AppState>> {
        Arc::new(RwLock::new(AppState {
            config,
            header_cache: Default::default(),
            proof_cache: Default::default(),
            job_channel,
            current_processing_job_id: None,
            next_job_id: 0,
            nullifier_to_job_id: Default::default(),
            provider,
        }))
    }
}

pub type ProofResult = Result<Proof, anyhow::Error>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Proof {
    pub target_block: U256,
    #[serde(flatten)]
    pub rapidsnark_output: RapidsnarkOutput,
}

impl Proof {
    pub fn new(target_block: U256, rapidsnark_output: RapidsnarkOutput) -> Self {
        Self {
            target_block,
            rapidsnark_output,
        }
    }
}
