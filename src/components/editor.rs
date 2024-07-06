use yew::prelude::*;
use yew::use_effect;
use gongcheck_editor::WebEditor;
use wasm_bindgen::JsValue;
use web_sys::Element;
use std::rc::Rc;
use std::cell::RefCell;

#[function_component(EditorComponent)]
pub fn editor_component() -> Html {
    let editor_ref = use_node_ref();
    let preview_ref = use_node_ref();
    let editor = use_state(|| None::<Rc<RefCell<WebEditor>>>);

    {
        let editor = editor.clone();
        let editor_ref = editor_ref.clone();
        let preview_ref = preview_ref.clone();

        use_effect(move || {
            if editor.is_some() {
                return;
            }

            if let (Some(editor_div), Some(preview_div)) = (editor_ref.cast::<Element>(), preview_ref.cast::<Element>()) {
                match WebEditor::new(&format!("#{}", editor_div.id()), &format!("#{}", preview_div.id())) {
                    Ok(web_editor) => {
                        let web_editor = Rc::new(RefCell::new(web_editor));
                        if let Err(e) = web_editor.borrow().set_content("# Hello, Markdown!\n\nThis is a **test**.") {
                            web_sys::console::error_1(&format!("Failed to set content: {:?}", e).into());
                        }
                        editor.set(Some(web_editor));
                    },
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to initialize editor: {:?}", JsValue::from(e)).into());
                    }
                }
            } else {
                web_sys::console::error_1(&"Editor or preview element not found".into());
            }
        });
    };

    html! {
        <div>
            <h1>{"GongCheck Editor"}</h1>
            <div ref={editor_ref} id="editor" class="border-2 border-rose-500"></div>
            <div ref={preview_ref} id="preview" class="border-2 border-sky-500"></div>
        </div>
    }
}