use yew::prelude::*;
use crate::components::overview::*;

#[derive(Properties, PartialEq)]
pub struct OverviewProps {
    pub labels: Vec<String>,
}

#[function_component(Overview)]
pub fn overview(props: &OverviewProps) -> Html {
    let labels = props.labels.clone();
    html! {
        <div class="flex flex-col w-full">
            <OverviewNavigator />
        </div>
    }
}