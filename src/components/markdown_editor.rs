use yew::prelude::*;
use wasm_bindgen::JsCast;

#[function_component(MarkdownEditor)]
pub fn markdown_editor() -> Html {
    // let (textarea_ref, set_textarea_ref) = use_state(|| NodeRef::default());

    // let oninput = Callback::from(move |_| {
    //     if let Some(textarea) = textarea_ref.get().dyn_ref::<web_sys::HtmlTextAreaElement>() {
    //         textarea.set_style("height", "auto").unwrap();
    //         textarea.set_style("height", &format!("{}px", textarea.scroll_height())).unwrap();
    //     }
    // });

    // html! {
    //     <div class="bg-wookd-100">
    //         <textarea ref={textarea_ref.clone()} class="bg-wookd-100 w-full h-full border-none focus:outline-none" placeholder="Write your note here..." oninput={oninput}></textarea>
    //     </div>
    // }
    html! {
        <div class="bg-wookd-100">
            <textarea class="bg-wookd-100 w-full h-full border-none focus:outline-none" placeholder="Write your note here..."></textarea>
        </div>
    }
}
