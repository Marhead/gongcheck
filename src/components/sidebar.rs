use yew::prelude::*;
use crate::components::{SidebarIcons, SidebarPanels, SidebarView};

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub filenames: Vec<String>,
    pub view: UseStateHandle<SidebarView>,
}

// #[derive(Clone, PartialEq)]
// pub enum SidebarView {
//     Home,
//     Notes,
//     Settings,
//     File,
//     User
//     // Add more views as needed
// }

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {

    // let view = props.view.clone();
    
    // let change_view = move |new_view: SidebarView| {
    //     let view = view.clone();
    //     Callback::from(move |_| view.set(new_view.clone()))
    // };

    html! {
        <aside class="flex w-1/4 bg-gray-900 text-white p-2">
            // <div class="flex-col">
            //     <div class="pb-8">
            //         <button onclick={change_view(SidebarView::Home)}>
            //             <img src="public/icons/icons8-home-24-white.svg" class="w-12 h-12" alt="Home"/>
            //         </button>
            //     </div>
            //     <div>
            //         <button onclick={change_view(SidebarView::Notes)}>
            //             <img src="public/icons/icons8-home-24-white.svg" class="w-12 h-12" alt="Notes" />
            //         </button>
            //     </div>
            //     <div>
            //         <button onclick={change_view(SidebarView::Settings)}>
            //             <img src="public/icons/icons8-home-24-white.svg" class="w-12 h-12" alt="Settings" />
            //         </button>
            //     </div>
            //     <div>
            //         <button onclick={change_view(SidebarView::File)}>
            //             <img src="public/icons/icons8-home-24-white.svg" class="w-12 h-12" alt="File" />
            //         </button>
            //     </div>
            //     <div>
            //         <button onclick={change_view(SidebarView::User)}>
            //             <img src="public/icons/icons8-home-24-white.svg" class="w-12 h-12" alt="User" />
            //         </button>
            //     </div>
            // </div>
            <SidebarIcons view={props.view.clone()} />
            <SidebarPanels filenames={props.filenames.clone()} />
            // <div class="p-2">
            //     <h1 class="text-2xl font-bold mb-4">{"GongCheck"}</h1>
            //     <nav>
            //         <ul>
            //             { for props.filenames.iter().map(|filename| html! {
            //                 <li class="mb-2"><a href="#" class="text-gray-300 hover:text-white">{ filename }</a></li>
            //             })}
            //         </ul>
            //     </nav>
            // </div>
        </aside>
    }
}