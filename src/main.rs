use axum::{
    routing::{get, post},
    Router,
    Json,
};
use web3_backend::{routes};

use eyre::Result;
use std::net::SocketAddr;

use tokio;
use tracing::info;
use tracing_subscriber;
use serde::Deserialize;

#[derive(Deserialize)]
struct DisperseParams {
    addresses: Vec<String>,
    amounts: Vec<f64>,
}

#[derive(Deserialize)]
struct PercentParams {
    addresses: Vec<String>,
    percentages: Vec<f64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Axum server
    let app = Router::new()
        .route("/api/number/", get(routes::handle_number))
        .route("/api/block-number/", get(routes::handle_block_number))
        .route("/api/disperse-ether/", get(handle_disperse_ether_route))
        .route("/api/disperse-ether-by-percent/", get(handle_disperse_ether_by_percent_route))
        .route("/api/collect-ether/", get(handle_collect_ether_route))
        .route("/api/collect-ether-by-percent/", get(handle_collect_ether_by_percent_route))
        .route("/api/disperse-token/", get(handle_disperse_token_route))
        .route("/api/disperse-token-by-percent/", get(handle_disperse_token_by_percent_route))
        .route("/api/collect-token/", get(handle_collect_token_route))
        .route("/api/collect-token-by-percent/", get(handle_collect_token_by_percent_route));


    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("LISTENING on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_disperse_ether_route(
    Json(params): Json<DisperseParams>,
) -> Result<Json<String>, routes::ApiError> {
    println!("Hello, handle_disperse_ether_route!");
    routes::handle_disperse_ether(params.addresses, params.amounts).await
}

async fn handle_disperse_ether_by_percent_route(
    Json(params): Json<PercentParams>,
) -> Result<Json<String>, routes::ApiError> {
    routes::handle_disperse_ether_by_percent(params.addresses, params.percentages).await
}

async fn handle_collect_ether_route(
    Json(params): Json<DisperseParams>,
) -> Result<Json<String>, routes::ApiError> {
    routes::handle_collect_ether(params.addresses, params.amounts).await
}

async fn handle_collect_ether_by_percent_route(
    Json(params): Json<PercentParams>,
) -> Result<Json<String>, routes::ApiError> {
    routes::handle_collect_ether_by_percent(params.addresses, params.percentages).await
}

async fn handle_disperse_token_route(
    Json(params): Json<DisperseParams>,
) -> Result<Json<String>, routes::ApiError> {
    routes::handle_disperse_token(params.addresses, params.amounts).await
}

async fn handle_disperse_token_by_percent_route(
    Json(params): Json<PercentParams>,
) -> Result<Json<String>, routes::ApiError> {
    routes::handle_disperse_token_by_percent(params.addresses, params.percentages).await
}

async fn handle_collect_token_route(
    Json(params): Json<DisperseParams>,
) -> Result<Json<String>, routes::ApiError> {
    routes::handle_collect_token(params.addresses, params.amounts).await
}

async fn handle_collect_token_by_percent_route(
    Json(params): Json<PercentParams>,
) -> Result<Json<String>, routes::ApiError> {
    routes::handle_collect_token_by_percent(params.addresses, params.percentages).await
}