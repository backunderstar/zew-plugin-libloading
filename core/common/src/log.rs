use std::env;

use interface::error::AppResult;
use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    fmt::{self, time},
    layer::SubscriberExt,
    Layer,
};

use crate::{config::get, utils::env_conversion::Convert};

pub fn init_logger() -> AppResult<tracing_appender::non_blocking::WorkerGuard> {
    //==================================================================================
    //= 日志输出到控制台
    //= Log output to console
    let console_layer = fmt::layer()
        .with_ansi(get("log_with_ansi")?.to_bool().unwrap_or(true))
        .with_writer(std::io::stdout)
        .with_line_number(true)
        .with_timer(time::ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
        .with_filter(match get("log_level")?.as_str() {
            "trace" => LevelFilter::TRACE,
            "debug" => LevelFilter::DEBUG,
            "info" => LevelFilter::INFO,
            "warn" => LevelFilter::WARN,
            "error" => LevelFilter::ERROR,
            _ => LevelFilter::INFO,
        });

    //==================================================================================
    //= 日志输出到文件
    //= Log output to file
    let file_appender = RollingFileAppender::new(
        match get("log_rolling")?.as_str() {
            "daily" => Rotation::DAILY,
            "hourly" => Rotation::HOURLY,
            "minutely" => Rotation::MINUTELY,
            "never" => Rotation::NEVER,
            _ => Rotation::DAILY,
        },
        get("log_directory")?,
        get("log_file_name")?,
    );
    //= 创建非阻塞的异步文件日志记录器
    //= Create a non-blocking async file logger
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_line_number(true)
        .with_timer(time::ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
        .json()
        .with_filter(
            match env::var("log_level").unwrap_or("info".to_string()).as_str() {
                "trace" => LevelFilter::TRACE,
                "debug" => LevelFilter::DEBUG,
                "info" => LevelFilter::INFO,
                "warn" => LevelFilter::WARN,
                "error" => LevelFilter::ERROR,
                _ => LevelFilter::INFO,
            },
        );

    //==================================================================================
    //= 选择注册控制台和文件日志记录器,并设置全局
    //= Select the console and file log recorder and set the global
    if get("log_to_stdout")?.to_bool().unwrap_or(false) {
        let collector = tracing_subscriber::registry()
            .with(file_layer)
            .with(console_layer);
        tracing::subscriber::set_global_default(collector).expect("Tracing collect error");
    } else {
        let collector = tracing_subscriber::registry().with(file_layer);
        tracing::subscriber::set_global_default(collector).expect("Tracing collect error");
    }

    //==================================================================================
    //= 记录并测试
    //= Record and test
    tracing::info!("Logger initialized");
    println!("Logger initialized");

    Ok(guard)
}
