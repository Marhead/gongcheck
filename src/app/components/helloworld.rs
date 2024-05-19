use yew::prelude::*;

#[function_component(HelloWorldTag)]
pub fn helloworld() -> Html {
    html! { 
        <p>{"Hello world"}</p> 
    }
}