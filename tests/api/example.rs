use axum::http::StatusCode;

use crate::helpers::*;

#[tokio::test]
async fn example_post_ok() {
    let app = TestApp::new().await;

    let resp = app.post("/example", r#"{ "ping": "hello" }"#).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = sqlx::query!("SELECT id, uid, ping, created_at FROM example")
        .fetch_one(&app.db)
        .await
        .expect("Failed to query db");

    assert_eq!(resp.id, 1);
    assert_eq!(resp.ping, "hello");
}
