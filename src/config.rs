use std::fs;

use anyhow::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub key: String,
    pub location: String,
    pub width: f64,
    pub height: f64,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_string = fs::read_to_string("Config.toml")?;
        toml::from_str::<Config>(&config_string).map_err(Error::from)
    }
}
