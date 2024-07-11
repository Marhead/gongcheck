use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{window, console};
use js_sys::{Reflect, Promise};
use yew_router::prelude::*;
use wasm_bindgen_futures::JsFuture;
use crate::Route;

#[derive(Deserialize)]
struct DirectoryResponse {
    path: Option<String>,
}

fn is_tauri() -> bool {
    let window = match window() {
        Some(win) => win,
        None => return false,
    };

    if let Ok(tauri) = js_sys::Reflect::get(&window, &"__TAURI__".into()) {
        if let Ok(tauri_obj) = tauri.dyn_into::<js_sys::Object>() {
            if let Ok(invoke) = js_sys::Reflect::get(&tauri_obj, &"invoke".into()) {
                return invoke.is_function();
            }
        }
    }

    false
}

async fn select_directory_tauri() -> Result<Option<String>, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("Window object is not available"))?;
    
    if !Reflect::has(&window, &"showDirectoryPicker".into())? {
        return Err(JsValue::from_str("showDirectoryPicker is not supported in this environment"));
    }

    let picker = Reflect::get(&window, &"showDirectoryPicker".into())?
        .dyn_into::<js_sys::Function>()?;

    let promise = picker.call0(&JsValue::NULL)?
        .dyn_into::<Promise>()?;

    let handle = JsFuture::from(promise).await?;
    
    let name = Reflect::get(&handle, &"name".into())?;
    
    Ok(Some(name.as_string().unwrap_or_default()))
}

async fn select_directory_web() -> Result<Option<String>, JsValue> {
    console::log_1(&"Entering select_directory_web".into());
    
    let window = window().unwrap();
    
    // Check if showDirectoryPicker is available
    if js_sys::Reflect::has(&window, &"showDirectoryPicker".into())? {
        console::log_1(&"showDirectoryPicker is available".into());
        
        let picker = js_sys::Reflect::get(&window, &"showDirectoryPicker".into())?
            .dyn_into::<js_sys::Function>()?;

        console::log_1(&"Calling showDirectoryPicker".into());
        
        let promise = picker.call0(&JsValue::NULL)?
            .dyn_into::<js_sys::Promise>()?;

        let handle = wasm_bindgen_futures::JsFuture::from(promise).await?;
        
        console::log_1(&"Got directory handle".into());

        let name = js_sys::Reflect::get(&handle, &"name".into())?;
        
        console::log_2(&"Selected directory:".into(), &name);
        
        Ok(Some(name.as_string().unwrap_or_default()))
    } else {
        console::log_1(&"showDirectoryPicker is not available".into());
        Err(JsValue::from_str("showDirectoryPicker is not supported in this browser"))
    }
}

async fn select_directory() -> Result<Option<String>, JsValue> {
    console::log_1(&"Initiate select_directory method".into());
    if is_tauri() {
        console::log_1(&"Using Tauri".into());
        select_directory_tauri().await
    } else {
        console::log_1(&"Using Web".into());
        select_directory_web().await
    }
}

#[function_component(Welcome)]
pub fn welcome() -> Html {
    let working_dir = use_state(|| None::<String>);
    let error = use_state(|| None::<String>);

    let on_select_directory = {
        let working_dir = working_dir.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let working_dir = working_dir.clone();
            let error = error.clone();
            spawn_local(async move {
                console::log_1(&"Directory selection started".into());
                match select_directory().await {
                    Ok(Some(path)) => {
                        console::log_1(&format!("Directory selected: {}", path).into());
                        working_dir.set(Some(path));
                        error.set(None);
                    }
                    Ok(None) => {
                        console::log_1(&"No directory selected".into());
                        error.set(Some("No directory selected".to_string()));
                    }
                    Err(e) => {
                        console::log_2(&"Error selecting directory:".into(), &e);
                        error.set(Some(format!("Error selecting directory: {:?}", e)));
                    }
                }
            });
        })
    };

    let on_confirm = {
        let working_dir = working_dir.clone();
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            if let Some(dir) = (*working_dir).clone() {
                // Store the selected directory in local storage
                if let Some(window) = window() {
                    if let Ok(Some(local_storage)) = window.local_storage() {
                        let _ = local_storage.set_item("working_directory", &dir);
                    }
                }
                web_sys::console::log_1(&format!("Selected working directory: {}", dir).into());
                
                // Navigate back to the Workspace page
                navigator.push(&Route::Workspace);
            }
        })
    };

    html! {
        <div class="w-screen h-screen flex flex-col justify-center">
            <div class="flex justify-center">
                <div class="w-96 h-64 flex flex-col justify-between">
                    <div>
                        <div class="flex justify-center">{"Welcome to GongCheck"}</div>
                        <div class="flex justify-center">{"Please select a working directory to get started."}</div>
                    </div>
                    <button onclick={on_select_directory} class="border-4 border-rose-500">{"Select Directory"}</button>
                    {
                        if let Some(err) = (*error).clone() {
                            html! {
                                <p class="text-red-500">{err}</p>
                            }
                        } else {
                            html! {}
                        }
                    }
                    {
                        if let Some(dir) = (*working_dir).clone() {
                            html! {
                                <div class="flex flex-col justify-center">
                                    <div class="flex justify-center">{"Selected directory: "}{dir}</div>
                                    <button onclick={on_confirm}>{"Confirm"}</button>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </div>
        </div>
    }
}