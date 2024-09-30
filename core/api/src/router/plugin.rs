use salvo::Router;

use crate::handler::plugin::plugin_get;

pub fn get_plugin_router() -> Router {
    Router::with_path("/plugin/<*+>").get(plugin_get)
}
