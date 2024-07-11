use yew::prelude::*;

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

    html! {
        <div class="flex flex-col w-48">
            <div class="p-4 text-center text-2xl font-bold">{ props.root_path.clone() }</div>
            { for buttons }
        </div>
    }
}