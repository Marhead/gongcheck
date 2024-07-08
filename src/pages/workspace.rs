use serde::{Deserialize, Serialize};
use yew::prelude::*;
use wasm_bindgen::prelude::*;

use crate::components::{Title, MarkdownEditor, Sidebar, SidebarView};
use crate::pages::*;

#[derive(Properties, PartialEq)]
pub struct RootPathProps {
    pub root_path: String,
}

#[function_component(Workspace)]
pub fn workspace(props: &RootPathProps) -> Html{
    let labels = vec![
        "Overview".to_string(),
        "Stories".to_string(),
        "Notes".to_string(),
        "Characters".to_string(),
        "Organizations".to_string(),
        "Cultures".to_string(),
        "Species".to_string(),
        "Locations".to_string(),
        "Discoveries".to_string(),
        "Relations".to_string(),
        "Items".to_string(),
        "Images".to_string()
    ];

    let current_page = use_state(|| labels[0].clone());

    let sidebar_load = {
        let current_page = current_page.clone();
        Callback::from(move |message: String| {
            current_page.set(message);
        })
    };

    html!{
        <div class="flex h-screen">
            <Sidebar {sidebar_load} labels={labels.clone()}/>
            <main>
                // <Title />
                // <MarkdownEditor />
                {
                    match current_page.as_str() {
                        "Overview" => html! { <Overview /> },
                        "Stories" => html! { <Story /> },
                        "Notes" => html! { <Note /> },
                        "Characters" => html! { <Character /> },
                        "Organizations" => html! { <Organization /> },
                        "Cultures" => html! { <Culture /> },
                        "Species" => html! { <Specy /> },
                        "Locations" => html! { <Location /> },
                        "Discoveries" => html! { <Discovery /> },
                        "Relations" => html! { <Relation /> },
                        "Items" => html! { <Item /> },
                        "Images" => html! { <Image /> },
                        _ => html! {},
                    }
                }
            </main>
        </div>
    }
}