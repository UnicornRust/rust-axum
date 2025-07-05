use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};


pub fn init() {
    // 日志通过不同的层叠加来实现功能的组合
    tracing_subscriber::registry()
        // 从默认的环境变量加载日志过滤器
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        // 将日志输出到控制台
        .with(tracing_subscriber::fmt::layer()
            // 文件名(显示)
            .with_file(true)
            // 行号(显示)
            .with_line_number(true)
            // 线程ID (显示)
            .with_thread_ids(true)
            // 线程名称(显示)
            .with_thread_names(true)
            // 当前构建目标程序的名称(不显示)
            .with_target(false)
        ).init()
}
