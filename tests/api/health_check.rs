use axum::http::StatusCode;

use crate::helpers::*;

#[tokio::test]
async fn health_check_ok() {
    let app = TestApp::new().await;

    let resp = app.get("/health_check").await;
    assert_eq!(resp.status(), StatusCode::OK);

    let request_id = resp.headers().get("x-request-id").unwrap();
    assert!(!request_id.is_empty());
}

#[tokio::test]
async fn db_connection_ok() {
    let app = TestApp::new().await;
    assert_eq!(app.db.size(), 1);
}
