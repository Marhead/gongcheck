use yew::prelude::*;

#[function_component(HelloWorldTag)]
pub fn helloworld() -> Html {
    html! { 
        <div className="bg-black">{"Hello world"}</div>
    }
}