use crate::{
    client::components::*,
    helpers::{MediaInfo, get_audio_info},
    server::*,
};
use dioxus::document::eval;
use kourain_core::ToSlug;
use rfd::FileDialog;
use std::collections::HashMap;

/// return (new_size, new_length)
fn calc_new_size(original_length: u64, bytes_per_second: f64) -> u64 {
    let new_size = bytes_per_second * (original_length as f64 / 1000.0) as f64; // Convert ms to seconds
    new_size as u64
}
/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Audio() -> Element {
    let mut file_path = use_signal(String::new);
    let mut selected_format = use_signal(|| String::from("opus"));
    let mut bit_rate = use_signal(|| 32);
    let mut sample_rate = use_signal(|| 48000); // Opus default: 48000 Hz
    let channel = use_signal(|| 1);
    let mut max_size = use_signal(|| 103_809_024u64); // 99 MB in bytes
    let mut is_converting = use_signal(|| false);
    
    // Sync sample_rate với selected_format
    use_effect(move || {
        if selected_format() == "opus" {
            sample_rate.set(48000);
        }
    });
    let bytes_per_sec = use_memo(move || {
        let format = selected_format();
        let bit_rate_val = bit_rate() as f64;
        let sample_rate_val = sample_rate() as f64;

        match format.as_str() {
            // opus — nén hiệu quả hơn mp3, bitrate thấp hơn cùng chất lượng
            // opus cố định sample rate nội bộ là 48000 Hz
            "opus" => (bit_rate_val * 1000.0) / 8.0,
            // file nén — chỉ bitrate ảnh hưởng
            "mp3" | "aac" | "ogg" => (bit_rate_val * 1000.0) / 8.0,
            // flac — nén lossless ~50-60% so với WAV
            "flac" => {
                let channels = channel() as f64;
                let bit_depth = 16.0;
                let compression_ratio = 0.55;
                sample_rate_val * channels * (bit_depth / 8.0) * compression_ratio
            }
            // file không nén — sample_rate * channels * bit_depth / 8
            "wav" | "pcm" => {
                let channels = channel() as f64;
                let bit_depth = 16.0;
                sample_rate_val * channels * (bit_depth / 8.0)
            }
            _ => (bit_rate_val * 1000.0) / 8.0,
        }
    });
    let file_list = use_memo(move || {
        let current_path = file_path();
        if current_path.trim().is_empty() {
            Vec::new()
        } else {
            let all_item = get_all_audioable_in_directory(current_path.as_str());
            let mut result = Vec::new();
            for item in all_item.iter() {
                let size = std::fs::metadata(item).map(|meta| meta.len()).unwrap_or(0);
                let media_info = get_audio_info(item).unwrap_or(MediaInfo {
                    sample_rate: None,
                    bit_rate: None,
                    channels: None,
                    duration_ms: None,
                    codec: None,
                });
                result.push((
                    item.clone(),
                    size.clone(),
                    calc_new_size(media_info.clone().duration_ms.unwrap_or(0), bytes_per_sec()),
                    media_info,
                ));
            }
            result
        }
    });
    let bit_rate_overrides = use_resource(move || {
        let files = file_list();
        async move {
            let mut overrides = HashMap::new();
            for (path, _, _, media_info) in files {
                if !media_info.bit_rate.is_some() {
                    if let Some(br) = get_audio_bit_rate_ffprobe_async(&path).await {
                        overrides.insert(path, br);
                    }
                }
            }
            overrides
        }
    });
    let resolved_bit_rates = bit_rate_overrides().unwrap_or_default();
    let display_rows = file_list()
        .into_iter()
        .map(|file| {
            let current_bit_rate = resolved_bit_rates
                .get(&file.0)
                .copied()
                .unwrap_or(file.3.bit_rate.unwrap_or(0));
            (file, current_bit_rate)
        })
        .collect::<Vec<_>>();
    rsx! {
        h1 { class: "text-3xl font-bold mb-4", "Audio" }
        p { "This is the audio page." }
        AppContainer { id: "input", class: "flex flex-rows gap-x-1",
            AppInput {
                id: "file-folder-path-input",
                class: "w-full text-left",
                // disabled: true,
                r#type: "button",
                value: if file_path() == "" { "Select Folder".to_string() } else { file_path() },
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
            }
        }
        AppContainer { id: "file-table",
            div { class: "flex items-center gap-x-2 mb-4 text-white",
                {"Convert to:"}
                select {
                    value: selected_format(),
                    onchange: move |e| selected_format.set(e.value()),
                    class: "p-2 border rounded bg-white text-black",
                    for ext in ["opus", "aac", "ogg", "mp3", "flac", "wav"].iter() {
                        option { class: "text-black", value: ext.to_string(), "{ext}" }
                    }
                }
                {" Bit rate: "}
                select {
                    value: bit_rate(),
                    onchange: move |e| bit_rate.set(e.value().parse::<u32>().unwrap_or(128)),
                    class: "p-2 border rounded bg-white text-black",
                    for ext in [24, 32, 48, 64, 96, 128, 192, 256, 320].iter() {
                        option { class: "text-black", value: ext.to_string(), "{ext} kbps" }
                    }
                }
                {" Sample rate: "}
                select {
                    value: sample_rate(),
                    disabled: selected_format() == "opus", // Opus chỉ hỗ trợ 48000 Hz
                    onchange: move |e| sample_rate.set(e.value().parse::<u32>().unwrap_or(44100)),
                    class: "p-2 border rounded bg-white text-black",
                    for ext in [8000, 16000, 22050, 24000, 32000, 44100, 48000, 96000].iter() {
                        option { class: "text-black", value: ext.to_string(), "{ext} Hz" }
                    }
                }
                {" Max Size: "}
                select {
                    value: max_size(),
                    onchange: move |e| max_size.set(e.value().parse::<u64>().unwrap_or(104857600)),
                    class: "p-2 border rounded bg-white text-black",
                    for ext in [103_809_024, 524_288_000, 1_073_741_824].iter() {
                        option { class: "text-black", value: ext.to_string(), "{ext.format_file_size()}" }
                    }
                }
                AppButton {
                    class: "p-2",
                    disabled: is_converting(),
                    onclick: move |_| async move {
                        if is_converting() {
                            return;
                        }

                        is_converting.set(true);

                        let files = file_list();
                        let output_type = selected_format();
                        let output_bit_rate = bit_rate();

                        for (path, _, _, media_info) in files {
                            let duration_ms = media_info.duration_ms.unwrap_or(0);
                            let key_for_done = path.clone();
                            add_file_to_converting_list(
                                &path,
                                &output_type,
                                output_bit_rate,
                                duration_ms,
                                move |progress, file_name, count_thread_running| {
                                    println!(
                                        "Progress: {}%, File: {}, Running Threads: {}",
                                        progress,
                                        file_name,
                                        count_thread_running,
                                    );
                                    eval( // if count_thread_running == 0 {
                                        &format!(
                                            r#"let el = document.getElementById("convert-status-{}");if (el) el.innerText = "{}%";"#,
                                            file_name,
                                            progress,
                                        ),
                                    );
                                    if progress == 100 {
                                        remove_file_from_converting_list(&key_for_done);
                                    }
                                },
                            );
                        }
                        is_converting.set(false);
                    },
                    {if is_converting() { "Converting..." } else { "Convert" }}
                }
            }
            table { class: "w-full border-collapse text-white border",
                thead { class: "w-full border-collapse",
                    tr { class: "w-full grid grid-cols-8 text-center border-b",
                        th { class: "col-span-3", "File Name" }
                        th { "File Size" }
                        th { "bit rate" }
                        th { "sample rate" }
                        th { "Duration" }
                        th { "Convert" }
                    }
                }
                tbody {
                    for (file , current_bit_rate) in display_rows {
                        tr {
                            class: "w-full grid grid-cols-8 text-center border-b",
                            id: file.0.file_stem().unwrap_or_default().to_string_lossy().into_owned(),
                            td { class: "col-span-3",
                                "{file.0.file_name().unwrap_or_default().to_string_lossy()}"
                            }
                            td {
                                div { class: "text-sm flex flex-col items-center gap-y-1",
                                    span { "{file.1.format_file_size()}" }
                                    span {
                                        class: format!(
                                            "text-md text-white {} px-1 rounded",
                                            if file.2 < max_size() { "bg-green-500" } else { "bg-red-500" },
                                        ),
                                        "-> {file.2.format_file_size()}"
                                    }
                                }
                            }
                            td {
                                div { class: "text-sm flex flex-col items-center gap-y-1",
                                    span {
                                        {
                                            if current_bit_rate == 0 {
                                                "Loading".to_string()
                                            } else {
                                                format!("{} kbps", current_bit_rate)
                                            }
                                        }
                                    }
                                    span {
                                        class: format!(
                                            "text-md text-white {} px-1 rounded",
                                            if current_bit_rate >= bit_rate() { "bg-green-500" } else { "bg-red-500" },
                                        ),
                                        "-> {bit_rate()} kbps"
                                    }
                                }
                            }
                            td {
                                div { class: "text-sm flex flex-col items-center gap-y-1",
                                    span { "{file.3.sample_rate.unwrap_or(0)} Hz" }
                                    span {
                                        class: format!(
                                            "text-md text-white {} px-1 rounded",
                                            if file.3.sample_rate.unwrap_or(0) >= sample_rate() {
                                                "bg-green-500"
                                            } else {
                                                "bg-red-500"
                                            },
                                        ),
                                        "-> {sample_rate()} Hz"
                                    }
                                }
                            }
                            td { "{file.3.duration_ms.unwrap_or(0).format_duration()}" }
                            td {
                                id: format!(
                                    "convert-status-{}",
                                    file.0.file_stem().unwrap_or_default().to_string_lossy().to_string().to_slug(),
                                ),
                                "-"
                            }
                        }
                    }
                }
            }
        }
    }
}
