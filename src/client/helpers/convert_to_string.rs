use dioxus::{core::IntoAttributeValue, dioxus_core::AttributeValue};
pub trait ToString {
    fn to_string(&self) -> String;
}
impl ToString for AttributeValue {
    fn to_string(&self) -> String {
        match self {
            AttributeValue::Text(s) => s.to_string(),
            AttributeValue::Float(f) => f.to_string(),
            AttributeValue::Int(i) => i.to_string(),
            AttributeValue::Bool(b) => b.to_string(),
            AttributeValue::None => String::new(),
            AttributeValue::Listener(_) => panic!("Listeners cannot be converted to strings"),
            AttributeValue::Any(v) => format!("{:?}", v.clone().into_value()), // fallback for custom values
        }
    }
}