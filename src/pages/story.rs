use yew::prelude::*;

#[function_component(Story)]
pub fn story() -> Html {
    html! {
        <div>
            <h1>{ "Story" }</h1>
            <p>{ "This is the story page" }</p>
        </div>
    }
}