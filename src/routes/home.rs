use axum::{extract::State, response::IntoResponse};

use crate::{components::Layout, html, metadata::Metadata, AppState};

pub async fn page(State(_state): State<AppState>) -> impl IntoResponse {
    let meta = Metadata {
        title: "Homepage".to_string(),
        description: "Website homepage".to_string(),
        lang: "en".to_string(),
        ..Metadata::default()
    };

    html!{ <Layout meta>"Hello world!"</Layout> }
}

