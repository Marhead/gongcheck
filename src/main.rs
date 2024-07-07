pub mod pages;
pub mod components;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use pages::*;
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    pub type FileSystemDirectoryHandle;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    fn select_directory() -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    // let is_first = {
    //     let window = window().expect("Should have a window");
    //     let local_storage = window.local_storage().expect("Should have local storage").expect("Should be able to access local storage");
    //     match local_storage.get_item("accessed_before").expect("Should be able to get item") {
    //         None => true,
    //         Some(_) => false,
    //     }
    // };

    // let local_directory_set = {
    //     false
    // };

    // if is_first || !local_directory_set {
    //     let window = window().expect("should have a window");
    //     let local_storage = window.local_storage().expect("should have local storage").expect("should be able to access local storage");
    //     local_storage.set_item("accessed_before", "true").expect("should be able to set item");

    //     html! {
    //         <Welcome />
    //     }
    // } else {
    //     html! {
    //         <Workspace />
    //     }
    // }

    let root_path = std::env::current_dir().unwrap();
    let root_path_str = root_path.to_str().unwrap();

    html! {
        <Workspace root_path={root_path_str} />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
