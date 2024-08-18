use leptos::*;

use crate::ui::{Button, Checkbox, Input, Label, Radio, Select, SelectOption};

#[component]
pub fn SearchForm() -> impl IntoView {
    let extensions = vec!["com", "net", "org"];
    let extensions = extensions
        .into_iter()
        .map(|n| {
            view! {
                <div class="flex items-center space-x-2">
                    <Checkbox attr:name="ext" attr:id=n attr:value=n/>
                    <Label attr:for=n>{n}</Label>
                </div>
            }
        })
        .collect_view();
    view! {
        <form method="get" action="/" class="space-y-5">
            <div class="relative w-full">
                <Input
                    attr:id="keywords"
                    attr:name="keywords"
                    class="focus:border-transparent pr-10 text-base md:text-sm"
                    attr:placeholder="Enter space separated names"
                    attr:autofocus=true
                />
            </div>

            <div class="flex flex-wrap gap-2">
                <div class="flex items-center space-x-2">
                    <Checkbox attr:name="add-prefixes" attr:id="add-prefixes"/>
                    <Label attr:for="add-prefixes">"Add prefixes"</Label>
                </div>

                <div class="flex items-center space-x-2">
                    <Checkbox attr:name="add-suffixes" attr:id="add-suffixes"/>
                    <Label attr:for="add-suffixes">"Add suffixes"</Label>
                </div>
            </div>

            <div class="flex items-center flex-wrap gap-2">{extensions}</div>

            <Select attr:id="select" attr:name="select">
                <SelectOption text="All" value="all"/>
                <SelectOption text="Starts with" value="starts_with"/>
                <SelectOption text="Ends with" value="ends_with"/>
                <SelectOption text="Contains" value="contains"/>
            </Select>

            <div class="flex items-center">
                <Radio attr:id="sms" attr:name="method" attr:value="sms"/>
                <Label attr:for="sms" class="ml-2">
                    "SMS"
                </Label>
            </div>

            <div class="flex items-center">
                <Radio attr:id="email" attr:name="method" attr:value="email" attr:checked=true/>
                <Label attr:for="email" class="ml-2">
                    "Email"
                </Label>
            </div>

            <Button attr:type="submit" class="w-full sm:w-[120px]">
                "Search"
            </Button>
        </form>
    }
}

