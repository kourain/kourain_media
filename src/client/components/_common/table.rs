use super::app_component_attribute::*;
#[component]
pub fn AppTable(props: AppCommonComponentProps) -> Element {
    rsx! {
        table {
            class: props
                .class(
                    "border overflow-hidden rounded-md border-stone-200 dark:border-gray-700 p-2 bg-[#0d0d0d]",
                ),
            onclick: move |evt| async move {
                if let Some(onclick) = &props.onclick {
                    onclick.call(evt);
                }
            },
            onabort: move |evt| async move {
                if let Some(onabort) = &props.onabort {
                    onabort.call(evt);
                }
            },
            onauxclick: move |evt| async move {
                if let Some(onauxclick) = &props.onauxclick {
                    onauxclick.call(evt);
                }
            },
            onblur: move |evt| async move {
                if let Some(onblur) = &props.onblur {
                    onblur.call(evt);
                }
            },
            onfocus: move |evt| async move {
                if let Some(onfocus) = &props.onfocus {
                    onfocus.call(evt);
                }
            },
            onkeydown: move |evt| async move {
                if let Some(onkeydown) = &props.onkeydown {
                    onkeydown.call(evt);
                }
            },
            onkeyup: move |evt| async move {
                if let Some(onkeyup) = &props.onkeyup {
                    onkeyup.call(evt);
                }
            },
            ..props.render_attribute(),
            {props.children.clone()}
        }
    }
}