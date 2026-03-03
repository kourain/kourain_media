use std::path::{PathBuf};
pub fn get_all_files_in_directory(dir: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            files.push(path);
        }
    }
    files
}
pub fn get_all_audioable_in_directory(dir: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                match ext.to_lowercase().as_str() {
                    "mp3" | "wav" | "flac" | "aac" | "ogg" | "opus" => files.push(path),
                    _ => {}
                }
            }
        }
    }
    files
}