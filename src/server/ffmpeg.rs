use crate::helpers::*;
use kourain_core::ToSlug;
use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;
use std::process::Stdio;
use std::sync::LazyLock;
use std::sync::mpsc::{Receiver, TryRecvError};
fn create_audio_encoder(output_type: &str) -> &str {
    match output_type {
        "opus" => "libopus",
        "vorbis" | "ogg" => "libvorbis",
        "mp3" => "libmp3lame",
        "aac" => "aac",
        "flac" => "flac",
        "wav" => "pcm_s16le",
        _ => format!("lib{}", output_type).leak(),
    }
}

fn create_audio_encoder_args(output_type: &str, output_bit_rate: u32) -> Vec<String> {
    let encoder = create_audio_encoder(output_type);
    let mut args = vec!["-c:a".to_string(), encoder.to_string()];
    match output_type {
        "opus" => {
            args.extend([
                "-b:a".to_string(),
                format!("{}k", output_bit_rate),
                "-vbr".to_string(),
                "on".to_string(),
                "-compression_level".to_string(),
                "0".to_string(),
                "-application".to_string(),
                "voip".to_string(),
            ]);
        }
        "flac" => {
            args.extend(["-compression_level".to_string(), "0".to_string()]);
        }
        "wav" => {}
        _ => {
            args.extend(["-b:a".to_string(), format!("{}k", output_bit_rate)]);
        }
    }
    args
}
pub fn get_audio_bit_rate_ffprobe(path: &Path) -> Option<u32> {
    let mut cmd = std::process::Command::new("ffprobe");

    cmd.hide_console();

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
static FFMPEG_RUNNING_PROCESSES: LazyLock<
    std::sync::Mutex<HashMap<String, ThreadHandle<Result<(), String>>>>,
> = LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));
static FFMPEG_CONVERTING_PROGRESS: LazyLock<std::sync::Mutex<HashMap<String, i32>>> =
    LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));
static CURRENT_FFMPEG_INSTANCES: LazyLock<std::sync::Mutex<u32>> =
    LazyLock::new(|| std::sync::Mutex::new(0));
static MAX_FFMPEG_INSTANCES: LazyLock<std::sync::Mutex<u32>> =
    LazyLock::new(|| std::sync::Mutex::new(4));
