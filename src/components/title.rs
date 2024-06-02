use yew::prelude::*;

#[function_component(Title)]
pub fn title() -> Html {
    html! { 
        <div class="flex mb-4">
            <input type="text" class="flex-1 p-2 border border-gray-300 rounded" placeholder="Search..."/>
            <button class="ml-2 p-2 bg-blue-500 text-white rounded">{"New Note"}</button>
        </div>
    }
}