use crate::Route;
use dioxus::prelude::*;
mod header;
mod footer;
pub use header::*;
pub use footer::*;

/// The Layout component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common header. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Layout() -> Element {
    rsx! {
        Header {}
        main { id: "view", class: "cmax-w c-w min-h-[calc(100dvh-8rem)] mx-auto",
            // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
            // same as render body {children} in a normal component, but it also handles rendering the correct component for the current route and synchronizing
            Outlet::<Route> {}
        }
        Footer {}
    }
}