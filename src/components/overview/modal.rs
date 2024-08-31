use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub on_close: Callback<MouseEvent>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    html! {
        <div class="fixed inset-0 flex items-center justify-center bg-gray-500 bg-opacity-75 backdrop-blur">
            <div class="bg-white p-4 rounded shadow-lg">
                <h2>{ "Modal Title" }</h2>
                <p>{ "This is the modal content." }</p>
                <button onclick={props.on_close.clone()}>{ "Close" }</button>
            </div>
        </div>
    }
}