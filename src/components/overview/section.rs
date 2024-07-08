use yew::prelude::*;

#[function_component(Section)]
pub fn section() -> Html {
    html! {
        <div>
            <h1>{ "Section" }</h1>
            <p>{ "This is the section page" }</p>
        </div>
    }
}