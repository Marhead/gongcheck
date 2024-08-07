use yew::prelude::*;
use crate::components::EditorComponent;

#[function_component(MarkdownEditor)]
pub fn markdown_editor() -> Html {
    html! {
        <div class="bg-wookd-100">
            <EditorComponent />
        </div>
    }
}
