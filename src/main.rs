use api::router::init_router;
use common::{config, log};

//==================================================================================
//= 使用 mimalloc 内存分配器
//= use mimalloc memory allocator
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    //==================================================================================
    //= 初始化配置、环境变量
    //= init config or env
    config::load_env();

    //==================================================================================
    //= 初始化日志
    //= init logger
    let _guard = log::init_logger().expect("failed to initialize logger");

    //==================================================================================
    //= 加载插件，改成延迟初始化单例，仍然不行
    //= load plugin

    //==================================================================================
    //= 初始化路由，启动web服务
    //= init router and start web server
    init_router().await;
}
