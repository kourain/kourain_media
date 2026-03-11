use crate::client::components::LoadingProviderState;

use super::app_component_attribute::*;
#[component]
pub fn AppLoadingScreen(props: AppCommonComponentProps) -> Element {
    let loadingstate = use_context::<LoadingProviderState>();
    let is_loading = loadingstate.is_loading;
    let cancel_action = loadingstate.cancel_action;
    let cancel_action_async = loadingstate.cancel_action_async;
    use_effect(move || {
        if !is_loading() {
            consume_context::<LoadingProviderState>()
                .cancel_action
                .set(None);
            consume_context::<LoadingProviderState>()
                .cancel_action_async
                .set(None);
        }
    });
    rsx! {
        if is_loading() {
            div { class: "w-screen h-screen top-0 left-0 fixed flex text-white bg-black/20 backdrop-blur-[1px] z-50 items-center justify-center",
                section { class: "flex flex-col items-center gap-4 bg-white dark:bg-gray-800 rounded-lg p-6",
                    Icon {
                        icon: dioxus_free_icons::icons::ld_icons::LdLoaderCircle,
                        class: "animate-spin",
                        width: 64,
                        height: 64,
                    }
                    {"Loading ..."}
                    button {
                        class: "px-4 py-2 rounded bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors",
                        onclick: move |_| async move {
                            if let Some(cancel_fn) = cancel_action() {
                                cancel_fn();
                            }
                            if let Some(cancel_fn_async) = cancel_action_async() {
                                cancel_fn_async().await;
                            }
                            consume_context::<LoadingProviderState>().is_loading.set(false);
                        },
                        "Cancel"
                    }
                }
            }
        }
    }
}
