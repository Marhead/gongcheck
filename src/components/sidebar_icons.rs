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
        <div class="flex-col border-4 border-amber-200">
            <div>
                <button onclick={change_view(SidebarView::Home)}>
                    <img src="public/icons/icons8-home-24-white.svg" class="w-12 h-12" alt="Home"/>
                </button>
            </div>
            <div>
                <button onclick={change_view(SidebarView::Notes)}>
                    <img src="public/icons/icons8-home-24-white.svg" class="w-12 h-12" alt="Notes" />
                </button>
            </div>
            <div>
                <button onclick={change_view(SidebarView::Settings)}>
                    <img src="public/icons/icons8-home-24-white.svg" class="w-12 h-12" alt="Settings" />
                </button>
            </div>
            <div>
                <button onclick={change_view(SidebarView::File)}>
                    <img src="public/icons/icons8-home-24-white.svg" class="w-12 h-12" alt="File" />
                </button>
            </div>
            <div>
                <button onclick={change_view(SidebarView::User)}>
                    <img src="public/icons/icons8-home-24-white.svg" class="w-12 h-12" alt="User" />
                </button>
            </div>
        </div>
    }
}
