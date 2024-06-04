use crate::components::{Title, MarkdownEditor, Sidebar, SidebarView};

use yew::prelude::*;


#[function_component(Workspace)]
pub fn workspace() -> Html{
    
    let filenames = vec![
        "Note1.md".to_string(),
        "Note2.md".to_string(),
        "Note3.md".to_string(),
    ];

    // Ensure use_state is declared here
    let sidebar_view = use_state(|| SidebarView::Home);

    html!{
        <div class="flex h-screen">
            <Sidebar filenames={filenames} view={sidebar_view.clone()}/>
            <main class="flex-1 bg-wookd-100 p-4">
                <Title />
                <MarkdownEditor />
            </main>
        </div>
    }
}