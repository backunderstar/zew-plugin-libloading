use std::env;

use interface::error::AppResult;
use tracing::info;

//==================================================================================
//= 加载环境变量
pub fn load_env() {
    dotenvy::dotenv().expect("failed to load .env file");
    println!("Succeed to load environment variables");
    info!("Succeed to load environment variables")
}

pub fn get(str: &str) -> AppResult<String> {
    Ok(env::var(str)?)
}
