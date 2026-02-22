use crate::{
    data::{AppState, Proof},
    error::ServerError,
};
use alloy::primitives::U256;
use anyhow::anyhow;
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

/// GET `/proof/{nullifier}` gets cached proof for the given nullifier.
/// nullifier should be base 10
pub async fn proof_get_by_nullifier(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(nullifier): Path<String>,
) -> Result<ProofGetByNullifierResponse, ServerError> {
    let nullifier = U256::from_str_radix(&nullifier, 10)
        .map_err(|e| ServerError::validation("nullifier", nullifier.clone(), format!("{e}")))?;

    let mut state = state.write().await;

    match state.proof_cache.get(&nullifier) {
        Some(Ok(proof)) => {
            if proof.is_expired() {
                state.proof_cache.remove(&nullifier);
                return Err(ServerError::NotFound("proof".to_string()));
            } else {
                Ok(ProofGetByNullifierResponse {
                    proof: proof.clone(),
                })
            }
        }
        Some(Err(err)) => Err(ServerError::Unexpected(
            anyhow!("{err}").into_boxed_dyn_error(),
        )),
        None => {
            if let Some(job_id) = state.nullifier_to_job_id.get(&nullifier) {
                let pos_in_queue = state
                    .current_processing_job_id
                    .map(|curr| job_id.saturating_sub(curr) as isize);
                Err(ServerError::InQueue(pos_in_queue.unwrap_or(-1)))
            } else {
                Err(ServerError::NotFound("proof".to_string()))
            }
        }
    }
}

#[derive(Serialize)]
pub struct ProofGetByNullifierResponse {
    #[serde(flatten)]
    proof: Proof,
}

impl IntoResponse for ProofGetByNullifierResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
