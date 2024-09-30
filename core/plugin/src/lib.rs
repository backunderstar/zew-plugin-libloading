use once_cell::sync::Lazy;
use plugin_manager::PluginManager;

pub mod collect;
pub mod plugin_manager;
// pub mod binding;

pub static mut PLUGIN: Lazy<PluginManager> = Lazy::new(PluginManager::load_plugin);
