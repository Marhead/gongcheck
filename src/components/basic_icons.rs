use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use gloo::console::log;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = tauri)]
    fn invoke(command: &str, payload: JsValue) -> Promise;
}

#[function_component(BasicIcons)]
pub fn basic_icons() -> Html {
    let create_file = Callback::from(move |_| {
        let path = "/untitle.md".to_string();
        let _ = invoke("create_file", JsValue::from_str(&path));
        log!("Create file: {}", path);
    });

    html! {
        <div class="flex items">
            <button onclick={create_file}>{"Create File"}</button>
        </div>
    }
}