use super::app_component_attribute::*;

#[component]
pub fn AppButton(props: AppCommonComponentProps) -> Element {
    rsx! {
        button {
            class: props
                .class(
                    "h-full p-3 inline-flex opacity-80 hover:opacity-100 transition-opacity rounded-md border border-transparent hover:border-stone-300 dark:hover:border-stone-700",
                ),
            onclick: move |evt| async move {
                if let Some(onclick) = &props.on_click {
                    onclick.call(evt);
                }
            },
            onabort: move |evt| async move {
                if let Some(onabort) = &props.on_abort {
                    onabort.call(evt);
                }
            },
            onauxclick: move |evt| async move {
                if let Some(onauxclick) = &props.on_auxclick {
                    onauxclick.call(evt);
                }
            },
            onblur: move |evt| async move {
                if let Some(onblur) = &props.on_blur {
                    onblur.call(evt);
                }
            },
            onfocus: move |evt| async move {
                if let Some(onfocus) = &props.on_focus {
                    onfocus.call(evt);
                }
            },
            ..props.render_attribute(),
            {props.children.clone()}
        }
    }
}
