use crate::{
    data::AppState,
    error::{LogIfError, ServerError},
    proof_queue_service::proof_job::ProofJob,
    utils::{get_provider, validate_account_proof},
};
use alloy::{
    eips::BlockNumberOrTag,
    primitives::{Address, Bytes, U256},
    providers::Provider,
    rpc::types::EIP1186AccountProofResponse,
};
use anyhow::anyhow;
use axum::{Json, extract::State, response::IntoResponse};
use common::{contracts::network::Network, utils::ether_amount_serializer};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// POST `/proof` gets inputs of the proof-of-burn zk circuit and starts proving.
pub async fn proof_post(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(body): Json<ProofPostRequest>,
) -> Result<ProofPostResponse, ServerError> {
    if body.network == Network::Sepolia {
        return Err(ServerError::InvalidAction(
            "sepolia network is not supported",
        ));
    }

    let provider = get_provider(body.network)?;

    let mut state = state.write().await;

    if body.prover_fee < state.config.min_prover_fee {
        return Err(ServerError::InvalidAction("prover fee is too low"));
    }

    if !state.header_cache.contains_key(&body.target_block) {
        let header = provider
            .get_block_by_number(BlockNumberOrTag::Number(body.target_block))
            .await?
            .ok_or(anyhow!("Block not found!"))?
            .header;
        state.header_cache.insert(body.target_block, header.into());
    }

    let block_header = state
        .header_cache
        .get(&body.target_block)
        .cloned()
        .ok_or(anyhow!("Header not found!"))?;

    validate_account_proof(body.account_proof.clone(), block_header.state_root).log()?;

    let job_id = state.next_job_id;
    let job = ProofJob::new(job_id, body, block_header);
    let nullifier = job.nullifier;

    if let Some(user_job_id) = state.nullifier_to_job_id.get(&nullifier) {
        if state.proof_cache.contains_key(&nullifier) {
            // Proof is already created!
            return Ok(ProofPostResponse {});
        } else {
            if let Some(running_job_id) = state.current_processing_job_id {
                if running_job_id <= *user_job_id {
                    // Proof is already in queue!
                    return Ok(ProofPostResponse {});
                } else {
                    // Loop is processing a newer job id but proof doesn't exist in cache!
                    state.nullifier_to_job_id.remove(&nullifier);
                }
            }
        }
    }

    if let Err(e) = state.job_channel.send(job) {
        return Err(ServerError::Unexpected(
            anyhow!("{e}").into_boxed_dyn_error(),
        ))
        .log_with_context("send_job_to_channel");
    }
    state.nullifier_to_job_id.insert(nullifier, job_id);
    state.next_job_id += 1;

    Ok(ProofPostResponse {})
}

#[derive(Deserialize, Debug)]
pub struct ProofPostRequest {
    pub target_block: u64,
    pub account_proof: EIP1186AccountProofResponse,

    pub network: Network,
    pub burn_key: U256,
    pub receiver_address: Address,

    #[serde(with = "ether_amount_serializer")]
    pub broadcaster_fee: U256,
    #[serde(with = "ether_amount_serializer")]
    pub prover_fee: U256,
    #[serde(with = "ether_amount_serializer")]
    pub spend: U256,

    pub receiver_hook: Bytes,
}

#[derive(Serialize)]
pub struct ProofPostResponse {}

impl IntoResponse for ProofPostResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
