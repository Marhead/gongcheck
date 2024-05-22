// use yew::prelude::*;

// #[function_component(MarkdownEditor)]
// fn markdown_editor() -> Html {
//     let markdown = use_state(|| String::from(""));
//     let html_output = use_state(|| String::from(""));

//     let on_input = {
//         let markdown = markdown.clone();
//         let html_output = html_output.clone();
//         Callback::from(move |e: InputEvent| {
//             if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
//                 let value = input.value();
//                 markdown.set(value.clone());

//                 // Convert Markdown to HTML (simple conversion for demo purposes)
//                 let converted_html = value.replace("\n", "<br>");
//                 html_output.set(converted_html);
//             }
//         })
//     };

//     html! {
//         <div>
//             <textarea
//                 id="markdown-input"
//                 oninput={on_input.clone()}
//                 placeholder="Type your markdown here"
//             />
//             <div id="markdown-preview" dangerously_set_inner_html={html_output.as_ref()} />
//         </div>
//     }
// }