use yew::prelude::*;
use pulldown_cmark::{html, Parser};

pub struct Editor {
    input: String,
    output: String,
}

pub enum Msg {
    UpdateInput(String),
}

impl Component for Editor {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            input: String::new(),
            output: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateInput(input) => {
                self.output = Self::render_markdown(&input);
                self.input = input;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <textarea
                    value={self.input.clone()}
                    oninput={ctx.link().callback(|e: InputEvent| {
                        let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                        Msg::UpdateInput(input.value())
                    })}
                />
                <div>
                    <h3>{ "Preview" }</h3>
                    <div dangerously_set_inner_html={self.output.clone()} />
                </div>
            </div>
        }
    }
}

impl Editor {
    fn render_markdown(input: &str) -> String {
        let parser = Parser::new(input);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}

// fn main() {
//     yew::start_app::<Editor>();
// }
