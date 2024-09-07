use reqwest;
use tokio;

#[tokio::test]
async fn test_number_endpoint() {
    let client = reqwest::Client::new();
    let res = client.get("http://localhost:8080/api/number/")
        .send()
        .await
        .unwrap();
    
    assert!(res.status().is_success());
    let body = res.text().await.unwrap();
    assert!(body.parse::<u64>().is_ok());
}

#[tokio::test]
async fn test_block_number_endpoint() {
    let client = reqwest::Client::new();
    let res = client.get("http://localhost:8080/api/block_number/")
        .send()
        .await
        .unwrap();
    
    assert!(res.status().is_success());
    let body = res.text().await.unwrap();
    assert!(body.parse::<u64>().is_ok());
}