use crate::common::wait_between_tests;
use jikan_rs::{JikanClient, JikanError};
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_anime() {
    let client = JikanClient::new();
    let result = client.get_anime(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_full() {
    let client = JikanClient::new();
    let result = client.get_anime_full(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_characters() {
    let client = JikanClient::new();
    let result = client.get_anime_characters(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_staff() {
    let client = JikanClient::new();
    let result = client.get_anime_staff(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_episodes() {
    let client = JikanClient::new();
    let result = client.get_anime_episodes(1, Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_videos() {
    let client = JikanClient::new();
    let result = client.get_anime_videos(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_news() {
    let client = JikanClient::new();
    let result = client.get_anime_news(1, Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_forum() {
    let client = JikanClient::new();
    let result = client.get_anime_forum(1, None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_themes() {
    let client = JikanClient::new();
    let result = client.get_anime_themes(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_recommendations() {
    let client = JikanClient::new();
    let result = client.get_anime_recommendations(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_userupdates() {
    let client = JikanClient::new();
    let result = client.get_anime_userupdates(1, Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_reviews() {
    let client = JikanClient::new();
    let result = client.get_anime_reviews(1, Some(1), None, None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_external() {
    let client = JikanClient::new();
    let result = client.get_anime_external(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_streaming() {
    let client = JikanClient::new();
    let result = client.get_anime_streaming(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_nonexistent_anime() {
    let client = JikanClient::new();
    let result = client.get_anime(999999999).await;
    assert!(matches!(result, Err(JikanError::NotFound)));
    wait_between_tests().await;
}
