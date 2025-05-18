use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct ScifferConfig {
    pub time_interval: u64,
}

pub fn load_config() -> ScifferConfig {
    let config = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .expect("Config may be missing");

    let config: ScifferConfig = config
        .try_deserialize()
        .expect("Config cannot be deserialized.");
    config
}
