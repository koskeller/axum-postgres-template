use axum::{extract::State, response::Html};

use crate::{components::Layout, html, AppState};

pub async fn page(State(_state): State<AppState>) -> Html<String> {
    html!{ <Layout>"Hello"</Layout> }
}

