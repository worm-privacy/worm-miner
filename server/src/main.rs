mod data;
mod error;
mod handlers;
mod proof_queue_service;
mod utils;
use crate::{
    data::{AppState, config::Config},
    handlers::*,
    proof_queue_service::{
        ProofQueueService,
        proof_job::{ProofJob, run_rapidsnark_task_if_child_process},
    },
};
use axum::{
    Router,
    routing::{get, post},
};
use common::utils::MultiNetworkProvider;
use std::{process::exit, sync::Arc};
use tokio::sync::mpsc;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() {
    // this will do nothing if its not child process
    // and will `exit()` after creating proof if its child process
    run_rapidsnark_task_if_child_process();

    let config = match Config::load() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("error while loading config:\n{}", e);
            exit(1);
        }
    };
    config.print();

    let provider = MultiNetworkProvider::new(config.signer())
        .await
        .expect("error while calling MultiNetworkProvider::new()");

    let (job_tx, job_rx) = mpsc::unbounded_channel::<ProofJob>();
    let state = AppState::new(config, job_tx, provider);

    ProofQueueService::new(job_rx, Arc::clone(&state)).start();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any());

    let router = Router::new()
        .route("/proof", get(proof_get))
        .route("/proof", post(proof_post))
        .route("/proof/{nullifier}", get(proof_get_by_nullifier))
        .route("/relay", get(relay_get))
        .route("/relay", post(relay_post))
        .layer(cors)
        .with_state(state);

    let address = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("Listening on `{}`", address);
    axum::serve(listener, router).await.unwrap();
}
