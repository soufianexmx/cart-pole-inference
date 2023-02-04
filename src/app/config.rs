use config::{Config, ConfigError, File};
use serde::Deserialize;

const CONFIG_PATH: &str = "config/default.toml";

#[derive(Deserialize)]
pub struct AppConfig {
    model_path: String,
    base_url: String,
    port: u16,
    log_level: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(CONFIG_PATH))
            .build()?;

        s.try_deserialize()
    }

    pub fn model_path(&self) -> &str {
        &self.model_path
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn log_level(&self) -> &str {
        &self.log_level
    }
}
