use crate::{data::AppState, error::ServerError};
use alloy::primitives::{Address, U256};
use axum::{Json, extract::State, response::IntoResponse};
use common::utils::ether_amount_serializer;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

/// GET `/proof` returns minimum proving fee of the relayer.
pub async fn proof_get(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<ProofGetResponse, ServerError> {
    let config = state.read().await.config;

    Ok(ProofGetResponse {
        prover_fee_share_inv: config.prover_fee_share_inv,
        min_prover_fee: config.min_prover_fee,
        prover_address: config.address(),
    })
}

#[derive(Serialize)]
pub struct ProofGetResponse {
    prover_fee_share_inv: u64,
    #[serde(with = "ether_amount_serializer")]
    min_prover_fee: U256,
    prover_address: Address,
}

impl IntoResponse for ProofGetResponse {
    fn into_response(self) -> axum::response::Response {
        Json(&self).into_response()
    }
}
