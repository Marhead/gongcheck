use yew::prelude::*;

#[function_component(Note)]
pub fn note() -> Html {
    html! {
        <div>
            <h1>{ "Note" }</h1>
            <p>{ "This is the note page" }</p>
        </div>
    }
}