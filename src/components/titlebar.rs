use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"])]
    fn close();

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"])]
    fn minimize();

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"])]
    fn maximize();
}

#[function_component(TitleBar)]
pub fn title_bar() -> Html {
    let on_close = Callback::from(|_| close());
    let on_minimize = Callback::from(|_| minimize());
    let on_maximize = Callback::from(|_| maximize());

    html! {
        <div class="flex justify-between bg-blue-500 p-4">
            <div>
                <span>{ "GongCheck" }</span>
            </div>
            <div>
                <button class="hover:bg-blue-700 active:bg-blue-900" onclick={on_minimize}>{ "-" }</button>
                <button class="hover:bg-blue-700 active:bg-blue-900" onclick={on_maximize}>{ "[]" }</button>
                <button class="hover:bg-blue-700 active:bg-blue-900" onclick={on_close}>{ "X" }</button>
            </div>
        </div>
    }
}