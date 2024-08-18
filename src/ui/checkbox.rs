use leptos::*;
use tailwind_fuse::*;

/// Usage:
/// ``` html
/// <div class="flex items-center space-x-2">
///     <Checkbox attr:name="add-prefixes" attr:id="add-prefixes"/>
///     <Label attr:for="add-prefixes">"Add prefixes"</Label>
/// </div>
/// ```
#[component]
pub fn Checkbox(
    #[prop(optional)] class: &'static str,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = tw_merge!(
        "h-4 w-4 rounded border-primary text-primary focus:ring-primary  disabled:cursor-not-allowed disabled:opacity-50",
        class,
    );
    view! { <input type="checkbox" {..attributes} class=class/> }
}

