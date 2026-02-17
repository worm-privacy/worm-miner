use crate::data::Proof;
use crate::{data::AppState, error::ServerError};
use alloy::primitives::{Address, Bytes, U256};
use axum::{Json, extract::State, response::IntoResponse};
use common::contracts::beth_to_eth::BETHToETHContract;
use common::contracts::{beth::BETHContract, network::Network};
use common::utils::ether_amount_serializer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// POST `/relay` gets inputs of a `mintCoin()` transaction and submits on behalf of you.
pub async fn relay_post(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(body): Json<RelayPostRequest>,
) -> Result<RelayPostResponse, ServerError> {
    let (signer, broadcaster_address, min_broadcaster_fee) = {
        let config = state.read().await.config;
        (
            config.signer(),
            config.address(),
            config.min_broadcaster_fee,
        )
    };

    if body.broadcaster_fee < min_broadcaster_fee {
        return Err(ServerError::InvalidAction("broadcaster fee is too low"));
    }

    let broadcaster_call_hook = BETHToETHContract::create_swap_hook(
        body.network,
        body.broadcaster_fee,
        broadcaster_address,
    )?;

    let beth = BETHContract::new(body.network, signer).await?;

    beth.mint(
        body.proof.rapidsnark_output,
        body.proof.target_block,
        body.nullifier,
        body.remaining_coin,
        body.broadcaster_fee,
        body.reveal_amount,
        body.receiver,
        body.prover_fee,
        broadcaster_address,
        body.swap_calldata,
        broadcaster_call_hook,
        Bytes::new(),
    )
    .await?;

    Ok(RelayPostResponse {})
}

#[derive(Deserialize, Debug)]
pub struct RelayPostRequest {
    network: Network,
    proof: Proof,
    nullifier: U256,

    // poseidon3(prefix, burn_key, amount-spend)
    remaining_coin: U256,

    #[serde(with = "ether_amount_serializer")]
    broadcaster_fee: U256,
    #[serde(with = "ether_amount_serializer")]
    reveal_amount: U256,
    receiver: Address,
    #[serde(with = "ether_amount_serializer")]
    prover_fee: U256,

    swap_calldata: Bytes,
}

#[derive(Serialize)]
pub struct RelayPostResponse {}

impl IntoResponse for RelayPostResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
