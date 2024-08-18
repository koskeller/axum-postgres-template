use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Table(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("w-full caption-bottom text-sm", class);
    view! {
        <div class="relative w-full overflow-auto">
            <table {..attributes} class=class>
                {children()}
            </table>
        </div>
    }
}

#[component]
pub fn TableHeader(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("[&_tr]:border-b", class);
    view! {
        <thead {..attributes} class=class>
            {children()}
        </thead>
    }
}

#[component]
pub fn TableBody(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("[&_tr:last-child]:border-0", class);
    view! {
        <tbody {..attributes} class=class>
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableFooter(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
        "border-t bg-muted/50 font-medium [&>tr]:last:border-b-0",
        class
    );
    view! {
        <tfoot {..attributes} class=class>
            {children()}
        </tfoot>
    }
}

#[component]
pub fn TableRow(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!(
        "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted",
        class
    );
    view! {
        <tr {..attributes} class=class>
            {children()}
        </tr>
    }
}

#[component]
pub fn TableHead(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("h-12 px-4 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0", class);
    view! {
        <th {..attributes} class=class>
            {children()}
        </th>
    }
}

#[component]
pub fn TableCell(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("p-4 align-middle [&:has([role=checkbox])]:pr-0", class);
    view! {
        <td {..attributes} class=class>
            {children()}
        </td>
    }
}

#[component]
pub fn TableCaption(
    #[prop(optional, into)] class: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = tw_merge!("mt-4 text-sm text-muted-foreground", class);
    view! {
        <caption {..attributes} class=class>
            {children()}
        </caption>
    }
}
