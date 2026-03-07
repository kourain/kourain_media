use std::process::{Command, Child};
pub trait CommandExt {
    fn hide_console(&mut self);
}
pub trait ChildExt {
    fn is_alive(&mut self) -> bool;
}
impl CommandExt for Command {
    fn hide_console(&mut self) {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            self.creation_flags(0x08000000); // CREATE_NO_WINDOW
        }
    }
}
impl ChildExt for Child {
    fn is_alive(&mut self) -> bool {
        match self.try_wait() {
            Ok(Some(_)) => false, // Process đã kết thúc
            Ok(None) => true,     // Process vẫn đang chạy
            Err(_) => false,      // Lỗi khi kiểm tra trạng thái process
        }
    }
}
pub fn open_windows_explorer(path: &str) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer").arg(path).spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg(path).spawn()?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(path).spawn()?;
    }
    Ok(())
}