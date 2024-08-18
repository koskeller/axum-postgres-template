use leptos::*;

use crate::metadata::Metadata;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let metadata = Metadata::default();
    view! {
        <html lang="en">
            <head>
                <meta charset="UTF-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <title>{metadata.title}</title>
                <meta name="description" content=metadata.description/>
                <link rel="canonical" href=metadata.base_url/>
                <link href="/public/styles.css" rel="stylesheet"/>
            </head>
            <body>
                <main>
                    <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                        <div class="space-y-6 py-6 pb-16 md:block">{children()}</div>
                    </div>
                </main>
            </body>
        </html>
    }
}

