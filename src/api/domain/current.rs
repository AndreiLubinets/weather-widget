use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CurrentWeatherData {
    pub current: Current,
}

impl Display for CurrentWeatherData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°C", self.current.temp_c)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Current {
    pub temp_c: f64,
}
