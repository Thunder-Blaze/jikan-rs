use crate::common::wait_between_tests;
use jikan_rs::{top::*, JikanClient};
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_top_anime() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(
            AnimeType::None,
            Filter::None,
            Rating::None,
            None,
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_anime_with_type() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(AnimeType::Tv, Filter::None, Rating::None, None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_anime_with_filter() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(
            AnimeType::None,
            Filter::Airing,
            Rating::None,
            None,
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_anime_with_rating() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(
            AnimeType::None,
            Filter::None,
            Rating::Pg13,
            None,
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_anime_with_sfw() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(
            AnimeType::None,
            Filter::None,
            Rating::None,
            Some(true),
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_anime_with_page() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(
            AnimeType::None,
            Filter::None,
            Rating::None,
            None,
            Some(1),
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_anime_with_limit() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(
            AnimeType::None,
            Filter::None,
            Rating::None,
            None,
            None,
            Some(10),
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_manga() {
    let client = JikanClient::new();
    let result = client
        .get_top_manga(MangaType::None, MangaFilter::None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_manga_with_type() {
    let client = JikanClient::new();
    let result = client
        .get_top_manga(MangaType::Manga, MangaFilter::None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_manga_with_filter() {
    let client = JikanClient::new();
    let result = client
        .get_top_manga(MangaType::None, MangaFilter::Publishing, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_manga_with_page() {
    let client = JikanClient::new();
    let result = client
        .get_top_manga(MangaType::None, MangaFilter::None, Some(1), None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_manga_with_limit() {
    let client = JikanClient::new();
    let result = client
        .get_top_manga(MangaType::None, MangaFilter::None, None, Some(10))
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_people() {
    let client = JikanClient::new();
    let result = client.get_top_people(None, None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_people_with_page() {
    let client = JikanClient::new();
    let result = client.get_top_people(Some(1), None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_people_with_limit() {
    let client = JikanClient::new();
    let result = client.get_top_people(None, Some(10)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_reviews() {
    let client = JikanClient::new();
    let result = client
        .get_top_reviews(ReviewType::None, None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_reviews_with_type() {
    let client = JikanClient::new();
    let result = client
        .get_top_reviews(ReviewType::Anime, None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_reviews_with_preliminary() {
    let client = JikanClient::new();
    let result = client
        .get_top_reviews(ReviewType::None, Some(true), None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_reviews_with_spoilers() {
    let client = JikanClient::new();
    let result = client
        .get_top_reviews(ReviewType::None, None, Some(true), None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_reviews_with_page() {
    let client = JikanClient::new();
    let result = client
        .get_top_reviews(ReviewType::None, None, None, Some(2))
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}
