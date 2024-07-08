use yew::prelude::*;

#[function_component(Image)]
pub fn image() -> Html {
    html! {
        <div>
            <h1>{ "Image" }</h1>
            <p>{ "This is the image page" }</p>
        </div>
    }
}