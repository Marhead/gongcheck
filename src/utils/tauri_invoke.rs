use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = invoke)]
    pub async fn invoke_without_args(cmd: &str) -> JsValue;

    // invoke with arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}