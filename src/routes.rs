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

pub async fn handle_disperse_ether(
    _addresses: Vec<String>,
    _amounts: Vec<f64>
) -> Result<Json<String>, ApiError> {

    println!("Hello, handle_disperse_ether!");

    // Ensure the arrays have the same length
    if _addresses.len() != _amounts.len() {
        return Err(ApiError::ProviderError(ProviderError::CustomError(
            "Addresses and amounts arrays must have the same length".into()
        )));
    }    
    let transaction_hash = "Transaction_Hash_disperse_ether";

    info!("API:: Disperse_ether served");

    Ok(Json(transaction_hash.to_string()))
}

pub async fn handle_disperse_ether_by_percent(
    _addresses: Vec<String>,
    _percentages: Vec<f64>
) -> Result<Json<String>, ApiError> {
    // Ensure the arrays have the same length
    if _addresses.len() != _percentages.len() {
        return Err(ApiError::ProviderError(ProviderError::CustomError(
            "Addresses and percentages arrays must have the same length".into()
        )));
    }

    let transaction_hash = "Transaction_Hash_disperse_ether_by_percent";

    info!("API:: Disperse_ether_by_percent served");

    Ok(Json(transaction_hash.to_string()))
}

pub async fn handle_collect_ether(
    _addresses: Vec<String>,
    _amounts: Vec<f64>
) -> Result<Json<String>, ApiError> {
    // Ensure the arrays have the same length
    if _addresses.len() != _amounts.len() {
        return Err(ApiError::ProviderError(ProviderError::CustomError(
            "Addresses and amounts arrays must have the same length".into()
        )));
    }     
    let transaction_hash = "Transaction_Hash_collect_ether";

    info!("API:: Collect_ether served");

    Ok(Json(transaction_hash.to_string()))
}

pub async fn handle_collect_ether_by_percent(
    _addresses: Vec<String>,
    _percentages: Vec<f64>
) -> Result<Json<String>, ApiError> {
     // Ensure the arrays have the same length
     if _addresses.len() != _percentages.len() {
        return Err(ApiError::ProviderError(ProviderError::CustomError(
            "Addresses and percentages arrays must have the same length".into()
        )));
    }

    let transaction_hash = "Transaction_Hash_collect_ether_by_percent";

    info!("API:: Collect_ether_by_percent served");

    Ok(Json(transaction_hash.to_string()))
}

pub async fn handle_disperse_token(
    _addresses: Vec<String>,
    _amounts: Vec<f64>
) -> Result<Json<String>, ApiError> {
    // Ensure the arrays have the same length
    if _addresses.len() != _amounts.len() {
        return Err(ApiError::ProviderError(ProviderError::CustomError(
            "Addresses and amounts arrays must have the same length".into()
        )));
    }       
    let transaction_hash = "Transaction_Hash_disperse_token";

    info!("API:: Disperse_token served");

    Ok(Json(transaction_hash.to_string()))
}

pub async fn handle_disperse_token_by_percent(
    _addresses: Vec<String>,
    _percentages: Vec<f64>
) -> Result<Json<String>, ApiError> {
    // Ensure the arrays have the same length
    if _addresses.len() != _percentages.len() {
        return Err(ApiError::ProviderError(ProviderError::CustomError(
            "Addresses and percentages arrays must have the same length".into()
        )));
    }
    let transaction_hash = "Transaction_Hash_disperse_token_by_percent";

    info!("API:: Disperse_token_by_percent served");

    Ok(Json(transaction_hash.to_string()))
}

pub async fn handle_collect_token(
    _addresses: Vec<String>,
    _amounts: Vec<f64>
) -> Result<Json<String>, ApiError> {
    // Ensure the arrays have the same length
    if _addresses.len() != _amounts.len() {
        return Err(ApiError::ProviderError(ProviderError::CustomError(
            "Addresses and amounts arrays must have the same length".into()
        )));
    }    
    let transaction_hash = "Transaction_Hash_collect_token";

    info!("API:: Collect_token served");

    Ok(Json(transaction_hash.to_string()))
}

pub async fn handle_collect_token_by_percent(
    _addresses: Vec<String>,
    _percentages: Vec<f64>
) -> Result<Json<String>, ApiError> {
    if _addresses.len() != _percentages.len() {
        return Err(ApiError::ProviderError(ProviderError::CustomError(
            "Addresses and percentages arrays must have the same length".into()
        )));
    }
    let transaction_hash = "Transaction_Hash_collect_token_by_percent";

    info!("API:: Collect_token_by_percent served");

    Ok(Json(transaction_hash.to_string()))
}