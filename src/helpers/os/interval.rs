use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

pub fn interval<F>(interval: u32, mut callback: F)
where
    F: FnMut(&dyn Fn()),
{
    let stopped = Arc::new(AtomicBool::new(false));
    let stopped_clone = Arc::clone(&stopped);

    let stop = move || {
        stopped_clone.store(true, Ordering::Relaxed);
    };

    while !stopped.load(Ordering::Relaxed) {
        std::thread::sleep(std::time::Duration::from_millis(interval as u64));
        callback(&stop);
    }
}
