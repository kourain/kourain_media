use super::app_component_attribute::*;

#[component]
pub fn AppContainer(props: AppCommonComponentProps) -> Element {
    rsx! {
        div {
            class: props
                .class(
                    "border overflow-hidden rounded-md border-stone-200 dark:border-gray-700 p-2 bg-[#0d0d0d]",
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
