use wasm_bindgen::{JsValue, JsCast};
use web_sys::window;
use js_sys::{Object, Function, Promise, Reflect};
use serde::Serialize;
use wasm_bindgen_futures::JsFuture;

pub async fn invoke_tauri_command<T: Serialize>(cmd: &str, args: &T) -> Result<JsValue, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("No window object found"))?;
    let tauri = Reflect::get(&window, &"__TAURI__".into())?;
    let tauri_obj = tauri.dyn_into::<Object>()?;
    let invoke = Reflect::get(&tauri_obj, &"invoke".into())?;
    let invoke_fn = invoke.dyn_into::<Function>().map_err(|_| "Failed to cast to Function".to_string())?;

    let args_js = serde_wasm_bindgen::to_value(args).map_err(|e| e.to_string())?;

    let promise = invoke_fn.call2(
        &JsValue::NULL,
        &cmd.into(),
        &args_js,
    )?;

    let promise = promise.dyn_into::<Promise>().map_err(|_| "Failed to cast to Promise".to_string())?;
    JsFuture::from(promise).await
}