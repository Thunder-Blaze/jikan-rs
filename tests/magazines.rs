use crate::common::wait_between_tests;
use jikan_rs::magazines::*;
use jikan_rs::JikanClient;
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_magazines_no_params() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(None, None, None, OrderBy::None, Sort::None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_magazines_with_page() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(Some(1), None, None, OrderBy::None, Sort::None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_magazines_with_limit() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(None, Some(10), None, OrderBy::None, Sort::None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_magazines_with_query() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(
            None,
            None,
            Some("Shonen".to_string()),
            OrderBy::None,
            Sort::None,
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_magazines_with_order() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(None, None, None, OrderBy::Name, Sort::None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_magazines_with_sort() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(None, None, None, OrderBy::None, Sort::Asc, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_magazines_with_letter() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(
            None,
            None,
            None,
            OrderBy::None,
            Sort::None,
            Some("A".to_string()),
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}
