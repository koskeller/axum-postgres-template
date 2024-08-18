use axum::{extract::State, response::Html};
use leptos::*;

use crate::{
    ui::{
        Button, ButtonVariant, Card, CardContent, CardDescription, CardFooter, CardHeader,
        CardTitle, Checkbox, Pagination, PaginationContent, PaginationEllipsis, PaginationItem,
        PaginationLink, PaginationNext, PaginationPrevious, Skeleton,
    },
    AppState,
};

pub async fn page(State(_state): State<AppState>) -> Html<String> {
    crate::components::render(move || -> Fragment {
        view! {
            <Card class="mt-100">
                <CardHeader>
                    <CardTitle>"Hello, world!"</CardTitle>
                    <CardDescription>"This is a card component."</CardDescription>
                </CardHeader>
                <CardContent>
                    <Checkbox id="1" label="Accept terms" />
                </CardContent>
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

            <div class="flex flex-col space-y-3">
              <Skeleton class="h-[125px] w-[250px] rounded-xl" />
              <div class="space-y-2">
                <Skeleton class="h-4 w-[250px]" />
                <Skeleton class="h-4 w-[200px]" />
              </div>
            </div>

            <div>
                <Pagination>
                <PaginationContent>
                    <PaginationItem>
                    <PaginationPrevious attr:href="#" />
                    </PaginationItem>
                    <PaginationItem>
                    <PaginationLink attr:href="#">1</PaginationLink>
                    </PaginationItem>
                    <PaginationItem>
                    <PaginationEllipsis />
                    </PaginationItem>
                    <PaginationItem>
                    <PaginationNext attr:href="#" />
                    </PaginationItem>
                </PaginationContent>
                </Pagination>
            </div>

        }
    })
}
