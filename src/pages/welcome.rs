// use crate::components::{welcome::Welcome, drawer::Drawer};
use crate::components::welcome::Welcome;
use yew::prelude::*;
// use crate::pages::background::Background;
// use crate::components::modals::modals::Modals;
// use crate::components::toasts::Toaster;

#[function_component(WelcomePage)]
pub fn welcome() -> Html {
    html! {
        <Welcome />
    }
    // html! {
    //     <Drawer>
    //         <Background>
    //             <WelcomeHero />
    //         </Background>
    //         <Modals />
    //         <Toaster />
    //     </Drawer>
    // }
}