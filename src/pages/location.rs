use yew::prelude::*;

#[function_component(Location)]
pub fn location() -> Html {
    html! {
        <div>
            <h1>{ "Location" }</h1>
            <p>{ "This is the location page" }</p>
        </div>
    }
}