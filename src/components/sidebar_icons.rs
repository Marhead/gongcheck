use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum SidebarView {
    Home,
    Notes,
    Settings,
    File,
    User,
}

#[derive(Properties, PartialEq)]
pub struct SidebarIconsProps {
    pub view: UseStateHandle<SidebarView>,
}

#[function_component(SidebarIcons)]
pub fn sidebar_icons(props: &SidebarIconsProps) -> Html {
    let view = props.view.clone();

    let change_view = |new_view: SidebarView| {
        let view = view.clone();
        Callback::from(move |_| view.set(new_view.clone()))
    };

    html! {
        <div class="flex flex-col bg-wookd-400 p-2 justify-between">
            <div>
                <div>
                    <button onclick={change_view(SidebarView::Home)}>
                        <img src="public/icons/home.svg" class="w-6 h-6" alt="Home"/>
                    </button>
                </div>
                <div>
                    <button onclick={change_view(SidebarView::Notes)}>
                        <img src="public/icons/search.svg" class="w-6 h-6" alt="Search" />
                    </button>
                </div>
                <div>
                    <button onclick={change_view(SidebarView::Settings)}>
                        <img src="public/icons/files.svg" class="w-6 h-6" alt="Files" />
                    </button>
                </div>
                <div>
                    <button onclick={change_view(SidebarView::File)}>
                        <img src="public/icons/partition.svg" class="w-6 h-6" alt="Partition" />
                    </button>
                </div>
                <div>
                    <button onclick={change_view(SidebarView::User)}>
                        <img src="public/icons/picture.svg" class="w-6 h-6" alt="Picture" />
                    </button>
                </div>
            </div>
            <div>
                <div>
                    <button>
                        <img src="public/icons/printer.svg" class="w-6 h-6" alt="Printer"/>
                    </button>
                </div>
                <div>
                    <button onclick={change_view(SidebarView::User)}>
                        <img src="public/icons/user.svg" class="w-6 h-6" alt="User" />
                    </button>
                </div>
                <div>
                    <button onclick={change_view(SidebarView::User)}>
                        <img src="public/icons/settings.svg" class="w-6 h-6" alt="Settings" />
                    </button>
                </div>
            </div>
        </div>
    }
}
