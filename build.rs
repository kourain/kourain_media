use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Write};
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
        println!("cargo:rerun-if-changed={}", module_dir.display());

        // Collect all `.rs` files except mod.rs
        let mut mods = HashSet::new();
        for entry in fs::read_dir(module_dir)? {
            let entry = entry?;
            let path = entry.path();

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

        // Generate mod.rs content
        let mut mod_file = File::create(module_dir.join("mod.rs"))?;
        for m in sorted_mods {
            writeln!(mod_file, "mod {};\npub use {}::*;", m, m)?;
        }
    }
    Ok(())
}
fn asset_create() -> Result<(), Box<dyn std::error::Error>> {
    let asset_dir = Path::new("assets");
    println!("cargo:rerun-if-changed={}", asset_dir.display());
    let mut mod_file = File::create(Path::new(r"src/client/components/_common/assets.rs"))?;
    writeln!(
        mod_file,
        "use dioxus::prelude::*;\npub struct ASSETS;\nimpl ASSETS {{"
    )?;
    for entry in fs::read_dir(asset_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                match ext.to_string_lossy().as_ref() {
                    "png" | "ico" | "svg" => {
                        if let Some(file_name) = path.file_name() {
                            writeln!(
                                mod_file,
                                "    pub const {}: Asset = asset!(\"assets/{}\");",
                                file_name.to_string_lossy().replace('.', "_").to_uppercase(),
                                file_name.to_string_lossy()
                            )?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    writeln!(mod_file, "}}")?;
    Ok(())
}
fn auto_route(base_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let views_dir = Path::new(base_path);
    println!("cargo:rerun-if-changed={}", views_dir.display());
    let mut mod_file = File::create(views_dir.join("mod.rs"))?;
    writeln!(mod_file, "use dioxus::prelude::*;")?;
    let mut views = HashSet::new();
    let paths = fs::read_dir(views_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    let mut sorted_paths = paths;
    sorted_paths.sort();
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
                            writeln!(
                                mod_file,
                                "mod {};\npub use {}::*;",
                                file_name.to_string_lossy(),
                                file_name.to_string_lossy()
                            )?;
                            views.insert((
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
                writeln!(
                    mod_file,
                    "mod {};\npub use {}::*;",
                    file_name.to_string_lossy(),
                    file_name.to_string_lossy()
                )?;
            }
        }
    }
    writeln!(
        mod_file,
        "#[derive(Debug, Clone, Routable, PartialEq)]\n#[rustfmt::skip]\npub enum Route {{\n    #[layout(Layout)]"
    )?;
    for page in views {
        if page.0.is_empty() || page.0 == "home" {
            writeln!(mod_file, "        #[route(\"/\")]\n        {},", page.1)?;
        } else {
            writeln!(
                mod_file,
                "        #[route(\"/{}\")]\n        {},",
                page.0.replace('\\', "/"),
                page.1
            )?;
        }
    }
    writeln!(mod_file, "}}")?;
    Ok(())
}
fn main() -> io::Result<()> {
    _ = mod_create();
    _ = asset_create();
    _ = auto_route("src/client/views");
    Ok(())
}
