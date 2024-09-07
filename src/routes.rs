use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use ethers::{prelude::ContractError};
use ethers::{
    providers::{Http, Provider, ProviderError},
};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(": ContractError {0}")]
    ContractError(#[from] ContractError<Provider<Http>>),
    #[error(": ProviderError {0}")]
    ProviderError(#[from] ProviderError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = match self {
            ApiError::ContractError(err) => format!("Contract Error: {}", err),
            ApiError::ProviderError(err) => format!("Provider Error: {}", err),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

pub async fn handle_number() -> Result<Json<String>, ApiError> {
    // let number: U256 = counter.get_number().await?;
    let number = 100;
    info!("API:: Number served");

    Ok(Json(number.to_string()))
}

pub async fn handle_block_number() -> Result<Json<String>, ApiError> {
    // let block_number = counter.client.get_block_number().await?;
    let block_number = 200;
    info!("API:: Block number served");

    Ok(Json(block_number.to_string()))
}