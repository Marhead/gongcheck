use yew::prelude::*;
use crate::components::{SidebarIcons, SidebarPanels, SidebarView};

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub filenames: Vec<String>,
    pub view: UseStateHandle<SidebarView>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    html! {
        <aside class="flex w-1/4 bg-gray-900 text-white p-2">
            <SidebarIcons view={props.view.clone()} />
            <SidebarPanels filenames={props.filenames.clone()} />
        </aside>
    }
}