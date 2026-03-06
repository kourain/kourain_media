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