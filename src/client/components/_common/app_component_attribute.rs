pub use super::super::super::helpers::*;
pub use dioxus::prelude::*;
pub use dioxus_free_icons::*;
#[derive(Props, Clone, PartialEq)]
pub struct AppCommonComponentProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
    // Event handlers
    // Form events
    #[props(default)]
    pub on_input: Option<EventHandler<FormEvent>>,
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub on_change: Option<EventHandler<FormEvent>>,
    #[props(default)]
    pub on_submit: Option<EventHandler<FormEvent>>,
    // media
    #[props(default)]
    pub on_abort: Option<EventHandler<dioxus::prelude::Event<dioxus::events::MediaData>>>,
    #[props(default)]
    pub on_auxclick: Option<EventHandler<dioxus::prelude::Event<dioxus::events::PointerData>>>,
    // Focus events
    #[props(default)]
    pub on_focus: Option<EventHandler<FocusEvent>>,
    #[props(default)]
    pub on_blur: Option<EventHandler<FocusEvent>>,
    #[props(default)]
    //keyboard events
    pub on_keydown: Option<EventHandler<KeyboardEvent>>,
    #[props(default)]
    pub on_keyup: Option<EventHandler<KeyboardEvent>>,
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
