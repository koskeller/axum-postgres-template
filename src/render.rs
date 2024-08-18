use axum::response::Html;
use leptos::*;

use crate::components::Layout;

pub fn render<F, N>(f: F) -> Html<String>
where
    F: FnOnce() -> N + 'static,
    N: IntoView + 'static,
{
    Html(
        leptos::ssr::render_to_string(move || {
            view! { <Layout>{f()}</Layout> }
        })
        .to_string(),
    )
}

