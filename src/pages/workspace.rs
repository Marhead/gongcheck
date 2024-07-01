use crate::components::{Title, MarkdownEditor, Sidebar, SidebarView};

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use wasm_bindgen::prelude::*;


#[function_component(Workspace)]
pub fn workspace() -> Html{
    
    let filenames = vec![
        "Note1.md".to_string(),
        "Note2.md".to_string(),
        "Note3.md".to_string(),
    ];

    // Ensure use_state is declared here
    let sidebar_view = use_state(|| SidebarView::Home);

    // let create_file = Callback::from(move |_| {
    //     let window = web_sys::window().expect("no global `window` exists");
    //     let tauri = window.get("tauri").expect("tauri not found");
    //     let command = JsValue::from_serde(&MyCommand { filename: "untitled.md".to_string() }).expect("failed to serialize command");
    //     tauri.call_method_with_arguments("invoke", &[JsValue::from_str("create_file"), command]).expect("failed to invoke command");
    // });

    html!{
        <div class="flex h-screen">
            <Sidebar filenames={filenames} view={sidebar_view.clone()}/>
            <main class="flex-1 bg-wookd-100 p-4">
                <Title />
                <MarkdownEditor />
            </main>
        </div>
    }
}