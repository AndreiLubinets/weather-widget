use std::{fs, path::PathBuf};

use anyhow::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub uri: String,
    pub key: String,
    pub location: String,
    pub width: f64,
    pub height: f64,
}

impl Config {
    pub fn load(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let config_string = fs::read_to_string(path.into())?;
        toml::from_str::<Config>(&config_string).map_err(Error::from)
    }
}
