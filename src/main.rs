pub mod pages;
pub mod components;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use pages::workspace::Workspace;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn listen(event: &str, handler: &Closure<dyn FnMut(String)>) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    // let editor_state = use_state(|| "".to_string());

    // Here is the code that I am trying to customize titlebar.
    // {
    //     let editor_state = editor_state.clone();
    //     use_effect_with((), move |_| {
    //         let editor_state = editor_state.clone();
    //         let closure = Closure::wrap(Box::new(move |payload: String| {
    //             let action = payload.split(':').next().unwrap();
    //             match action {
    //                 "new" => editor_state.set("".to_string()),
    //                 "undo" => { /* Handle undo in editor */ },
    //                 "redo" => { /* Handle redo in editor */ },
    //                 _ => {}
    //             }
    //         }) as Box<dyn FnMut(String)>);
    
    //         let _ = listen("tauri://menu", &closure);
    //         closure.forget();
    
    //         || {}
    //     });
    // }

    html! {
        <div>
            <Workspace />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
