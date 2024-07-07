use yew::prelude::*;
use crate::components::{SidebarView};

// #[derive(Properties, PartialEq)]
// pub struct SidebarProps {
//     pub filenames: Vec<String>,
//     pub view: UseStateHandle<SidebarView>,
// }

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let labels = vec![
        "Overview",
        "Stories",
        "Notes",
        "Characters",
        "Organizations",
        "Cultures",
        "Species",
        "Locations",
        "Discoveries",
        "Relations",
        "Items",
        "Images"];

    let buttons = labels.iter().enumerate().map(|(i, label)| {
        let cloned_label = label.to_string(); // Clone the label variable
        html! {
            <button onclick={Callback::from(move |_| {
                web_sys::console::log_1(&format!("Clicked {}", cloned_label).into());
            })} class="p-2">
                { label }
            </button>
        }
    });

    html! {
        <div class="flex flex-col w-48">
            <div class="p-4 text-center text-2xl font-bold">{ "WorldName" }</div>
            { for buttons }
        </div>
    }
}