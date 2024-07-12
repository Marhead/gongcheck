pub mod pages;
pub mod components;

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

// Yewdux is only working with function components.
// So make "Wrapper" for class components and return it.
#[function_component(WorkspaceWrapper)]
fn workspace_wrapper() -> Html {
    let (store, _) = use_store::<DirectoryStore>();
    
    html! {
        <Workspace root_path={store.path.clone()} />
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
