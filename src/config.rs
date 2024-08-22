use std::{env, fs, path::PathBuf};

use anyhow::Error;
use derive_builder::Builder;
use druid::{Color, Env};
use serde::{Deserialize, Serialize};

use crate::view::BACKGROUND_COLOR_KEY;

const DEFAULT_BG_COLOR: Color = Color::rgb8(42, 42, 62);

#[derive(Serialize, Deserialize, Builder, Debug, PartialEq, Eq)]
pub struct Config {
    pub uri: String,
    pub location: String,
    bg_color: Option<String>,
    size: Option<Size>,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let path = Self::os_config_path()?;
        let config = Self::default();

        log::info!("Creating new config file: {}", path.display());

        fs::create_dir_all(path.parent().ok_or(anyhow::Error::msg("Unable to create config dir"))?)?;
        fs::write(path, toml::to_string::<Config>(&config)?)?;

        Ok(config)
    }

    pub fn load(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let config_string = fs::read_to_string(path.into())?;
        toml::from_str::<Config>(&config_string).map_err(Error::from)
    }

    pub fn load_from_os_config() -> anyhow::Result<Self> {
        Config::load(Self::os_config_path()?)
    }

    pub fn get_window_size(&self) -> druid::Size {
        self.size.unwrap_or_default().into()
    }

    pub fn set_env(&self, env: &mut Env) {
        let color = &self
            .bg_color
            .as_ref()
            .and_then(|hex| Color::from_hex_str(hex).ok())
            .unwrap_or(DEFAULT_BG_COLOR);

        env.set(BACKGROUND_COLOR_KEY, *color);
    }

    fn os_config_path() -> anyhow::Result<PathBuf> {
        let package_name = env!("PKG_NAME");
        let home_path =
            env::var_os("HOME").ok_or(anyhow::Error::msg("Unable to get home directory path"))?;
        let path = PathBuf::from(home_path)
            .join(".config")
            .join(package_name)
            .join("Config.toml");

        Ok(path)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            uri: "http://api.weatherapi.com/v1/".to_string(),
            //TODO: implement location detection
            location: "Stockholm, Sweden".to_string(),
            bg_color: None,
            size: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Size {
    width: f64,
    height: f64,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: 400.,
            height: 300.,
        }
    }
}

impl Size {
    pub fn new(width: impl Into<f64>, height: impl Into<f64>) -> Self {
        Size {
            width: width.into(),
            height: height.into(),
        }
    }
}

impl From<Size> for druid::Size {
    fn from(val: Size) -> Self {
        druid::Size {
            width: val.width,
            height: val.height,
        }
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl Eq for Size {}
