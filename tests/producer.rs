use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_producer_by_id() {
    let client = JikanClient::new();
    let result = client.get_producer_by_id(1).await;
    println!("{:?}", result);
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_producer_by_id_full() {
    let client = JikanClient::new();
    let result = client.get_producer_full_by_id(1).await;
    println!("{:?}", result);
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_producer_external() {
    let client = JikanClient::new();
    let result = client.get_producer_external(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_producer_search() {
    let client = JikanClient::new();
    let result = client
        .get_producer_search(None, None, None, None, None, Some(String::from("m")))
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_producers() {
    let client = JikanClient::new();
    let result = client.get_producers().await;
    assert!(result.is_ok());
    wait_between_tests().await;
}
