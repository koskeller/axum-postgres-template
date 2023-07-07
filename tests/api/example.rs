use axum::http::StatusCode;
use std::collections::HashMap;

use crate::helpers::*;

#[tokio::test]
async fn example_post_ok() {
    let app = TestApp::new().await;
    let url = app.get_url("/example");
    let mut json = HashMap::new();
    json.insert("ping", "hello");
    let resp = app.reqwest.post(url).json(&json).send().await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let resp = sqlx::query!("SELECT id, uid, ping, created_at FROM example")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to query db");

    assert_eq!(resp.id, 1);
    assert_eq!(resp.ping, "hello");
}
