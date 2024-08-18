use leptos::*;
use tailwind_fuse::*;

#[derive(Clone, Copy)]
pub enum BadgeVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
}

impl BadgeVariant {
    fn as_class(self) -> &'static str {
        match self {
            BadgeVariant::Default => "border-transparent bg-primary text-primary-foreground hover:bg-primary/80",
            BadgeVariant::Secondary => "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80",
            BadgeVariant::Destructive => "border-transparent bg-destructive text-destructive-foreground hover:bg-destructive/80",
            BadgeVariant::Outline => "text-foreground",
        }
    }
}

#[component]
pub fn Badge(
    #[prop(optional, default = BadgeVariant::Default)] variant: BadgeVariant,
    #[prop(optional)] class: &'static str,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
        "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2",
        variant.as_class(),
        class,
    );
    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}
