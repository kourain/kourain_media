use dioxus::prelude::*;
mod _common;
pub use _common::*;
mod audio;
pub use audio::*;
mod home;
pub use home::*;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/audio")]
        Audio,
        #[route("/")]
        Home,
}
