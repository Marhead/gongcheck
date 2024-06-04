pub mod pages;
pub mod components;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use pages::workspace::Workspace;
use components::titlebar::TitleBar;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn listen(event: &str, handler: &Closure<dyn FnMut(String)>) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component]
pub fn HelloWorld() -> Html {
    html! { "Hello world" }
}

#[function_component(App)]
pub fn app() -> Html {
    // let greet_input_ref = use_node_ref();

    // let name = use_state(|| String::new());

    // let greet_msg = use_state(|| String::new());
    // {
    //     let greet_msg = greet_msg.clone();
    //     let name = name.clone();
    //     let name2 = name.clone();
    //     use_effect_with(
    //         name2,
    //         move |_| {
    //             spawn_local(async move {
    //                 if name.is_empty() {
    //                     return;
    //                 }

    //                 let args = to_value(&GreetArgs { name: &*name }).unwrap();
    //                 // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //                 let new_msg = invoke("greet", args).await.as_string().unwrap();
    //                 greet_msg.set(new_msg);
    //             });

    //             || {}
    //         },
    //     );
    // }

    // let greet = {
    //     let name = name.clone();
    //     let greet_input_ref = greet_input_ref.clone();
    //     Callback::from(move |e: SubmitEvent| {
    //         e.prevent_default();
    //         name.set(
    //             greet_input_ref
    //                 .cast::<web_sys::HtmlInputElement>()
    //                 .unwrap()
    //                 .value(),
    //         );
    //     })
    // };

    let editor_state = use_state(|| "".to_string());

    {
        let editor_state = editor_state.clone();
        use_effect_with_deps(move |_| {
            let editor_state = editor_state.clone();
            let closure = Closure::wrap(Box::new(move |payload: String| {
                let action = payload.split(':').next().unwrap();
                match action {
                    "new" => editor_state.set("".to_string()),
                    "undo" => { /* Handle undo in editor */ },
                    "redo" => { /* Handle redo in editor */ },
                    _ => {}
                }
            }) as Box<dyn FnMut(String)>);

            let _ = listen("tauri://menu", &closure);
            closure.forget();

            || {}
        }, ());
    }

    html! {
        <div>
            <TitleBar />
            <Workspace />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
