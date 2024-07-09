pub mod pages;
pub mod components;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use pages::*;

#[wasm_bindgen]
extern "C" {
    pub type FileSystemDirectoryHandle;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    fn select_directory() -> JsValue;
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Welcome => html! { <Welcome /> },
        Route::Workspace => html! { <Workspace root_path={"NewWorld"} /> },
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
