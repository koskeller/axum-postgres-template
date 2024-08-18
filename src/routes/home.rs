use axum::{extract::State, response::Html};
use leptos::*;

use crate::{components::search_form::SearchForm, AppState};

pub async fn page(State(_state): State<AppState>) -> Html<String> {
    crate::components::render(move || {
        view! {
            <main>
                <div class="max-w-2xl mx-auto py-8 sm:py-20 md:py-40 px-4 md:px-6">
                    <a href="/">
                        <h1 class="py-3 font-bold italic text-4xl">"Supername"</h1>
                    </a>
                    <SearchForm/>
                    <div>
                        <label
                            for="location"
                            class="block text-sm font-medium leading-6 text-gray-900"
                        >
                            Location
                        </label>
                        <select
                            id="location"
                            name="location"
                            class="mt-2 block w-full rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-indigo-600 sm:text-sm sm:leading-6"
                        >
                            <option>United States</option>
                            <option selected>Canada</option>
                            <option>Mexico</option>
                        </select>
                    </div>
                </div>
            </main>
        }
    })
}

