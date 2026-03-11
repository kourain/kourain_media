use crate::{Route, client::components::*};
/// The Header component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This header component is rendered above the outlet inside the [Layout] component
#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            id: "header",
            class: "sticky flex justify-center top-0 z-30 h-16 bg-opacity-80 dark:text-gray-200 dark:bg-opacity-80 border-b border-stone-300 dark:border-stone-700 backdrop-blur-sm",
            div {
                id: "header-content",
                class: "cmax-w c-w h-full flex items-center gap-4  px-3 ",
                div { id: "logo",
                    Link {
                        to: Route::Home {},
                        class: "flex flex-row items-center gap-2 hover:opacity-80 transition-opacity hover:border-stone-300 dark:hover:border-stone-700 rounded-md border border-transparent p-1",
                        img {
                            src: ASSETS::FAVICON_ICO,
                            alt: "Kourain Media Logo",
                            class: "h-10 w-10 rounded-md",
                        }
                        p { class: "text-lg font-bold", "Kourain Media" }
                    }
                }
                div { id: "items",
                    Link {
                        id: "header-youtube",
                        to: Route::Home {},
                        class: "px-3 py-2 rounded-md hover:bg-stone-200 dark:hover:bg-stone-800",
                        "Youtube"
                    }
                    Link {
                        id: "header-audio",
                        to: Route::Audio {
                            default_path: String::new(),
                        },
                        class: "px-3 py-2 rounded-md hover:bg-stone-200 dark:hover:bg-stone-800",
                        "Audio"
                    }
                }
                div { id: "actions", class: "ml-auto",
                    Link {
                        id: "github",
                        to: "https://github.com/kourain/kourain_media",
                        alt: "GitHub",
                        class: "p-3 inline-flex opacity-80 hover:opacity-100 transition-opacity rounded-md border border-transparent hover:border-stone-300 dark:hover:border-stone-700",
                        Icon { icon: dioxus_free_icons::icons::ld_icons::LdGithub }
                    }
                }
            }
        }
    }
}
