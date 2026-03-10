use std::sync::mpsc;

pub struct ThreadHandle<T> {
    thread: std::thread::JoinHandle<T>,
    stop_signal_sender: mpsc::Sender<()>, // Dùng để giữ sender trong scope để tránh bị drop
}
impl<T> ThreadHandle<T> {
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce(mpsc::Receiver<()>) -> T + Send + 'static,
        T: Send + 'static,
    {
        let (stop_signal_sender, stop_signal_listener) = mpsc::channel();
        let thread = std::thread::spawn(move || f(stop_signal_listener));
        Self {
            thread,
            stop_signal_sender,
        }
    }
    pub fn kill(self) {
        let _ = self.stop_signal_sender.send(()); // Gửi tín hiệu dừng
    }
}
