use yew::prelude::*;

#[function_component(Overview)]
pub fn overview() -> Html {
    html! {
        <div>
            <h1>{ "Overview" }</h1>
            <p>{ "This is the overview page" }</p>
        </div>
    }
}