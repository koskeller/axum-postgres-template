use axum::{extract::State, response::Html};
use leptos::*;

use crate::{components::search_form::SearchForm, AppState};

pub async fn page(State(_state): State<AppState>) -> Html<String> {
    crate::components::render(move || {
        view! {
            <main>
                <div class="w-full max-w-2xl mx-auto py-8 sm:py-20 md:py-40 px-4 md:px-6">
                    <h1 class="py-4 font-bold italic text-4xl">"Supername"</h1>
                    <SearchForm/>
                </div>
            </main>
        }
    })
}
