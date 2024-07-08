use yew::prelude::*;

#[function_component(Item)]
pub fn item() -> Html {
    html! {
        <div>
            <h1>{ "Item" }</h1>
            <p>{ "This is the item page" }</p>
        </div>
    }
}