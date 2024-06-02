use yew::prelude::*;

#[function_component(MarkdownEditor)]
pub fn markdown_editor() -> Html {
    html! {
        <div class="bg-white p-4 border border-gray-300 rounded">
            <textarea class="w-full h-full border-none focus:outline-none" placeholder="Write your note here..."></textarea>
        </div>
    }
}
