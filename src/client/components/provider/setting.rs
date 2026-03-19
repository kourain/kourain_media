pub use dioxus::prelude::*;

#[derive(Clone)]
pub struct SettingProviderState {
    pub default_path: Signal<String>,
    pub default_selected_format: Signal<String>,
    pub default_bit_rate: Signal<u32>,
    pub default_sample_rate: Signal<u32>,
    pub default_channels: Signal<u32>,
    pub default_max_instances: Signal<u32>,
}

const SETTING_FILE_NAME: &str = ".kourain/kourain_media.json";

fn load_setting_file() -> serde_json::Value {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_default();
    let path = std::path::Path::new(&home).join(SETTING_FILE_NAME);
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or(serde_json::Value::Object(serde_json::Map::new()))
}

pub fn use_setting_provider() -> SettingProviderState {
    let json = load_setting_file();
    let default_path = use_signal(|| {
        json.get("default_path")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    });
    let default_selected_format = use_signal(|| {
        json.get("default_selected_format")
            .and_then(|v| v.as_str())
            .unwrap_or("aac")
            .to_string()
    });
    let default_bit_rate = use_signal(|| {
        json.get("default_bit_rate")
            .and_then(|v| v.as_u64())
            .unwrap_or(48) as u32
    });
    let default_sample_rate = use_signal(|| {
        json.get("default_sample_rate")
            .and_then(|v| v.as_u64())
            .unwrap_or(22050) as u32
    });
    let default_channels = use_signal(|| {
        json.get("default_channels")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as u32
    });
    let default_max_instances = use_signal(|| {
        json.get("default_max_instances")
            .and_then(|v| v.as_u64())
            .unwrap_or(4) as u32
    });
    use_effect(move || {
        let setting = serde_json::json!({
            "default_path": default_path(),
            "default_selected_format": default_selected_format(),
            "default_bit_rate": default_bit_rate(),
            "default_sample_rate": default_sample_rate(),
            "default_channels": default_channels(),
            "default_max_instances": default_max_instances(),
        });
        let home = std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .unwrap_or_default();
        let path = std::path::Path::new(&home).join(SETTING_FILE_NAME);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        std::fs::write(path, serde_json::to_string_pretty(&setting).unwrap()).ok();
    });
    use_context_provider(|| SettingProviderState {
        default_path,
        default_selected_format,
        default_bit_rate,
        default_sample_rate,
        default_channels,
        default_max_instances,
    })
}