use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod components;
use components::helloworld::HelloWorldTag;
use components::header::Header;
use crate::components::editor::Editor;
// use components::markdown_editor::MarkdownEditor;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
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
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with(
            name2,
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let args = to_value(&GreetArgs { name: &*name }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("greet", args).await.as_string().unwrap();
                    greet_msg.set(new_msg);
                });

                || {}
            },
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <div class="flex h-screen">
            <aside class="w-1/4 bg-gray-900 text-white p-4">
                <h1 class="text-2xl font-bold mb-4">{"Obsidian-like Editor"}</h1>
                <nav>
                    <ul>
                        <li class="mb-2"><a href="#" class="text-gray-300 hover:text-white">{"Home"}</a></li>
                        <li class="mb-2"><a href="#" class="text-gray-300 hover:text-white">{"Notes"}</a></li>
                        <li class="mb-2"><a href="#" class="text-gray-300 hover:text-white">{"Settings"}</a></li>
                    </ul>
                </nav>
            </aside>
            <main class="flex-1 bg-gray-100 p-4">
                <div class="flex mb-4">
                    <input type="text" class="flex-1 p-2 border border-gray-300 rounded" placeholder="Search..."/>
                    <button class="ml-2 p-2 bg-blue-500 text-white rounded">{"New Note"}</button>
                </div>
                <div class="bg-white p-4 border border-gray-300 rounded">
                    <textarea class="w-full h-full border-none focus:outline-none" placeholder="Write your note here..."></textarea>
                </div>
            </main>
        </div>
    }
}


fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
