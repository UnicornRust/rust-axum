pub mod server;

use std::sync::LazyLock;

use anyhow::Context;
use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;
use server::ServerConfig;


// 懒加载(到静态变量, 全局共享)
static CONFIG: LazyLock<AppConfig> = LazyLock::new(||AppConfig::load().expect("Fail to initialize config"));


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {

    pub fn load() -> anyhow::Result<Self> {
        Config::builder()
            .add_source(
                File::with_name("application")
                    .format(FileFormat::Yaml)
                    .required(true)
            )
            .add_source(
                Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(",")
            )
            .build()
            .with_context(|| anyhow::anyhow!("Fail to load config"))? 
            .try_deserialize()
            .with_context(|| anyhow::anyhow!("Fail to deserialize config"))
    }
}

// 暴露公共方法
pub fn get() -> &'static AppConfig {
    &CONFIG
}
