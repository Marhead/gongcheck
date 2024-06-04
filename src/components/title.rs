use yew::prelude::*;

#[function_component(Title)]
pub fn title() -> Html {
    html! { 
        <div class="flex mb-4">
            <input type="text" class="bg-wookd-100 flex-1" placeholder="Title"/>
            <button class="ml-2 p-2 bg-wookd-300 text-white rounded">{"New Note"}</button>
        </div>
    }
}