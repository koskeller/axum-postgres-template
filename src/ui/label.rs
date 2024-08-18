use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Label(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
        "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
        class,
    );
    view! {
        <label {..attributes} class=class>
            {children()}
        </label>
    }
}
