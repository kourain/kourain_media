// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;
/// Define a components module that contains all shared components for our app.
mod client;
mod helpers;
mod server;
use client::*;

use crate::client::components::{ASSETS};

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
///
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
// GENERATED ROUTE ENUM on build.rs

// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const TAILWIND_CSS: Asset = asset!("assets/tailwind.css");
const CUSTOM_CSS: Asset = asset!("assets/custom.css");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    #[cfg(feature = "desktop")]
    fn launch_app() {
        use dioxus::desktop::tao;
        let window = tao::window::WindowBuilder::new().with_resizable(true);
        dioxus::LaunchBuilder::new()
            .with_cfg(
                dioxus::desktop::Config::new()
                    .with_window(window)
                    .with_menu(None)
            )
            .launch(App);
    }

    #[cfg(not(feature = "desktop"))]
    fn launch_app() {
        dioxus::launch(App);
    }

    launch_app();
}
/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: ASSETS::FAVICON_ICO }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: CUSTOM_CSS }
        document::Title { "Kourain Media" }
        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
    }
}
