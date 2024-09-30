use std::{fs::read_dir, path::PathBuf};

use tracing::info;

pub fn collect_lib_dir(dir: &str) -> Vec<PathBuf> {
    let mut libs = Vec::new();
    if let Ok(plugin_root_dir) = read_dir(dir) {
        for plugin_dir in plugin_root_dir.flatten() {
            if let Ok(metadata) = plugin_dir.metadata() {
                if metadata.is_dir() {
                    match plugin_dir.path().to_str() {
                        Some(path_str) => libs.extend(collect_lib_dir(path_str)),
                        None => {
                            info!("Failed to convert path to string")
                        }
                    }
                } else if is_lib(plugin_dir.path()) {
                    libs.push(plugin_dir.path());
                    println!("Found plugin: {:?}", plugin_dir.path())
                }
            }
        }
    }
    libs
}

pub fn is_lib(path: PathBuf) -> bool {
    path.extension()
        .map_or(false, |ext| ext == "so" || ext == "dll")
}
