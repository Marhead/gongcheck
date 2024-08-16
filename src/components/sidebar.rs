use std::path::Path;
use yew::prelude::*;
use web_sys::console;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub root_path: String,
    pub labels: Vec<String>,
    pub sidebar_load: Callback<String>
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let buttons = props.labels.iter().enumerate().map(|(_i, label)| {
        let cloned_label = label.clone(); // Use clone here
        let sidebar_load = props.sidebar_load.clone();
        html! {
            <button onclick={Callback::from(move |_| {
                sidebar_load.emit(cloned_label.clone());
            })} class="p-2">
                { label }
            </button>
        }
    });

    console::log_1(&format!("Sidebar loaded: {}", props.root_path).into());

    let project_name = props.root_path
        .replace('\\', "/")  // Replace backslashes with forward slashes
        .split('/')
        .last()
        .unwrap_or("Default")
        .to_string();

    console::log_1(&format!("Project name: {}", project_name).into());

    html! {
        <div class="flex flex-col w-48">
            <div class="p-4 text-center text-2xl font-bold">{ project_name }</div>
            { for buttons }
        </div>
    }
}