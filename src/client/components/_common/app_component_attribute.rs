pub use crate::helpers::*;
pub use dioxus::prelude::*;
pub use dioxus_free_icons::*;
#[derive(Props, Clone, PartialEq)]
pub struct AppCommonComponentProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
    //sub props
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub placeholder: Option<String>,
    #[props(default)]
    pub value: Option<String>,
    #[props(default)]
    pub r#type: Option<String>,
    // Event handlers
    // Form events
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    #[props(default)]
    pub onsubmit: Option<EventHandler<FormEvent>>,
    // media
    #[props(default)]
    pub onabort: Option<EventHandler<dioxus::prelude::Event<dioxus::events::MediaData>>>,
    #[props(default)]
    pub onauxclick: Option<EventHandler<dioxus::prelude::Event<dioxus::events::PointerData>>>,
    // Focus events
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,
    #[props(default)]
    //keyboard events
    pub onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(default)]
    pub onkeyup: Option<EventHandler<KeyboardEvent>>,
}
impl AppCommonComponentProps {
    pub fn class(&self, default: &str) -> String {
        format!(
            "{} {}",
            default,
            self.attributes
                .iter()
                .find(|attr| attr.name == "class")
                .map(|attr| attr.value.to_string())
                .unwrap_or_else(|| "".to_string())
        )
    }
    pub fn get_attribute_value_string(&self, name: &str, default: &str) -> String {
        self.attributes
            .iter()
            .find(|attr| attr.name == name)
            .map(|attr| attr.value.to_string())
            .unwrap_or_else(|| default.to_string())
    }
    pub fn render_attribute(&self) -> Vec<Attribute> {
        self.attributes
            .iter()
            .filter(|attr| attr.name != "class" && !attr.name.starts_with("on_"))
            .cloned()
            .collect()
    }
}