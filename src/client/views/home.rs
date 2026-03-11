use rfd::FileDialog;

use crate::{
    client::{Route, components::*},
    server::*,
};
use dioxus::document::eval;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    // use_signal is a hook. Hooks in dioxus must be run in a consistent order every time the component is rendered.
    // That means they can't be run inside other hooks, async blocks, if statements, or loops.
    //
    // use_signal is a hook that creates a state for the component. It takes a closure that returns the initial value of the state.
    // The state is automatically tracked and will rerun any other hooks or components that read it whenever it changes.
    let mut url = use_signal(|| String::new());
    let mut video_data = use_signal(|| Vec::<YoutubeVideoInfo>::new());
    let mut save_path = use_signal(|| String::new());
    let mut is_downloading = use_signal(|| false);
    use_effect(move || {
        if is_downloading() == true {
            use_future(move || async move {
                for video in video_data.iter() {
                    eval(&format!(
                        r#"let el = document.getElementById("dl-{}");if (el) el.innerText = "Downloading...";"#,
                        video.id.clone(),
                    ));
                    let mut file_size = 0;
                    let mut try_count = 0;
                    loop {
                        file_size = download_audio_async(save_path(), video.id.clone())
                            .await
                            .unwrap_or_else(|_| 0);
                        try_count += 1;
                        if try_count > 3 || file_size > 0 {
                            break;
                        }
                    }
                    eval(&format!(
                        r#"let el = document.getElementById("dl-{}");if (el) el.innerText = "{}";"#,
                        video.id.clone(),
                        file_size.format_file_size(),
                    ));
                    if is_downloading() == false {
                        break;
                    }
                }
            });
        }
    });
    use_effect(move || {
        if url() != "" && !video_data.is_empty() && save_path() != "" {
            let data = video_data.read();
            let playlist_title: String = data
                .first()
                .and_then(|v| v.playlist_title.clone())
                .unwrap_or_default()
                .trim()
                .to_string();
            if !playlist_title.is_empty() && !save_path().contains(&playlist_title) {
                let new_path = format!("{}\\{}", save_path(), playlist_title);
                save_path.set(new_path);
            }
        }
    });
    rsx! {
        AppContainer { id: "youtube",
            h4 { "Youtube Video or Playlist" }
            AppContainer { class: "flex flex-col space-y-1",
                div { class: "flex w-full",
                    AppInput {
                        id: "youtube-input",
                        class: "bg-white flex-auto",
                        placeholder: "Enter YouTube URL...",
                        value: url(),
                        oninput: move |evt: Event<FormData>| url.set(evt.value()),
                    }
                    AppButton {
                        id: "getinfo-button",
                        class: "w-[100px] flex-none bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| async move {
                            consume_context::<LoadingProviderState>().is_loading.set(true);
                            video_data.set(get_youtube_url_info_async(&url()).await.unwrap_or_default());
                            consume_context::<LoadingProviderState>().is_loading.set(false);
                        },
                        {"Get Info"}
                    }
                }
                div { class: "flex w-full gap-2",
                    AppInput {
                        id: "file-folder-path-input",
                        class: "bg-white flex-auto text-left",
                        // disabled: true,
                        r#type: "button",
                        value: if save_path() == "" { "Select Folder".to_string() } else { save_path() },
                        onclick: move |_| async move {
                            let ket_qua = FileDialog::new()
                                .set_title("Chọn thư mục")
                                .pick_folder();
                            if let Some(thu_muc) = ket_qua {
                                let selected_path = thu_muc.to_string_lossy().to_string();
                                save_path.set(selected_path.clone());
                                println!("ket_qua: {selected_path}");
                            }
                        },
                    }
                    AppButton {
                        id: "download-button",
                        class: {
                            format!(
                                "flex-none bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded {}",
                                if is_downloading() { " bg-red-500" } else { " bg-green-500" },
                            )
                        },
                        disabled: save_path() == "",
                        onclick: move |_| {
                            if video_data.is_empty() {
                                println!("No videos to download");
                                return;
                            }
                            is_downloading.set(!is_downloading());
                        },
                        {if is_downloading() { "Stop" } else { "Download" }}
                    }
                    AppButton {
                        id: "convert-button",
                        class: "flex-none bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded bg-green-500",
                        disabled: save_path() == "" || is_downloading(),
                        onclick: move |_| {
                            navigator()
                                .push(Route::Audio {
                                    default_path: save_path(),
                                });
                        },
                        "Convert"
                    }
                }
            }
            AppContainer { id: "youtube-response", class: "bg-gray-100 p-4 rounded",
                table { class: "w-full border-collapse text-white border",
                    thead { class: "w-full border-collapse",
                        tr { class: "w-full grid grid-cols-10 text-center border-b",
                            th { class: "col-span-1", "" }
                            th { class: "col-span-6", "Video Name" }
                            th { class: "col-span-2", "Duration" }
                            th { class: "col-span-1", "Download" }
                        }
                    }
                    tbody {
                        for video in video_data.iter() {
                            tr { class: "w-full grid grid-cols-10 text-center border-b",
                                // id: item.id,
                                td { class: "col-span-1", "" }
                                td { class: "col-span-6 line-clamp-2 text-left", "{video.title}" }
                                td { class: "col-span-2", "{video.duration.format_duration()}" }
                                td {
                                    id: format!("dl-{}", video.id),
                                    class: "col-span-1",
                                    "-"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
