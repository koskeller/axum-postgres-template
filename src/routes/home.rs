use axum::{extract::State, response::Html};
use leptos::*;

use crate::{components::ui::button::Button, AppState};

pub async fn page(State(_state): State<AppState>) -> Html<String> {
    crate::components::render(move || {
        view! {
            "Hello"
            <Button>"Hello"</Button>
        }
    })
}
