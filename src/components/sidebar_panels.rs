use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SidebarPanelsProps {
    pub filenames: Vec<String>,
}

#[function_component(SidebarPanels)]
pub fn sidebar_panels(props: &SidebarPanelsProps) -> Html {
    html! {
        <div class="flex-1 bg-[#74512D] pl-2">
            <h1 class="text-2xl font-bold mb-4 pt-2 text-white">{"Gongcheck"}</h1>
            <nav>
                <ul>
                    { for props.filenames.iter().map(|filename| html! {
                        <li class="mb-2"><a href="#" class="text-white hover:text-gray-300">{ filename }</a></li>
                    })}
                </ul>
            </nav>
        </div>
    }
}
