use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Checkbox(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    #[prop(optional)] class: &'static str,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = tw_merge!(
        "peer h-4 w-4 shrink-0 rounded-sm border border-primary ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground",
        class,
    );
    view! {
        <div class="flex items-center space-x-2">
            <input id=id.clone() type="checkbox" {..attributes} class=class />
            <label
            for=id
            className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            >
                {label}
            </label>
        </div>
    }
}
