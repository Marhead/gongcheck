use crate::components::{Title, MarkdownEditor, Sidebar, SidebarView};

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use wasm_bindgen::prelude::*;


#[derive(Properties, PartialEq)]
pub struct RootPathProps {
    pub root_path: String,
}

#[function_component(Workspace)]
pub fn workspace(props: &RootPathProps) -> Html{

    html!{
        <div class="flex h-screen">
            <Sidebar />
            <main>
                <Title />
                <MarkdownEditor />
            </main>
        </div>
    }
}