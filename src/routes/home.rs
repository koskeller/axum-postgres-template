use axum::{extract::State, response::Html};
use leptos::*;

use crate::{render, AppState};

pub async fn page(State(_state): State<AppState>) -> Html<String> {
    render(move || {
        view! { <main></main> }
    })
}

