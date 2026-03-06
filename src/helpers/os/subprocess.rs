pub struct Subprocess {
    child: std::process::Child,
    is_killed: bool,
}
impl Drop for Subprocess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait(); // Dọn dẹp zombie process
    }
}
impl Subprocess {
    pub fn new(child: std::process::Child) -> Self {
        Self {
            child,
            is_killed: false,
        }
    }
    pub fn child(&self) -> &std::process::Child {
        &self.child
    }
    pub fn kill(&mut self) {
        let _ = self.child.kill();
        self.is_killed = true;
    }
    pub fn run(
        &mut self,
        f: impl FnOnce(&mut std::process::Child, bool),
    ) -> std::io::Result<std::process::ExitStatus> {
        f(&mut self.child, self.is_killed);
        self.child.wait()
    }
    pub fn is_alive(&mut self) -> bool {
        match self.child.try_wait() {
            Ok(Some(_)) => false, // Process đã kết thúc
            Ok(None) => true,     // Process vẫn đang chạy
            Err(_) => false,      // Lỗi khi kiểm tra trạng thái process
        }
    }
}
