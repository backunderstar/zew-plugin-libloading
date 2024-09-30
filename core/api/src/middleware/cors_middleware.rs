use std::env;

use salvo::cors::{AllowHeaders, AllowMethods, Cors, CorsHandler};

pub fn cors_middleware() -> CorsHandler {
    let allow_origin = env::var("server_cors_allow_origin").unwrap_or("*".to_string());

    let cors_handler = Cors::new()
        .allow_origin(&allow_origin)
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .into_handler();
    cors_handler
}
