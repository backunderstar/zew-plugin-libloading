use common::config::get;
use router::get_all_route;
use salvo::{catcher::Catcher, prelude::*, server::ServerHandle};
use tokio::signal;
use tracing::info;

use crate::middleware::cather_all::cather_all;
mod router;
mod plugin;

//==================================================================================
//= 初始化路由器,启动web服务器
//= Initialize the router and start the web server
pub async fn init_router() {
    // get all router
    let router = get_all_route().await;

    // Create services, using router and global logging, error capture middleware
    let service = Service::new(router)
        .catcher(Catcher::default().hoop(cather_all))
        .hoop(Logger::new());

    // Read listening address from env and bind
    let addr = format!(
        "{}:{}",
        get("server_host").unwrap_or("0.0.0.0".to_string()),
        get("server_port").unwrap_or("4321".to_string())
    );
    let acceptor = TcpListener::new(addr).bind().await;

    // console prompt
    println!(
        "==========欢迎使用 {} ========== \n==========访问地址 http://{}:{} ==========",
        "zew CMS",
        get("server_host")
            .unwrap_or("0.0.0.0".to_string())
            .replace("0.0.0.0", "127.0.0.1"),
        get("server_port").unwrap_or("4321".to_string())
    );

    // graceful shutdown
    let server = Server::new(acceptor);
    let handle = server.handle();
    tokio::spawn(listen_shutdown_signal(handle));

    info!("Server is starting...");
    println!("Server is starting...");
    // start server
    server.serve(service).await;
}

//==================================================================================
//= 优雅停机
//= graceful shutdown
async fn listen_shutdown_signal(handle: ServerHandle) {
    // Wait Shutdown Signal
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(windows)]
    let terminate = async {
        signal::windows::ctrl_c()
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => println!("ctrl_c signal received"),
        _ = terminate => println!("terminate signal received"),
    };

    // Graceful Shutdown Server
    handle.stop_graceful(None);
}
