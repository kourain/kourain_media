pub trait FormatNumber {
    fn format_file_size(&self) -> String;
    /// Định dạng thời lượng từ milliseconds sang HH:MM:SS hoặc MM:SS
    fn format_duration(&self) -> String;
}
impl FormatNumber for u64 {
    fn format_file_size(&self) -> String {
        let mut size = *self as f64;
        for suffix in ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"] {
            if size < 1024.0 {
                return format!("{:.2} {}", size, suffix);
            }
            size /= 1024.0;
        }
        format!("{:.2} B", size)
    }
    fn format_duration(&self) -> String {
        let total_seconds = *self / 1000;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }
}
