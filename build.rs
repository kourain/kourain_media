use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
fn mod_create() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the folder containing your modules
    let module_dirs = [
        Path::new("src/client/components/_common"),
        Path::new("src/client/components/"),
        Path::new("src/client/helpers/"),
        Path::new("src/server"),
        Path::new("src/client/views/_common")
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
    writeln!(mod_file, "use dioxus::prelude::*;\npub struct ASSETS;\nimpl ASSETS {{")?;
    for entry in fs::read_dir(asset_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                match ext.to_string_lossy().as_ref() {
                    "png" | "ico" | "svg" => {
                        if let Some(file_name) = path.file_name() {
                            writeln!(mod_file, "    pub const {}: Asset = asset!(\"assets/{}\");",file_name.to_string_lossy().replace('.', "_").to_uppercase(), file_name.to_string_lossy())?;
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
fn main() -> io::Result<()> {
    _ = mod_create();
    _ = asset_create();
    Ok(())
}
