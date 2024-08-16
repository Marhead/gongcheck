use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{window, console};
use js_sys::{Reflect, Promise};
use wasm_bindgen_futures::JsFuture;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use crate::utils::tauri_invoke::*;
use crate::Route;


// This struct is assigned in Yewdux store.
#[derive(Default, Clone, PartialEq, Store)]
pub struct DirectoryStore {
    pub path: String,
}


#[derive(Serialize)]
struct CreateInitFileParams {
    folder_name: String,
}

// Yewdux is only working with function components.
// So make "Wrapper" for class components and return it.
// Yewdux storing data method.
pub fn set_path(path: String, dispatch: Dispatch<DirectoryStore>) {
    dispatch.reduce_mut(move |store| {
        store.path = path;
    })
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
    let response = invoke_without_args("select_directory").await;
    
    // Handle the response
    if response.is_undefined() || response.is_null() {
        Err(JsValue::from_str(&format!("Failed to select directory: {:?}", response)))
    } else {
        Ok(Some(response.as_string().unwrap_or_default()))
    }
}


async fn select_directory_web() -> Result<Option<String>, JsValue> {
    console::log_1(&"Entering select_directory_web".into());
    
    let window = window().unwrap();
    
    // Check if showDirectoryPicker is available
    if js_sys::Reflect::has(&window, &"showDirectoryPicker".into())? {        
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
    if is_tauri() {
        select_directory_tauri().await
    } else {
        select_directory_web().await
    }
}

#[function_component(Welcome)]
pub fn welcome() -> Html {
    let working_dir = use_state(|| None::<String>);
    let error = use_state(|| None::<String>);
    let (_store, dispatch) = use_store::<DirectoryStore>();

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
        let dispatch = dispatch.clone();

        Callback::from(move |_| {
            let working_dir = working_dir.clone();
            let navigator = navigator.clone();
            let dispatch = dispatch.clone();
            
            spawn_local(async move {
                if let Some(dir) = (*working_dir).clone() {
                    // Store the selected directory in local storage
                    if let Some(window) = window() {
                        if let Ok(Some(local_storage)) = window.local_storage() {
                            let _ = local_storage.set_item("working_directory", &dir);
                        }
                    }
                    set_path(dir.clone(), dispatch);

                    web_sys::console::log_1(&format!("Selected working directory: {}", dir.clone()).into());

                    let params = CreateInitFileParams { folder_name: dir.clone() };
                    let args = serde_wasm_bindgen::to_value(&params).unwrap();
                    invoke("create_init_file", args).await;

                    // Navigate back to the Workspace page
                    navigator.push(&Route::Workspace);
                }
            });
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