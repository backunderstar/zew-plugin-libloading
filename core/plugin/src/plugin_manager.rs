use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use interface::plugin::{CommonData, Plugin};
use libloading::{Library, Symbol};
use tracing::warn;

use crate::collect::collect_lib_dir;

pub struct PluginManager {
    pub loaded_libraries: Vec<Library>,
    pub plugins: HashMap<String, Arc<Box<dyn Plugin>>>,
    pub name: Vec<String>,
    pub data: HashMap<String, Arc<Mutex<CommonData>>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            loaded_libraries: Vec::new(),
            plugins: HashMap::new(),
            name: Vec::new(),
            data: HashMap::new(),
        }
    }

    pub fn load_plugin() -> PluginManager {
        type PluginCreate = unsafe fn() -> *mut dyn Plugin;

        let lib_dirs = collect_lib_dir("./plugin");

        let mut mp = PluginManager::new();

        for lib_dir in lib_dirs {
            let lib = match unsafe { Library::new(lib_dir) } {
                Ok(lib) => lib,
                Err(e) => {
                    warn!("Failed to load library: {:?}", e);
                    continue;
                }
            };

            mp.loaded_libraries.push(lib);

            let lib = match mp.loaded_libraries.last() {
                Some(lib) => lib,
                None => {
                    warn!("Failed to load library");
                    mp.loaded_libraries.pop();
                    continue;
                }
            };

            let constructor: Symbol<PluginCreate> = match unsafe { lib.get(b"create_plugin") } {
                Ok(sym) => sym,
                Err(e) => {
                    warn!("Failed to load symbol: {:?}", e);
                    mp.loaded_libraries.pop();
                    continue;
                }
            };

            let plugin = unsafe { Box::from_raw(constructor()) };

            // plugin.on_plugin_load();

            mp.name.push(plugin.name().to_string());

            mp.plugins
                .insert(plugin.name().to_string(), Arc::new(plugin));

            let plugin = unsafe { Box::from_raw(constructor()) };

            println!("Succeed to load plugin: {}", plugin.name());

            _ = plugin;
        }
        mp
    }

    // pub fn pre_send(&mut self, request: &mut Request) {
    //     println!("Firing pre_send hooks");

    //     for (_, plugin) in &mut self.plugins {
    //         println!("Firing pre_send for {:?}", plugin.name());
    //         plugin.pre_send(request);
    //     }
    // }

    // pub fn post_receive(&mut self, response: &mut Response) {
    //     println!("Firing post_receive hooks");

    //     for (_, plugin) in &mut self.plugins {
    //         println!("Firing post_receive for {:?}", plugin.name());
    //         plugin.post_receive(response);
    //     }
    // }

    // pub fn unload(&mut self) {
    //     println!("Unloading plugins");

    //     for (_, plugin) in &mut self.plugins {
    //         println!("Firing on_plugin_unload for {:?}", plugin.name());
    //         plugin.on_plugin_unload();
    //     }

    //     for lib in self.loaded_libraries.drain(..) {
    //         drop(lib);
    //     }
    // }
}

// impl Drop for PluginManager {
//     fn drop(&mut self) {
//         if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
//             self.unload();
//         }
//     }
// }
