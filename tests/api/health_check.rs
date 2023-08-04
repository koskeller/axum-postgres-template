use axum::http::StatusCode;

use crate::helpers::*;

#[tokio::test]
async fn health_check_ok() {
    let app = TestApp::new().await;

    let resp = app.get("/health_check").await;
    let headers = resp.headers().clone();

    assert_eq!(resp.status(), StatusCode::OK);
    assert!(headers.get("x-request-id").is_some());
    assert_eq!(headers.get("access-control-allow-origin").unwrap(), "*");
    assert!(headers.get("vary").is_some());
}

#[tokio::test]
async fn db_connection_ok() {
    let app = TestApp::new().await;
    assert_eq!(app.db.size(), 1);
}
