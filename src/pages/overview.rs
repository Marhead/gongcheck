use std::collections::HashMap;

use yew::prelude::*;
use crate::components::overview::*;

#[derive(Properties, PartialEq)]
pub struct OverviewProps {
    pub labels: Vec<String>,
}

#[function_component(Overview)]
pub fn overview(props: &OverviewProps) -> Html {
    // Borrow "labels" as mutable
    let mut labels = props.labels.clone();
    
    // Remove the first element("Overview")
    labels.remove(0);

    // Add "Tags" to the end of the list
    labels.push("Tags".to_string());

    // Create a HashMap to store the sections
    let mut sections = HashMap::new();

    // Insert the labels into the HashMap
    labels.iter().for_each(|label| {
        if label == "Stories" || label == "Characters" {
            sections.insert(label.clone(), true);
        } else {
            sections.insert(label.clone(), false);
        }
    });

    html! {
        <div class="flex flex-col w-full">
            <OverviewNavigator />
            <OverviewCard title="Something"/>
        </div>
    }
}