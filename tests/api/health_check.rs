use axum::http::StatusCode;

use crate::helpers::*;

#[tokio::test]
async fn health_check_ok() {
    let app = TestApp::new().await;

    let resp = app.get("/health_check").await;
    assert_eq!(resp.status(), StatusCode::OK);

    let h = resp.headers();
    assert!(h.get("x-request-id").is_some());
    assert_eq!(h.get("access-control-allow-origin").unwrap(), "*");
    assert!(h.get("vary").is_some());
}

#[tokio::test]
async fn db_connection_ok() {
    let app = TestApp::new().await;
    assert_eq!(app.db.size(), 1);
}
