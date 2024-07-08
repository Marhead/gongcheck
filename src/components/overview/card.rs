use yew::prelude::*;

#[function_component(OverviewCard)]
pub fn overview_card() -> Html {
    html! {
        <div>
            <h1>{ "Overview" }</h1>
            <p>{ "This is the overview page" }</p>
        </div>
    }
}