use yew:: prelude::*;

#[function_component(Organization)]
pub fn organization() -> Html {
    html! {
        <div>
            <h1>{ "Organization" }</h1>
            <p>{ "This is the organization page" }</p>
        </div>
    }
}