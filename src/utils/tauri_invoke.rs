use wasm_bindgen::{JsValue, JsCast};
use js_sys::{Object, Function, Promise, Reflect};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {

    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = invoke)]
    pub async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// pub async fn invoke_tauri_command_async<T: Serialize>(cmd: &str, args: &T) -> Result<JsValue, JsValue> {
//     let window = window().ok_or_else(|| JsValue::from_str("No window object found"))?;
//     let tauri = Reflect::get(&window, &"__TAURI__".into())?;
//     let tauri_obj = tauri.dyn_into::<Object>()?;
//     let invoke = Reflect::get(&tauri_obj, &"invoke".into())?;
//     let invoke_fn = invoke.dyn_into::<Function>().map_err(|_| "Failed to cast to Function".to_string())?;

//     let args_js = serde_wasm_bindgen::to_value(args).map_err(|e| e.to_string())?;

//     let promise = invoke_fn.call2(
//         &JsValue::NULL,
//         &cmd.into(),
//         &args_js,
//     )?;

//     let promise = promise.dyn_into::<Promise>().map_err(|_| "Failed to cast to Promise".to_string())?;
//     JsFuture::from(promise).await
// }

// pub fn invoke_tauri_command<T: Serialize>(cmd: &str, args: &T) -> Result<Promise, JsValue> {
//     web_sys::console::log_1(&"Entering invoke_tauri_command".into());
//     web_sys::console::log_1(&cmd.into());
//     web_sys::console::log_1(&serde_wasm_bindgen::to_value(args).map_err(|e| e.to_string())?.into());

//     let window = window().ok_or_else(|| JsValue::from_str("No window object found"))?;
//     let tauri = Reflect::get(&window, &"__TAURI__".into())?;
//     let tauri_obj = tauri.dyn_into::<Object>()?;
//     let invoke = Reflect::get(&tauri_obj, &"invoke".into())?;
//     let invoke_fn = invoke.dyn_into::<Function>().map_err(|_| "Failed to cast to Function".to_string())?;

//     // let args_js = serde_wasm_bindgen::to_value(args).map_err(|e| e.to_string())?;

//     // let promise = invoke_fn.call2(
//     //     &JsValue::NULL,
//     //     &cmd.into(),
//     //     &args_js,
//     // )?;

//     // // let promise = promise.dyn_into::<Promise>().map_err(|_| "Failed to cast to Promise".to_string())?;
//     // Ok(promise.dyn_into::<Promise>().map_err(|_| JsValue::from_str("Failed to cast to Promise"))?)
    
//     // Wrap the arguments in an object with the command name as key
//     let args_obj = Object::new();
//     Reflect::set(&args_obj, &cmd.into(), &serde_wasm_bindgen::to_value(args).map_err(|e| e.to_string())?.into())?;

//     let promise = invoke_fn.call2(
//         &JsValue::NULL,
//         &cmd.into(),
//         &args_obj
//     )?;

//     Ok(promise.dyn_into::<Promise>().map_err(|_| JsValue::from_str("Failed to cast to Promise"))?)
// }