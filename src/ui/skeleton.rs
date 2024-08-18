use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Skeleton(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = tw_merge!("animate-pulse rounded-md bg-muted", class);
    view! {
        <div {..attributes} class=class />
    }
}
