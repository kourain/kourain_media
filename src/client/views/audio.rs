use crate::{client::components::*, server::*};
use rfd::FileDialog;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Audio() -> Element {
    let mut file_path = use_signal(String::new);
    let file_list = use_memo(move || {
        let current_path = file_path();
        if current_path.trim().is_empty() {
            Vec::new()
        } else {
            get_all_files_in_directory(current_path.as_str())
        }
    });
    rsx! {
        h1 { class: "text-3xl font-bold mb-4", "Audio" }
        p { "This is the audio page." }
        AppContainer { id: "input", class: "flex flex-rows gap-x-1",
            AppInput {
                id: "file-folder-path-input",
                class: "w-[80%]",
                disabled: true,
                value: file_path(),
            }
            AppButton {
                id: "select-folder",
                class: "w-[20%] bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| async move {
                    let ket_qua = FileDialog::new()
                        .set_title("Chọn thư mục")
                        .pick_folder();
                    if let Some(thu_muc) = ket_qua {
                        let selected_path = thu_muc.to_string_lossy().to_string();
                        file_path.set(selected_path.clone());
                        println!("ket_qua: {selected_path}");
                    }
                },
                {"Select Folder"}
            }
        }
        AppContainer { id: "file-table",
            table { class: "w-full border-collapse text-white border",
                thead { class: "w-full border-collapse",
                    tr { class: "w-full grid grid-cols-5",
                        th { "File Name" }
                        th { "File Size" }
                        th { "bit rate" }
                        th { "sample rate" }
                        th { "Duration" }
                    }
                }
                tbody {
                    for file in file_list().iter() {
                        tr { class: "w-full grid grid-cols-5 text-center ",
                            td { "{file.file_name().and_then(|name| name.to_str()).unwrap_or(\"-\")}" }
                            td {
                                "{std::fs::metadata(file).map(|meta| meta.len().to_string()).unwrap_or_else(|_| \"-\".to_string())}"
                            }
                            td { "-" }
                            td { "-" }
                            td { "-" }
                        }
                    }
                }
            }
        }
    }
}
