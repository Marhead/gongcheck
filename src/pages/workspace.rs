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
            // <aside class="w-1/4 bg-gray-900 text-white p-4">
                // <h1 class="text-2xl font-bold mb-4">{"folder-name"}</h1>
                // <nav>
                //     <ul>
                //         <li class="mb-2"><a href="#" class="text-gray-300 hover:text-white">{"Home"}</a></li>
                //         <li class="mb-2"><a href="#" class="text-gray-300 hover:text-white">{"Notes"}</a></li>
                //         <li class="mb-2"><a href="#" class="text-gray-300 hover:text-white">{"Settings"}</a></li>
                //     </ul>
                // </nav>
            // </aside>
            <main class="flex-1 bg-gray-100 p-4">
                <Title />
                <MarkdownEditor />
            </main>
        </div>
    }
}