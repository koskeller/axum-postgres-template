use leptos::*;
use tailwind_fuse::*;

use crate::ui::{IconChevronLeft, IconChevronRight, IconEllipsis};

#[component]
pub fn Pagination(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("mx-auto flex w-full justify-center", class);
    view! {
        <nav role="navigation" aria-label="pagination" {..attributes} class=class>
            {children()}
        </nav>
    }
}

#[component]
pub fn PaginationContent(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("flex flex-row items-center gap-1", class);
    view! {
        <ul {..attributes} class=class>
            {children()}
        </ul>
    }
}

#[component]
pub fn PaginationItem(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    view! {
        <li {..attributes} class=class>
            {children()}
        </li>
    }
}

#[component]
pub fn PaginationLink(
    #[prop(optional, into)] class: String,
    #[prop(optional)] is_active: bool,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let variant = if is_active {
        "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
    } else {
        "hover:bg-accent hover:text-accent-foreground"
    };

    let class = tw_merge!(
        "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
        "h-10 px-4 py-2",
        variant,
        class
    );

    view! {
        <a {..attributes} class=class>
            {children()}
        </a>
    }
}

#[component]
pub fn PaginationPrevious(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = tw_merge!("gap-1 pl-2.5", class);
    view! {
        <PaginationLink attr:aria-label="Go to previous page" {..attributes} class=class>
            <IconChevronLeft class="h-4 w-4"/>
            <span>"Previous"</span>
        </PaginationLink>
    }
}

#[component]
pub fn PaginationNext(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = tw_merge!("gap-1 pr-2.5", class);
    view! {
        <PaginationLink attr:aria-label="Go to next page" {..attributes} class=class>
            <span>"Next"</span>
            <IconChevronRight class="h-4 w-4"/>
        </PaginationLink>
    }
}

#[component]
pub fn PaginationEllipsis(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <span
            aria-hidden=true
            class=tw_merge!("flex h-9 w-9 items-center justify-center", class)
            {..attributes}
        >
            <IconEllipsis class="h-4 w-4"/>
            <span class="sr-only">"More pages"</span>
        </span>
    }
}
