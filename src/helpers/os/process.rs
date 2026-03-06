use std::process::{Child};
pub struct SubProcess {
    child: Child,
}

impl Drop for SubProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait(); // Dọn dẹp zombie process
    }
}
