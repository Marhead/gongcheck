pub mod pages;
pub mod components;
pub mod utils;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::pages::welcome::DirectoryStore;

use pages::welcome::Welcome;
use pages::workspace::Workspace;
use pages::Route;

#[wasm_bindgen]
extern "C" {
    pub type FileSystemDirectoryHandle;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    fn select_directory() -> JsValue;
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeInitFile, catch)]
    pub async fn create_init_file(folder_name: JsValue) -> Result<JsValue, JsValue>;
}

// Yewdux is only working with function components.
// So make "Wrapper" for class components and return it.
#[function_component(WorkspaceWrapper)]
fn workspace_wrapper() -> Html {
    let (store, _) = use_store::<DirectoryStore>();
    
    let root_path = if store.path.starts_with("/") {
        store.path.split("/").last().unwrap_or("").to_string()
    } else {
        store.path.clone()
    };

    html! {
        <Workspace root_path={root_path} />
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Welcome => html! { <Welcome /> },
        Route::Workspace => html! { <WorkspaceWrapper /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
