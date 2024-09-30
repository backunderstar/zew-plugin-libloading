use interface::error::AppResult;
use salvo::{Request, Response};

use crate::plugin_manager::PluginManager;

pub fn plugin_manager_new() -> PluginManager {
    PluginManager::new()
}

/// Destroy a `PluginManager` once you are done with it.
pub fn plugin_manager_destroy(pm: &mut PluginManager) {
    let _ = pm;
}

/// Unload all loaded plugins.
pub fn plugin_manager_unload(pm: &mut PluginManager) {
    pm.unload();
}

/// Fire the `pre_send` plugin hooks.
pub fn plugin_manager_pre_send(pm: &mut PluginManager, request: &mut Request) {
    pm.pre_send(request);
}

/// Fire the `post_receive` plugin hooks.
pub fn plugin_manager_post_receive(pm: &mut PluginManager, response: &mut Response) {
    pm.post_receive(response);
}

pub fn plugin_manager_load_plugin(pm: &mut PluginManager, filename: String) -> AppResult<()> {
    // TODO: proper error handling and catch_unwind
    unsafe { pm.load_plugin(filename) }
}
