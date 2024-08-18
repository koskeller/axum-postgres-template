use leptos::*;
use tailwind_fuse::*;

/// Usage:
/// ``` html
/// <div class="flex items-center">
///  <Radion attr:id="email" attr:name="method" attr:checked=true>
///  <Label attr:for="email">"Email"</label>
/// </div>
/// ```
#[component]
pub fn Radio(
    #[prop(optional)] class: &'static str,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = tw_merge!(
        "h-4 w-4 border-primary text-primary focus:ring-primary  disabled:cursor-not-allowed disabled:opacity-50",
        class,
    );
    view! { <input type="radio" {..attributes} class=class/> }
}

