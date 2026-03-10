use crate::{client::components::*, server::*};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    // use_signal is a hook. Hooks in dioxus must be run in a consistent order every time the component is rendered.
    // That means they can't be run inside other hooks, async blocks, if statements, or loops.
    //
    // use_signal is a hook that creates a state for the component. It takes a closure that returns the initial value of the state.
    // The state is automatically tracked and will rerun any other hooks or components that read it whenever it changes.
    let mut url = use_signal(|| String::new());
    let mut data = use_signal(|| Vec::<YoutubeVideoInfo>::new());
    rsx! {
        AppContainer { id: "youtube",
            h4 { "Youtube Video or Playlist" }
            AppContainer { class: "flex",
                AppInput {
                    id: "youtube-input",
                    class: "bg-white flex-auto",
                    aria_placeholder: "Enter YouTube URL...",
                    value: url(),
                    oninput: move |evt: Event<FormData>| url.set(evt.value()),
                }
                AppButton {
                    id: "youtube-button",
                    class: "w-[100px] flex-none bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    onclick: move |_| async move {
                        data.set(get_youtube_url_info(&url()).await.unwrap_or_default());
                        // response.set(data);
                    },
                    {"Get Info"}
                }
            }
            AppContainer { id: "youtube-response", class: "bg-gray-100 p-4 rounded",
                table { class: "w-full border-collapse text-white border",
                    thead { class: "w-full border-collapse",
                        tr { class: "w-full grid grid-cols-10 text-center border-b",
                            th { class: "col-span-1", "" }
                            th { class: "col-span-6", "Video Name" }
                            th { class: "col-span-2", "Duration" }
                            th { class: "col-span-1", "Convert" }
                        }
                    }
                    tbody {
                        for video in data.iter() {
                            tr { class: "w-full grid grid-cols-10 text-center border-b",
                                // id: item.id,
                                td { class: "col-span-1", "" }
                                td { class: "col-span-6 line-clamp-2 text-left", "{video.title}" }
                                td { class: "col-span-2", "{video.duration.format_duration()}" }
                                td { class: "col-span-1", "-" }
                            }
                        }
                    }
                }
            }
        }
    }
}
