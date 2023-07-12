use axum::http::StatusCode;

use crate::helpers::*;

#[tokio::test]
async fn health_check_ok() {
    let app = TestApp::new().await;
    let url = app.get_url("/health_check");
    let resp = reqwest::get(url).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let request_id = resp.headers().get("x-request-id").unwrap();
    assert!(!request_id.is_empty());
}

#[tokio::test]
async fn db_connection_ok() {
    let app = TestApp::new().await;
    assert_eq!(app.pool.size(), 1);
}
