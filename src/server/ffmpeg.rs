use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use std::sync::LazyLock;

use kourain_core::ToSlug;

pub fn get_audio_bit_rate_ffprobe(path: &Path) -> Option<u32> {
    let mut cmd = std::process::Command::new("ffprobe");

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    let output = cmd
        .args([
            "-v",
            "quiet",
            "-select_streams",
            "a:0",
            "-show_entries",
            "stream=bit_rate",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path.to_str()?,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let trimmed = stdout.trim();
    if trimmed.is_empty() || trimmed == "N/A" {
        return None;
    }
    let bit_rate_bps: u64 = trimmed.parse().ok()?;
    Some((bit_rate_bps / 1000) as u32)
}

pub async fn get_audio_bit_rate_ffprobe_async(path: &Path) -> Option<u32> {
    let path = path.to_path_buf();
    tokio::task::spawn_blocking(move || get_audio_bit_rate_ffprobe(path.as_path()))
        .await
        .ok()
        .flatten()
}
static RUNNING_FFMPEG_PROCESSES: LazyLock<std::sync::Mutex<HashMap<String, std::thread::JoinHandle<Result<(), String>>>>> =
    LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));
/// Eg: `convert_audio(input, "opus", 128, |progress| { println!("{}%", progress); })`
fn convert_audio_async<F>(
    input_file: &Path,
    output_type: &str,
    output_bit_rate: u32,
    duration_ms: u64,
    mut on_progress: F,
) -> Result<(), String>
where
    F: FnMut(u32, String, u32),
{
    // Tạo thư mục output nếu chưa có
    let output_file = input_file
        .parent()
        .unwrap_or(Path::new("."))
        .join("opus")
        .join(input_file.file_stem().unwrap_or_default())
        .with_extension("opus");

    if let Some(parent) = output_file.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("create ouput folder fail: {}", e))?;
    }

    // Chạy ffmpeg với progress report
    let mut cmd = std::process::Command::new("ffmpeg");
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    let mut child = cmd
        .args([
            "-i",
            input_file.to_str().unwrap_or(""),
            "-ac",
            "1",
            "-c:a",
            &format!("lib{}", output_type),
            "-b:a",
            &format!("{}k", output_bit_rate),
            "-vbr",
            "on",
            "-compression_level",
            "0",
            "-application",
            "voip",
            "-progress",
            "pipe:1",
            output_file.to_str().unwrap_or(""),
            "-y"
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("start ffmpeg fail: {}", e))?;

    let stdout = child.stdout.take().ok_or("error: stdout")?;
    let reader = std::io::BufReader::new(stdout);
    let mut last_progress = 0u32;

    // Đọc progress từ ffmpeg
    use std::io::BufRead;
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("out_time_ms=") {
                if let Ok(time_ms) = line
                    .strip_prefix("out_time_ms=")
                    .unwrap_or("")
                    .parse::<u64>()
                {
                    let progress = ((time_ms * 100) / duration_ms.max(1)).min(100) as u32;
                    if progress != last_progress {
                        last_progress = progress;
                        on_progress(
                            progress,
                            format!(
                                "{}",
                                input_file
                                    .file_stem()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string()
                                    .to_slug()
                            ),
                            RUNNING_FFMPEG_PROCESSES.lock().unwrap().len() as u32,
                        );
                    }
                }
            }
        }
    }

    let status = child.wait().map_err(|e| format!("err: {}", e))?;

    if status.success() {
        on_progress(100, "Ok".to_string(), RUNNING_FFMPEG_PROCESSES.lock().unwrap().len() as u32);
        Ok(())
    } else {
        Err(format!("ffmpeg fail code: {}", status.code().unwrap_or(-1)))
    }
}
pub fn kill_all_ffmpeg_processes() {
    if let Ok(mut processes) = RUNNING_FFMPEG_PROCESSES.lock() {
        for (_, child) in processes.iter(){
            child.thread().unpark(); // Cố gắng dừng thread
        }
        processes.clear();
    }
}
pub fn add_file_to_converting_list(input_file: &Path, output_type: &str, output_bit_rate: u32, duration_ms: u64, on_progress: impl FnMut(u32, String, u32) + Send + 'static) {
    // clone thành owned trước khi move vào thread
    let input_file_owned = input_file.to_path_buf();
    let output_type = output_type.to_string();
    let file_name = input_file_owned.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
        .to_slug();
    let thread: std::thread::JoinHandle<Result<(), String>> = std::thread::spawn(move || {
        convert_audio_async(input_file_owned.as_path(), &output_type, output_bit_rate, duration_ms, on_progress)
    });
    if let Ok(mut processes) = RUNNING_FFMPEG_PROCESSES.lock() {
        processes.insert(file_name, thread);
    }
}
pub fn remove_file_from_converting_list(file: &Path) {
    let file_slug = file
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
        .to_slug();
    if let Ok(mut processes) = RUNNING_FFMPEG_PROCESSES.lock() {
        processes.retain(|name, _| name != &file_slug);
    }
}
