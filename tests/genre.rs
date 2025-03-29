use crate::common::wait_between_tests;
use jikan_rs::genre::GenreFilter;
use jikan_rs::JikanClient;
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_anime_genres() {
    let client = JikanClient::new();
    let result = client.get_anime_genres(GenreFilter::None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_genres_genres() {
    let client = JikanClient::new();
    let result = client.get_anime_genres(GenreFilter::Genres).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_genres_explicit() {
    let client = JikanClient::new();
    let result = client.get_anime_genres(GenreFilter::ExplicitGenres).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_genres_themes() {
    let client = JikanClient::new();
    let result = client.get_anime_genres(GenreFilter::Themes).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_anime_genres_demographics() {
    let client = JikanClient::new();
    let result = client.get_anime_genres(GenreFilter::Demographics).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_genres() {
    let client = JikanClient::new();
    let result = client.get_manga_genres(GenreFilter::None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_genres_genres() {
    let client = JikanClient::new();
    let result = client.get_manga_genres(GenreFilter::Genres).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_genres_explicit() {
    let client = JikanClient::new();
    let result = client.get_manga_genres(GenreFilter::ExplicitGenres).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_genres_themes() {
    let client = JikanClient::new();
    let result = client.get_manga_genres(GenreFilter::Themes).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_genres_demographics() {
    let client = JikanClient::new();
    let result = client.get_manga_genres(GenreFilter::Demographics).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}
