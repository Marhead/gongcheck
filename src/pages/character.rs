use yew::prelude::*;

#[function_component(Character)]
pub fn character() -> Html {
    html! {
        <div>
            <h1>{ "Character" }</h1>
            <p>{ "This is the character page" }</p>
        </div>
    }
}