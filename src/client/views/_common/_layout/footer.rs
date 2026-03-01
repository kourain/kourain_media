use dioxus::prelude::*;


/// The Footer component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This footer component is rendered under the outlet inside the [Layout] component
#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            id: "footer",
            class: "h-12 text-center text-sm p-4 border-t border-stone-300 dark:border-stone-700 mt-4",
            p { "Made by Kourain@2026" }
        }
    }
}
