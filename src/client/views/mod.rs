use dioxus::prelude::*;
mod audio;
pub use audio::*;
mod home;
pub use home::*;
mod _common;
pub use _common::*;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/audio")]
        Audio,
        #[route("/")]
        Home,
}
