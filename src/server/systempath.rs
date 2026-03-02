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