/// Eg: `convert_audio(input, "opus", 128, |progress| { println!("{}%", progress); })`
fn convert_audio_async(
    input_file: &Path,
    output_type: &str,
    output_bit_rate: u32,
    duration_ms: u64,
    kill_signal: Receiver<()>,
) -> Result<(), String> {
    // Tạo thư mục output nếu chưa có
    let output_file = input_file
        .parent()
        .unwrap_or(Path::new("."))
        .join(output_type)
        .join(input_file.file_stem().unwrap_or_default())
        .with_extension(output_type);

    if let Some(parent) = output_file.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("create ouput folder fail: {}", e))?;
    }
    loop {
        // Check kill signal while waiting
        match kill_signal.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                return Err("ffmpeg process killed while waiting".to_string());
            }
            Err(TryRecvError::Empty) => {}
        }
        if CURRENT_FFMPEG_INSTANCES
            .lock()
            .ok()
            .map(|count| *count)
            .unwrap_or(0)
            < MAX_FFMPEG_INSTANCES
                .lock()
                .ok()
                .map(|max| *max)
                .unwrap_or(4)
        {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    *CURRENT_FFMPEG_INSTANCES.lock().unwrap() += 1;
    // Chạy ffmpeg với progress report
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.hide_console();
    let mut ffmpeg_args: Vec<String> = vec![
        "-i".to_string(),
        input_file.to_str().unwrap_or("").to_string(),
        "-ac".to_string(),
        "1".to_string(),
    ];
    ffmpeg_args.extend(create_audio_encoder_args(output_type, output_bit_rate));
    ffmpeg_args.extend([
        "-progress".to_string(),
        "pipe:1".to_string(),
        output_file.to_str().unwrap_or("").to_string(),
        "-y".to_string(),
    ]);
    let mut child = cmd
        .args(&ffmpeg_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("start ffmpeg fail: {}", e))?;
    let stdout = child.stdout.take().ok_or("error: stdout")?;
    let reader = std::io::BufReader::new(stdout);
    let mut last_progress = 0i32;
    FFMPEG_CONVERTING_PROGRESS.lock().ok().map(|mut map| {
        map.insert(
            input_file
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                .to_slug()
                .sub_string(0, 50),
            0,
        );
    });
    for line in reader.lines() {
        match kill_signal.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Terminating.");
                let _ = child.kill();
                let _ = child.wait(); // Reap the killed process
                *CURRENT_FFMPEG_INSTANCES.lock().unwrap() -= 1;
                FFMPEG_CONVERTING_PROGRESS.lock().ok().map(|mut map| {
                    map.insert(
                        input_file
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string()
                            .to_slug()
                            .sub_string(0, 50),
                        -1,
                    );
                });
                return Err("ffmpeg process killed".to_string());
            }
            Err(TryRecvError::Empty) => {
                // print!("ffmpeg progress: {}\n", CURRENT_FFMPEG_INSTANCES.lock().unwrap());
            }
        }
        if let Ok(line) = line {
            if line.starts_with("out_time_ms=") {
                if let Ok(time_us) = line
                    .strip_prefix("out_time_ms=")
                    .unwrap_or("")
                    .parse::<u64>()
                {
                    let time_ms = time_us / 1000;
                    let progress = ((time_ms * 100) / duration_ms.max(1)).min(100) as i32;
                    if progress != last_progress {
                        last_progress = progress;
                        FFMPEG_CONVERTING_PROGRESS.lock().ok().map(|mut map| {
                            map.insert(
                                input_file
                                    .file_stem()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string()
                                    .to_slug()
                                    .sub_string(0, 50),
                                progress,
                            );
                        });
                    }
                }
            }
        }
    }

    let status = child.wait().map_err(|e| format!("err: {}", e))?;
    *CURRENT_FFMPEG_INSTANCES.lock().unwrap() -= 1;
    FFMPEG_RUNNING_PROCESSES.lock().ok().map(|mut map| {
        map.remove(
            &input_file
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                .to_slug()
                .sub_string(0, 50),
        );
    });
    if status.success() {
        FFMPEG_CONVERTING_PROGRESS.lock().ok().map(|mut map| {
            map.insert(
                input_file
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
                    .to_slug()
                    .sub_string(0, 50),
                100,
            );
        });
        Ok(())
    } else {
        Err(format!("ffmpeg fail code: {}", status.code().unwrap_or(-1)))
    }
}
pub fn is_all_converting_finished() -> bool {
    println!(
        "Number of running FFMPEG processes: {}",
        FFMPEG_RUNNING_PROCESSES
            .lock()
            .ok()
            .map(|map| map.len())
            .unwrap_or(0)
    );
    FFMPEG_RUNNING_PROCESSES
        .lock()
        .ok()
        .map(|map| map.is_empty())
        .unwrap_or(true)
}
pub fn get_converting_progress() -> HashMap<String, i32> {
    FFMPEG_CONVERTING_PROGRESS
        .lock()
        .ok()
        .map(|map| map.clone())
        .unwrap_or_default()
}
pub fn kill_all_ffmpeg_processes() {
    if let Ok(mut processes) = FFMPEG_RUNNING_PROCESSES.lock() {
        for (_, handle) in processes.drain() {
            handle.kill();
        }
    }
}
pub fn set_max_ffmpeg_instances(max: u32) {
    *MAX_FFMPEG_INSTANCES.lock().unwrap() = max;
}
pub fn add_file_to_converting_list(
    input_file: &Path,
    output_type: &str,
    output_bit_rate: u32,
    duration_ms: u64,
) {
    // clone thành owned trước khi move vào thread
    let input_file_owned = input_file.to_path_buf();
    let output_type = output_type.to_string();
    let file_name = input_file_owned
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
        .to_slug()
        .sub_string(0, 50);
    if let Ok(mut processes) = FFMPEG_RUNNING_PROCESSES.lock() {
        processes.insert(
            file_name,
            ThreadHandle::spawn(move |kill_signal| {
                convert_audio_async(
                    &input_file_owned,
                    &output_type,
                    output_bit_rate,
                    duration_ms,
                    kill_signal,
                )
            }),
        );
    }
}
