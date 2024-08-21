use yew::prelude::*;
use crate::components::overview::modal::Modal;

#[derive(Properties, PartialEq)]
pub struct OverviewCardProps {
    pub title: String,
}

#[function_component(OverviewCard)]
pub fn overview_card(props: &OverviewCardProps) -> Html {
    let modal_visible = use_state(|| false);

    let toggle_modal = {
        let modal_visible = modal_visible.clone();
        Callback::from(move |_| {
            modal_visible.set(!*modal_visible);
        })
    };

    html! {
        <div class="flex flex-col m-2 p-2 border-2">
            <div class="flex justify-between">
                <h1 class="m-2">{ props.title.clone() }</h1>
                <div>
                    <button class="m-2">{ "Grid" }</button>
                    <button class="m-2">{ "List"} </button>
                    <button class="m-2" onclick={toggle_modal.clone()}>{ "+" }</button>
                </div>
            </div>
            <div class="border-1">
                { "This is the overview page" }
            </div>
            if *modal_visible {
                <Modal on_close={toggle_modal.clone()} />
            }
        </div>
    }
}