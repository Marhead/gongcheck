use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"])]
    fn close();

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"])]
    fn minimize();

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"])]
    fn maximize();
}

#[function_component(TitleBar)]
pub fn title_bar() -> Html {
    let on_close = Callback::from(|_| close());
    let on_minimize = Callback::from(|_| minimize());
    let on_maximize = Callback::from(|_| maximize());

    html! {
        <div class="titlebar">
            <span>{ "My Markdown Editor" }</span>
            <div class="titlebar-buttons">
                <button onclick={on_minimize}>{ "-" }</button>
                <button onclick={on_maximize}>{ "[]" }</button>
                <button onclick={on_close}>{ "X" }</button>
            </div>
        </div>
    }
}

// use yew::prelude::*;
// use tauri::window::Window;

// #[function_component]
// pub fn titlebar() -> Html {

//     let handle_minimize = Callback::from(|_| {
//         let window = Window::current();
//         wasm_bindgen_futures::spawn_local(async move {
//             window.minimize().await.unwrap();
//         });
//     });

//     let handle_maximize = Callback::from(|_| {
//         let window = Window::current();
//         wasm_bindgen_futures::spawn_local(async move {
//             window.toggle_maximize().await.unwrap();
//         });
//     });

//     let handle_close = Callback::from(|_| {
//         let window = Window::current();
//         wasm_bindgen_futures::spawn_local(async move {
//             window.close().await.unwrap();
//         });
//     });

//     html!{
//         <div class="flex justify-between items-center bg-gray-800 text-white p-2">
//             <div class="text-lg font-bold">
//                 {"Gongcheck"}
//             </div>
//             <div class="flex space-x-2">
//                 <button onclick={handle_minimize} class="p-2 hover:bg-gray-700 rounded">
//                     {"_"}
//                 </button>
//                 <button onclick={handle_maximize} class="p-2 hover:bg-gray-700 rounded">
//                     {"[ ]"}
//                 </button>
//                 <button onclick={handle_close} class="p-2 hover:bg-red-700 rounded">
//                     {"X"}
//                 </button>
//             </div>
//         </div>
//     }
// }