use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! { 
        <div class="row header">
            <div class="header left">
                <div class="title">{"GongCheck"}</div>
            </div>
            <div class="header right">
                <div class="create">{"New Note"}</div>
                <div class="login">{"Log in"}</div>
            </div>
        </div>
    }
}