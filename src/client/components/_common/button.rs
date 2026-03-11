use super::app_component_attribute::*;

#[component]
pub fn AppButton(props: AppCommonComponentProps) -> Element {
    rsx! {
        button {
            class: props
                .class(
                    "text-white inline-flex opacity-80 hover:opacity-100 transition-opacity rounded-md border border-transparent hover:border-stone-300 dark:hover:border-stone-700",
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
            disabled: props.disabled,
            ..props.render_attribute(),
            {props.children.clone()}
        }
    }
}
