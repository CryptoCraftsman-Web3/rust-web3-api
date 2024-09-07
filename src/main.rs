use axum::{routing::get, Router};
use web3_backend::{routes};

use eyre::Result;
use std::net::SocketAddr;

use tokio;
use tracing::info;
use tracing_subscriber;


#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Ethers-rs part for interacting with the contract
    // let provider = Provider::<Http>::try_from("https://rpc.goerli.eth.gateway.fm").unwrap();
    // let counter_address: Address =
    //     String::from("0x5719046dd09aF1718306Fb6f6d3AB106B95C0d31").parse()?;
    // let counter = Counter::new(counter_address, Arc::new(provider));

    // Axum server
    let app = Router::new()
        .route("/api/number/", get(routes::handle_number))
        .route("/api/block_number/", get(routes::handle_block_number));
        // .layer(Extension(counter));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("LISTENING on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}