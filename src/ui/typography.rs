use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn TypographyH1(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
        "scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl",
        class,
    );
    view! {
        <h1 {..attributes} class=class>{children()}</h1>
    }
}

#[component]
pub fn TypographyH2(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
        "scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0",
        class,
    );
    view! {
        <h2 {..attributes} class=class>{children()}</h2>
    }
}

#[component]
pub fn TypographyH3(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("scroll-m-20 text-2xl font-semibold tracking-tight", class);
    view! {
        <h3 {..attributes} class=class>{children()}</h3>
    }
}

#[component]
pub fn TypographyH4(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("scroll-m-20 text-xl font-semibold tracking-tight", class);
    view! {
        <h4 {..attributes} class=class>{children()}</h4>
    }
}

#[component]
pub fn TypographyP(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("leading-7 [&:not(:first-child)]:mt-6", class);
    view! {
        <p {..attributes} class=class>{children()}</p>
    }
}

#[component]
pub fn TypographyBlockquote(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("mt-6 border-l-2 pl-6 italic", class);
    view! {
        <blockquote {..attributes} class=class>{children()}</blockquote>
    }
}

#[component]
pub fn TypographyList(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("my-6 ml-6 list-disc [&>li]:mt-2", class);
    view! {
        <ul {..attributes} class=class>{children()}</ul>
    }
}

#[component]
pub fn TypographyInlineCode(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
        "relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold",
        class,
    );
    view! {
        <code {..attributes} class=class>{children()}</code>
    }
}

#[component]
pub fn TypographyLead(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("text-xl text-muted-foreground", class);
    view! {
        <p {..attributes} class=class>{children()}</p>
    }
}

#[component]
pub fn TypographyLarge(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("text-lg font-semibold", class);
    view! {
        <div {..attributes} class=class>{children()}</div>
    }
}

#[component]
pub fn TypographySmall(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("text-sm font-medium leading-none", class,);
    view! {
        <small {..attributes} class=class>{children()}</small>
    }
}

#[component]
pub fn TypographyMuted(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("text-sm text-muted-foreground", class,);
    view! {
        <p {..attributes} class=class>{children()}</p>
    }
}
