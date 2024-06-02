use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub filenames: Vec<String>,
    pub view: UseStateHandle<SidebarView>,
}

#[derive(Clone, PartialEq)]
pub enum SidebarView {
    Home,
    Notes,
    Settings,
    File,
    User
    // Add more views as needed
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let view = props.view.clone();
    let change_view = move |new_view: SidebarView| {
        let view = view.clone();
        Callback::from(move |_| view.set(new_view.clone()))
    };

    html! {
        <aside class="flex w-1/4 bg-gray-900 text-white p-4">
            <div class="flex-col p-2">
                <div>
                    <button onclick={change_view(SidebarView::Home)} class="w-6 h-6">
                        <img src="/icons/icon8-home-48.svg" alt="Home" />
                    </button>
                </div>
                <div>
                    <button onclick={change_view(SidebarView::Notes)} class="w-6 h-6"><img src="home.svg" alt="Notes" /></button>
                </div>
                <div>
                    <button onclick={change_view(SidebarView::Settings)} class="w-6 h-6"><img src="home.svg" alt="Settings" /></button>
                </div>
                <div>
                    <button onclick={change_view(SidebarView::File)} class="w-6 h-6"><img src="home.svg" alt="File" /></button>
                </div>
                <div>
                    <button onclick={change_view(SidebarView::User)} class="w-6 h-6"><img src="home.svg" alt="User" /></button>
                </div>
            </div>
            <div>
                <h1 class="text-2xl font-bold mb-4">{"GongCheck"}</h1>
                <nav>
                    <ul>
                        { for props.filenames.iter().map(|filename| html! {
                            <li class="mb-2"><a href="#" class="text-gray-300 hover:text-white">{ filename }</a></li>
                        })}
                    </ul>
                </nav>
            </div>
        </aside>
    }
}