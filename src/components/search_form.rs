use leptos::*;

use crate::ui::{Button, Checkbox, Input};

#[component]
pub fn SearchForm() -> impl IntoView {
    let extensions = vec!["com", "net", "org"];
    let extensions = extensions
        .into_iter()
        .map(|n| view! { <Checkbox attr:name="ext" attr:value=n id=n label=n/> })
        .collect_view();
    view! {
        <form method="get" action="/" class="space-y-3">
            <div class="relative w-full">
                <Input
                    attr:id="keywords"
                    attr:name="keywords"
                    class="focus:border-transparent pr-10 text-base md:text-sm"
                    attr:placeholder="Enter space separated names"
                    attr:autofocus=true
                />
            </div>

            <div class="flex flex-wrap gap-2">
                <Checkbox attr:name="add-prefixes" id="add-prefixes" label="Add prefixes"/>
                <Checkbox attr:name="add-suffixes" id="add-suffixes" label="Add suffixes"/>
            </div>

            <div className="flex items-center flex-wrap gap-2">{extensions}</div>

            <Button attr:type="submit" class="w-full sm:w-[120px]">
                "Search"
            </Button>
        </form>
    }
}

