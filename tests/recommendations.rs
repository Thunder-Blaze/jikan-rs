use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_recent_anime_recommendations() {
    let client = JikanClient::new();
    let result = client.get_recent_anime_recommendations(None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_recent_anime_recommendations_with_page() {
    let client = JikanClient::new();
    let result = client.get_recent_anime_recommendations(Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_recent_manga_recommendations() {
    let client = JikanClient::new();
    let result = client.get_recent_manga_recommendations(None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_recent_manga_recommendations_with_page() {
    let client = JikanClient::new();
    let result = client.get_recent_manga_recommendations(Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}
