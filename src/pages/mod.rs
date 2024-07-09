use yew_router::prelude::*;

pub mod workspace;
pub mod welcome;
pub mod overview;
pub mod story;
pub mod note;
pub mod character;
pub mod organization;
pub mod culture;
pub mod specy;
pub mod place;
pub mod discovery;
pub mod relation;
pub mod item;
pub mod image;

pub use workspace::*;
pub use welcome::*;
pub use overview::*;
pub use story::*;
pub use note::*;
pub use character::*;
pub use organization::*;
pub use culture::*;
pub use specy::*;
pub use place::*;
pub use discovery::*;
pub use relation::*;
pub use item::*;
pub use image::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Welcome,
    #[at("/workspace")]
    Workspace,
}