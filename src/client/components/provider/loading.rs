pub use dioxus::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
#[derive(Clone)]
pub struct LoadingProviderState {
    pub is_loading: Signal<bool>,
    pub cancel_action: Signal<Option<Arc<dyn Fn() + Send + Sync>>>,
    pub cancel_action_async: Signal<Option<Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>>>,
}
pub fn use_loading_provider() -> LoadingProviderState {
    let is_loading = use_signal(|| false);
    let cancel_action = use_signal(|| None);
    let cancel_action_async = use_signal(|| None);
    use_context_provider(|| LoadingProviderState {
        is_loading,
        cancel_action,
        cancel_action_async,
    })
}