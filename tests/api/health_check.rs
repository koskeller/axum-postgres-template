use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use crate::helpers::*;

#[tokio::test]
async fn test_health_check_ok() {
    let app = TestApp::new().await;

    let req = Request::get("/health_check").body(Body::empty()).unwrap();
    let resp = app.request(req).await;
    let headers = resp.headers().clone();

    assert_eq!(resp.status(), StatusCode::OK);
    assert!(headers.get("x-request-id").is_some());
    assert_eq!(headers.get("access-control-allow-origin").unwrap(), "*");
    assert!(headers.get("vary").is_some());
}

#[tokio::test]
async fn test_db_connection_ok() {
    let app = TestApp::new().await;
    assert_eq!(app.db.pool.size(), 1);
}
