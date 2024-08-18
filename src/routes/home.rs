use axum::{extract::State, response::Html};
use leptos::*;

use crate::{
    ui::{
        Button, ButtonVariant, Card, CardContent, CardDescription, CardFooter, CardHeader,
        CardTitle,
    },
    AppState,
};

pub async fn page(State(_state): State<AppState>) -> Html<String> {
    crate::components::render(move || {
        view! {
            <Card class="mt-100">
                <CardHeader>
                    <CardTitle>"Hello, world!"</CardTitle>
                    <CardDescription>"This is a card component."</CardDescription>
                </CardHeader>
                <CardContent>"Hello, world!"</CardContent>
                <CardFooter>
                    <Button>"Hello"</Button>
                </CardFooter>
            </Card>

            <div class="space-y-2">
            <Button>"Hello"</Button>
            <Button variant=ButtonVariant::Link>"Hello"</Button>
            <Button variant=ButtonVariant::Ghost>"Hello"</Button>
            <Button variant=ButtonVariant::Outline>"Hello"</Button>
            <Button variant=ButtonVariant::Secondary>"Hello"</Button>
            <Button variant=ButtonVariant::Destructive>"Hello"</Button>
            </div>
        }
    })
}
