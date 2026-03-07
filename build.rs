use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
//helper
fn to_camel_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}
fn sort_by_file_stem(mut paths: Vec<std::path::PathBuf>) -> Vec<std::path::PathBuf> {
    paths.sort_by(|a, b| {
        a.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
            .to_lowercase()
            .cmp(
                &b.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string().to_lowercase(),
            )
    });
    paths
}
fn mod_create() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the folder containing your modules
    let module_dirs = [
        Path::new("src/client/components/_common"),
        Path::new("src/client/components/"),
        Path::new("src/helpers/"),
        Path::new("src/server"),
        Path::new("src/client/views/_common"),
    ];

    for module_dir in module_dirs {
        // Tell Cargo to rerun build.rs if any file in the folder changes

        // Collect all `.rs` files except mod.rs
        let mut mods = HashSet::new();
        let paths = fs::read_dir(module_dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        let mut sorted_paths = sort_by_file_stem(paths);
        for path in sorted_paths {
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        if let Some(file_name) = path.file_stem() {
                            if file_name != "mod" {
                                mods.insert(file_name.to_string_lossy().into_owned());
                            }
                        }
                    }
                }
            }
            if path.is_dir() {
                if let Some(file_name) = path.file_name() {
                    mods.insert(file_name.to_string_lossy().into_owned());
                }
            }
        }

        // Sort for consistent output
        let mut sorted_mods: Vec<String> = mods.into_iter().collect();
        sorted_mods.sort();
        let mut content = String::new();
        for m in sorted_mods {
            content.push_str(&format!("mod {};\npub use {}::*;\n", m, m));
        }
        if let Ok(existing_content) = fs::read_to_string(module_dir.join("mod.rs")) {
            if existing_content != content {
                // Only write if content has changed to avoid unnecessary rebuilds
                // Generate mod.rs content
                eprintln!("cargo:rerun-if-changed={}", module_dir.display());
                let mut mod_file = File::create(module_dir.join("mod.rs"))?;
                mod_file.write_all(content.as_bytes())?;
            }
        }
    }
    Ok(())
}
fn asset_create() -> Result<(), Box<dyn std::error::Error>> {
    let asset_dir = Path::new("assets");
    let mut str_builder: String = String::new();
    str_builder.push_str("use dioxus::prelude::*;\npub struct ASSETS;\nimpl ASSETS {\n");
    let paths = fs::read_dir(asset_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    let mut sorted_paths = sort_by_file_stem(paths);
    for path in sorted_paths {
        if path.is_file() {
            if let Some(ext) = path.extension() {
                match ext.to_string_lossy().as_ref() {
                    "png" | "ico" | "svg" => {
                        if let Some(file_name) = path.file_name() {
                            str_builder.push_str(&format!(
                                "    pub const {}: Asset = asset!(\"assets/{}\");\n",
                                file_name.to_string_lossy().replace('.', "_").to_uppercase(),
                                file_name.to_string_lossy()
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    str_builder.push_str("}\n");
    if let Ok(existing_content) = fs::read_to_string("src/client/components/_common/assets.rs") {
        if existing_content != str_builder {
            let mut mod_file = File::create(Path::new(r"src/client/components/_common/assets.rs"))?;
            eprintln!("cargo:rerun-if-changed={}", asset_dir.display());
            mod_file.write_all(str_builder.as_bytes())?;
        }
    }
    Ok(())
}
fn auto_route(base_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let views_dir = Path::new(base_path);
    let mut str_builder: String = String::new();
    str_builder.push_str("use dioxus::prelude::*;\n");
    let mut views = Vec::new();
    let paths = fs::read_dir(views_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    let mut sorted_paths = sort_by_file_stem(paths);
    for path in sorted_paths {
        let path = path;
        let relative_file_path_without_ext = path
            .strip_prefix("src/client/views")
            .unwrap()
            .with_extension("");
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "rs" {
                    if let Some(file_name) = path.file_stem() {
                        if file_name != "mod" {
                            str_builder.push_str(&format!(
                                "mod {};\npub use {}::*;\n",
                                file_name.to_string_lossy(),
                                file_name.to_string_lossy()
                            ));
                            views.push((
                                relative_file_path_without_ext.to_string_lossy().to_string(),
                                to_camel_case(&file_name.to_string_lossy().to_string()),
                            ));
                        }
                    }
                }
            }
        }
        if path.is_dir() {
            if let Some(file_name) = path.file_name() {
                str_builder.push_str(&format!(
                    "mod {};\npub use {}::*;\n",
                    file_name.to_string_lossy(),
                    file_name.to_string_lossy()
                ));
            }
        }
    }
    str_builder.push_str("#[derive(Debug, Clone, Routable, PartialEq)]\n#[rustfmt::skip]\npub enum Route {\n    #[layout(Layout)]\n");
    for page in views {
        if page.0.is_empty() || page.0 == "home" {
            str_builder.push_str(&format!("        #[route(\"/\")]\n        {},\n", page.1));
        } else {
            str_builder.push_str(&format!(
                "        #[route(\"/{}\")]\n        {},\n",
                page.0.replace('\\', "/"),
                page.1
            ));
        }
    }
    str_builder.push_str("}\n");
    if let Ok(existing_content) = fs::read_to_string(views_dir.join("mod.rs")) {
        if existing_content != str_builder {
            let mut mod_file = File::create(views_dir.join("mod.rs"))?;
            eprintln!("cargo:rerun-if-changed={}", views_dir.display());
            mod_file.write_all(str_builder.as_bytes())?;
        }
    }
    Ok(())
}
fn main() -> io::Result<()> {
    eprintln!("cargo:rerun-if-changed=build.rs");
    _ = mod_create();
    _ = asset_create();
    _ = auto_route("src/client/views");
    Ok(())
}
