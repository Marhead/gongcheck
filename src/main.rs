pub mod pages;
pub mod components;
pub mod contexts;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::Page;
use pages::welcome::WelcomePage;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Page> render={move |page| {
                match page {
                    Page::Welcome => html!(<WelcomePage />)
                }
            }} />
        </BrowserRouter>
    }
}


fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
