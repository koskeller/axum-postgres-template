use axum::response::Html;
use leptos::*;

pub fn render<F, N>(f: F) -> Html<String>
where
    F: FnOnce() -> N + 'static,
    N: IntoView + 'static,
{
    #[allow(unused_braces)]
    let html = leptos::ssr::render_to_string(move || {
        view! { {f()} }
    })
    .to_string();
    // TODO find a better way
    let html = format!("<!DOCTYPE html>{}", html);
    Html(html)
}

#[macro_export]
macro_rules! html {
    ( $($tokens:tt)* ) => {
        $crate::render(move || {
            leptos::view! { $($tokens)* }
        })
    };
}
