use yew::prelude::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

#[derive(Deserialize)]
struct DirectoryResponse {
    path: Option<String>,
}

#[function_component(Welcome)]
pub fn welcome() -> Html {
    let working_dir = use_state(|| None::<String>);

    let on_select_directory = {
        let working_dir = working_dir.clone();
        Callback::from(move |_| {
            let working_dir = working_dir.clone();
            spawn_local(async move {
                let window = web_sys::window().unwrap();
                let result = js_sys::Reflect::get(&window, &"__TAURI__".into())
                    .unwrap()
                    .dyn_into::<js_sys::Object>()
                    .unwrap();
                let invoke = js_sys::Reflect::get(&result, &"invoke".into())
                    .unwrap()
                    .dyn_into::<js_sys::Function>()
                    .unwrap();

                let promise = invoke.call2(
                    &JsValue::NULL,
                    &"select_directory".into(),
                    &JsValue::NULL,
                ).unwrap();

                let response = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                let dir_response: DirectoryResponse = serde_wasm_bindgen::from_value(response).unwrap();

                if let Some(path) = dir_response.path {
                    working_dir.set(Some(path));
                }
            });
        })
    };

    let on_confirm = {
        let working_dir = working_dir.clone();
        Callback::from(move |_| {
            if let Some(dir) = (*working_dir).clone() {
                log::info!("Selected working directory: {}", dir);
                // You might want to navigate to the main app page here
            }
        })
    };

    html! {
        <div class="welcome-page">
            <h1>{"Welcome to GongCheck"}</h1>
            <p>{"Please select a working directory to get started."}</p>
            <button onclick={on_select_directory}>{"Select Directory"}</button>
            {
                if let Some(dir) = (*working_dir).clone() {
                    html! {
                        <>
                            <p>{"Selected directory: "}{dir}</p>
                            <button onclick={on_confirm}>{"Confirm"}</button>
                        </>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}