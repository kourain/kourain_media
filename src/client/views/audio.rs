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
pub fn Audio(default_path:String) -> Element {
    let settings = use_context::<SettingProviderState>();
    let mut file_path = use_signal(|| default_path);
    let mut selected_format = settings.default_selected_format;
    let mut bit_rate = settings.default_bit_rate;
    let mut sample_rate = settings.default_sample_rate;
    let channel = settings.default_channels;
    let mut max_instance = settings.default_max_instances;
    let mut is_converting = use_signal(|| false);
    use_effect(move || {
        if is_converting() {
            dioxus::prelude::spawn(async move {
                let mut current_state = HashMap::new(); // Lưu trạng thái hiện tại của các file đang convert
                print!("progress checker started.\n");
                loop {
                    if !is_converting() {
                        break;
                    }
                    print!("Checking converting progress...\n");
                    if !is_all_converting_finished() {
                        let converting_list = get_converting_progress();
                        for (file_name, progress) in converting_list {
                            let prev_perc = current_state.get(&file_name).copied().unwrap_or(0);
                            if progress > prev_perc {
                                print!("{}: {}%\n", file_name, progress);
                                current_state.insert(file_name.clone(), progress);
                                eval(&format!(
                                    // if count_thread_running == 0 {
                                    r#"let el = document.getElementById("cv-perc-{}");if (el) el.innerText = "{}%";"#,
                                    file_name.sub_string(0, 50), // first 10 chars of slug as id
                                    progress,
                                ));
                            }
                        }
                    } else {
                        let converting_list = get_converting_progress();
                        // Tránh race ở thời điểm vừa bấm Convert nhưng thread chưa kịp đăng ký.
                        // Nếu chưa thấy file nào, đợi vòng sau thay vì dừng checker ngay.
                        if converting_list.is_empty() && current_state.is_empty() {
                            tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
                            continue;
                        }
                        // Sau khi hoàn tất, cập nhật lại progress của tất cả file về 100%
                        for (file_name, _) in converting_list {
                            let prev_perc = current_state.get(&file_name).copied().unwrap_or(0);
                            if 100 > prev_perc {
                                current_state.insert(file_name.clone(), 100);
                                eval(&format!(
                                    // if count_thread_running == 0 {
                                    r#"let el = document.getElementById("cv-perc-{}");if (el) el.innerText = "100%";"#,
                                    file_name.sub_string(0, 50), // first 10 chars of slug as id
                                ));
                            }
                        }
                        is_converting.set(false);
                        break;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
                print!("progress checker stopped.\n");
            });
        }
    });
    let bytes_per_sec = use_memo(move || {
        let format = selected_format();
        let bit_rate_val = bit_rate() as f64;
        let sample_rate_val = sample_rate() as f64;
        let channels = channel() as f64;
        match format.as_str() {
            // opus — nén hiệu quả hơn mp3, bitrate thấp hơn cùng chất lượng
            // opus cố định sample rate nội bộ là 48000 Hz
            "opus" => (bit_rate_val * 1000.0) / 8.0,
            // file nén — chỉ bitrate ảnh hưởng
            "mp3" | "aac" | "ogg" => (bit_rate_val * 1000.0) / 8.0,
            // flac — nén lossless ~50-60% so với WAV
            "flac" => {
                let bit_depth = 16.0;
                let compression_ratio = 0.55;
                sample_rate_val * channels * (bit_depth / 8.0) * compression_ratio
            }
            // file không nén — sample_rate * channels * bit_depth / 8
            "wav" | "pcm" => {
                let bit_depth = 16.0;
                sample_rate_val * channels * (bit_depth / 8.0)
            }
            _ => (bit_rate_val * 1000.0) / 8.0,
        }
    });
    let all_file = use_memo(move || {
        let current_path = file_path();
        if current_path.trim().is_empty() {
            Vec::new()
        } else {
            get_all_audioable_in_directory(current_path.as_str())
        }
    });
    let file_list = use_memo(move || {
        let current_path = file_path();
        if current_path.trim().is_empty() {
            Vec::new()
        } else {
            let mut result = Vec::new();
            for item in all_file().clone() {
                let size = std::fs::metadata(&item).map(|meta| meta.len()).unwrap_or(0);
                let media_info = get_audio_info(&item).unwrap_or(MediaInfo {
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
        let files = all_file();
        consume_context::<LoadingProviderState>()
            .is_loading
            .set(true);
        async move {
            let mut overrides = HashMap::new();
            for path in files {
                if let Some(br) = get_audio_bit_rate_ffprobe_async(&path).await {
                    overrides.insert(path, br);
                }
            }
            consume_context::<LoadingProviderState>()
                .is_loading
                .set(false);
            overrides
        }
    });
    let resolved_bit_rates = bit_rate_overrides().unwrap_or_default();
    let display_rows: Vec<((std::path::PathBuf, u64, u64, MediaInfo), u32)> = file_list()
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
            div { class: "flex flex-wrap justify-center items-center gap-2 mb-4 text-white",
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
                    onselect: move |_| {
                        if selected_format() == "opus" {
                            sample_rate.set(48000);
                        }
                    },
                    class: "p-2 border rounded bg-white text-black",
                    for ext in [16, 24, 32, 48, 64, 96, 128, 192, 256, 320].iter() {
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
                {" Max Instances: "}
                select {
                    value: max_instance(),
                    onchange: move |e| max_instance.set(e.value().parse::<u32>().unwrap_or(3)),
                    class: "p-2 border rounded bg-white text-black",
                    for ext in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].iter() {
                        option { class: "text-black", value: ext.to_string(), "{ext}" }
                    }
                }
                AppButton {
                    class: {
                        format!(
                            "p-2 text-white rounded {}",
                            if is_converting() { " bg-red-500" } else { " bg-green-500" },
                        )
                    },
                    disabled: file_list().is_empty(),
                    onclick: move |_| async move {
                        if is_converting() {
                            kill_all_ffmpeg_processes();
                            is_converting.set(false);
                            return;
                        }

                        let files = file_list();
                        let output_type = selected_format();
                        let output_bit_rate = bit_rate();

                        set_max_ffmpeg_instances(max_instance());

                        for (path, _, _, _) in &files {
                            let file_name = path
                                .file_stem()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string()
                                .to_slug()
                                .sub_string(0, 50);
                            eval(
                                &format!(
                                    r#"let el = document.getElementById("cv-perc-{}");if (el) el.innerText = "-";"#,
                                    file_name,
                                ),
                            );
                        }
                        for (path, _, _, media_info) in files {
                            let duration_ms = media_info.duration_ms.unwrap_or(0);
                            add_file_to_converting_list(
                                &path,
                                &output_type,
                                output_bit_rate,
                                duration_ms,
                            );
                        }
                        is_converting.set(true);
                    },
                    {if is_converting() { "Stop" } else { "Convert" }}
                }
                AppButton {
                    id: "open-output-folder-btn",
                    class: "p-2 text-white rounded bg-blue-500",
                    onclick: move |_| async move {
                        let current_path = file_path();
                        let encode_type = selected_format();
                        if current_path.trim().is_empty() {
                            return;
                        }
                        let output_folder = format!("{}\\{}", current_path, encode_type);
                        open_windows_explorer(&output_folder)
                            .unwrap_or_else(|e| eprintln!("Failed to open output folder: {}", e));
                    },
                    {"Open Output Folder"}
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
                            td { class: "col-span-3 line-clamp-2",
                                "{file.0.file_name().unwrap_or_default().to_string_lossy()}"
                            }
                            td {
                                div { class: "text-sm flex flex-col items-center gap-y-1",
                                    span { "{file.1.format_file_size()}" }
                                    span {
                                        class: format!(
                                            "text-md text-white {} px-1 rounded",
                                            if file.2 < file.1 { "bg-green-500" } else { "bg-red-500" },
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
                                    "cv-perc-{}",
                                    file
                                        .0
                                        .file_stem()
                                        .unwrap_or_default()
                                        .to_string_lossy()
                                        .to_string()
                                        .to_slug()
                                        .sub_string(0, 50),
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
