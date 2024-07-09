use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OverviewCardProps {
    pub title: String,
}

#[function_component(OverviewCard)]
pub fn overview_card(props: &OverviewCardProps) -> Html {
    html! {
        <div class="flex flex-col m-2 p-2 border-2">
            <div class="flex justify-between">
                <h1 class="m-2">{ props.title.clone() }</h1>
                <div>
                    <button class="m-2">{ "Grid" }</button>
                    <button class="m-2">{ "List"} </button>
                    <button class="m-2">{ "+" }</button>
                </div>
            </div>
            <div class="border-1">
                { "This is the overview page" }
            </div>
        </div>
    }
}