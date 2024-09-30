use std::sync::{Arc, Mutex};

use interface::writer::ResponseBuilder;
use plugin::PLUGIN;
use salvo::{handler, Depot, Request, Response};

#[handler]
pub async fn plugin_get(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let plugin = match unsafe { PLUGIN.plugins.get("Example") }.clone() {
        Some(plugin) => plugin,
        None => return res.render("Plugin not found"),
    };

    let common_data = plugin.plugin_get(req, depot, res);

    unsafe {
        PLUGIN
            .data
            .insert("Example".to_string(), Arc::new(Mutex::new(common_data)))
    };

    {
        let common_data = unsafe { PLUGIN.data.get("Example").unwrap().lock().unwrap() };

        ResponseBuilder::with_data(common_data.clone()).into_response(res);
    }
}
