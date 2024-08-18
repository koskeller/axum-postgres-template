use leptos::*;
use tailwind_fuse::*;

#[derive(Clone, Copy)]
pub enum AlertVariant {
    Default,
    Destructive,
}

impl AlertVariant {
    fn as_class(self) -> &'static str {
        match self {
            AlertVariant::Default => "bg-background text-foreground",
            AlertVariant::Destructive => {
                "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive"
            }
        }
    }
}

#[component]
pub fn Alert(
    #[prop(optional, default = AlertVariant::Default)] variant: AlertVariant,
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
        "relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground",
        variant.as_class(),
        class,
    );
    view! {
        <div role="alert" {..attributes} class=class>
            {children()}
        </div>
    }
}
#[component]
pub fn AlertTitle(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("mb-1 font-medium leading-none tracking-tight", class);
    view! {
        <h5 {..attributes} class=class>
            {children()}
        </h5>
    }
}

#[component]
pub fn AlertDescription(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("text-sm [&_p]:leading-relaxed", class);
    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

