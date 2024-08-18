use leptos::*;
use tailwind_fuse::*;

#[derive(Clone, Copy)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonVariant {
    fn as_class(self) -> &'static str {
        match self {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Destructive => {
                "bg-destructive text-destructive-foreground hover:bg-destructive/90"
            }
            ButtonVariant::Outline => {
                "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
            }
            ButtonVariant::Secondary => {
                "bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

#[derive(Clone, Copy)]
pub enum ButtonSize {
    Default,
    Small,
    Large,
    Icon,
}

impl ButtonSize {
    fn as_class(self) -> &'static str {
        match self {
            ButtonSize::Default => "h-10 px-4 py-2",
            ButtonSize::Small => "h-9 rounded-md px-3",
            ButtonSize::Large => "h-11 rounded-md px-8",
            ButtonSize::Icon => "h-10 w-10",
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional, default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(optional, default = ButtonSize::Default)] size: ButtonSize,
    #[prop(optional)] class: &'static str,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
        "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
        variant.as_class(),
        size.as_class(),
        class,
    );
    view! {
        <button {..attributes} class=class>
            {children()}
        </button>
    }
}
