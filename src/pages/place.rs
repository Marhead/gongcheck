use yew::prelude::*;

#[function_component(Place)]
pub fn place() -> Html {
    html! {
        <div>
            <h1>{ "Place" }</h1>
            <p>{ "This is the location page" }</p>
        </div>
    }
}