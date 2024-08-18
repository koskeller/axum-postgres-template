use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Select(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
        "mt-2 block w-full rounded-md border border-input py-1.5 pl-3 pr-10 ring-1 ring-inset focus:border-input focus:ring-2 focus:ring-primary sm:text-sm sm:leading-6",
        class,
    );
    view! {
        <select {..attributes} class=class>
            {children()}
        </select>
    }
}

#[component]
pub fn SelectOption(
    #[prop(optional, into)] text: String,
    #[prop(optional, into)] value: String,
) -> impl IntoView {
    view! { <option value=value>{text}</option> }
}

