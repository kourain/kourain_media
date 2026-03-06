use dioxus::prelude::*;
use tokio::time::Duration;

pub async fn use_interval<F>(interval: u64,mut callback: F)
where
    F: FnMut(Signal<bool>) + 'static,
{
    let stopped = use_signal(|| false);

    loop {
        if stopped() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(interval)).await;
        // std::thread::sleep(std::time::Duration::from_millis(interval));
         callback(stopped);
    }
    // let callback = use_resource(move || async move {
    //         if stopped() {
    //             break;
    //         }
    //         callback(stopped);
    //     }
    // });
}
