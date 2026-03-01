use super::app_component_attribute::*;
#[component]
/// use aria-placeholder instead of placeholder to avoid conflicts with the actual placeholder attribute
pub fn AppInput(props: AppCommonComponentProps) -> Element {
    rsx! {
        input {
            class: props.class("px-4 py-2 border rounded"),
            placeholder: props.get_attribute_value_string("aria-placeholder", "Type here..."),
            oninput: move |evt| async move {
                if let Some(oninput) = &props.on_input {
                    oninput.call(evt);
                }
            },
            onchange: move |evt| async move {
                if let Some(onchange) = &props.on_change {
                    onchange.call(evt);
                }
            },
            onfocus: move |evt| async move {
                if let Some(onfocus) = &props.on_focus {
                    onfocus.call(evt);
                }
            },
            onblur: move |evt| async move {
                if let Some(onblur) = &props.on_blur {
                    onblur.call(evt);
                }
            },
            onkeydown: move |evt| async move {
                if let Some(onkeydown) = &props.on_keydown {
                    onkeydown.call(evt);
                }
            },
            onkeyup: move |evt| async move {
                if let Some(onkeyup) = &props.on_keyup {
                    onkeyup.call(evt);
                }
            },
            ..props.render_attribute(),
        }
    }
}
