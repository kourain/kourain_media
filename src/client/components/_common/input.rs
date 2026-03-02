use super::app_component_attribute::*;
#[component]
pub fn AppInput(props: AppCommonComponentProps) -> Element {
    let placeholder = props
        .placeholder
        .clone()
        .unwrap_or_else(|| props.get_attribute_value_string("aria_placeholder", "Type here..."));
    let value = props
        .value
        .clone()
        .unwrap_or_else(|| props.get_attribute_value_string("value", ""));
    let r#type = props
        .r#type
        .clone()
        .unwrap_or_else(|| props.get_attribute_value_string("type", "text"));

    rsx! {
        input {
            class: props.class("px-4 py-2 border rounded bg-white"),
            disabled: props.disabled,
            r#type,
            placeholder,
            value,
            oninput: move |evt| async move {
                if let Some(oninput) = &props.oninput {
                    oninput.call(evt);
                }
            },
            onchange: move |evt| async move {
                if let Some(onchange) = &props.onchange {
                    onchange.call(evt);
                }
            },
            onfocus: move |evt| async move {
                if let Some(onfocus) = &props.onfocus {
                    onfocus.call(evt);
                }
            },
            onblur: move |evt| async move {
                if let Some(onblur) = &props.onblur {
                    onblur.call(evt);
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
        }
    }
}
