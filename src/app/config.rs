use config::{Config, ConfigError, File};
use serde::Deserialize;

const CONFIG_BASE_PATH: &str = "config";
const CONFIG_DEFAULT: &str = "default";
const CONFIG_PREFIX: &str = "toml";

#[derive(Deserialize)]
pub struct AppConfig {
    model_path: String,
    address: String,
    port: u16,
    log_level: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = std::env::var("RUN_ENV").unwrap_or_else(|_| "local".into());

        let s = Config::builder()
            .add_source(File::with_name(&format!(
                "{}/{}.{}",
                CONFIG_BASE_PATH, CONFIG_DEFAULT, CONFIG_PREFIX
            )))
            .add_source(
                File::with_name(&format!(
                    "{}/{}.{}",
                    CONFIG_BASE_PATH, run_mode, CONFIG_PREFIX
                ))
                .required(false),
            )
            .build()?;

        s.try_deserialize()
    }

    pub fn model_path(&self) -> &str {
        &self.model_path
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn log_level(&self) -> &str {
        &self.log_level
    }
}
