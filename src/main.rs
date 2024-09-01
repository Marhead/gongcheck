pub mod pages;
pub mod components;
pub mod utils;

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::pages::welcome::DirectoryStore;
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{Event, CustomEvent};

use pages::welcome::Welcome;
use pages::workspace::Workspace;
use pages::Route;

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
    let navigator = use_navigator().unwrap();
    let dispatch = use_dispatch::<DirectoryStore>();

    {
        let navigator = navigator.clone();
        let dispatch = dispatch.clone();

        use_effect(move || {
            let window = web_sys::window().unwrap();

            // Use `Closure::wrap` to create a Rust closure that can be used as a JavaScript callback
            let listener = Closure::wrap(Box::new(move |event: web_sys::Event| {
                let event = event.dyn_ref::<web_sys::CustomEvent>().unwrap();
                let route: String = event.detail().as_string().unwrap_or("welcome".to_string());
                if route != "welcome" {
                    set_path(route.clone(), dispatch.clone());
                    navigator.push(&Route::Workspace);
                } else {
                    navigator.push(&Route::Welcome);
                }
            }) as Box<dyn FnMut(_)>);

            // Add the event listener to the window object
            window
                .add_event_listener_with_callback("set_initial_route", listener.as_ref().unchecked_ref())
                .unwrap();

            // Prevent the closure from being dropped
            listener.forget();

            // Return a cleanup function that removes the event listener (if needed)
            || ()
        });
    }

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
