use yew::prelude::*;

#[function_component(OverviewNavigator)]
pub fn overview_navigator() -> Html {
    html! {
        <div>
            <div class="flex w-full justify-between">
                <div class="flex">
                    <button class="m-4">{ "<-" }</button>
                    <div class="border-2 m-4"/>
                    <span class="ml-2 mt-4">{ "Overview" }</span>
                    <span class="ml-2 mt-4">{ "project" }</span>
                </div>
                <div>
                    <button class="ml-2 mt-4">{ "Search" }</button>
                    <button class="ml-2 mt-4 mr-2">{ "Bookmark" }</button>
                </div>
            </div>
            <div class="flex justify-between w-full">
                <button class="ml-4">{ "section" }</button>
                <div>
                    <button class="m-2">{ "Export" }</button>
                    <button class="m-2">{ "Template" }</button>
                </div>
            </div>
        </div>
    }
}