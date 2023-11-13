use anyhow::{Error, Result};
use reqwest::blocking::Client;

use super::domain::WeatherData;

pub trait Api {
    fn get(&self, location: impl Into<String>) -> Result<WeatherData>;
}

pub struct WeatherApi {
    client: Client,
    key: String,
}

impl WeatherApi {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            key: key.into(),
        }
    }
}

impl Api for WeatherApi {
    fn get(&self, location: impl Into<String>) -> Result<WeatherData> {
        let uri = "http://api.weatherapi.com/v1/forecast.json";
        self.client
            .get(uri)
            .query(&[
                ("key", &self.key),
                ("q", &location.into()),
                ("days", &"4".to_owned()),
            ])
            .send()?
            .json::<WeatherData>()
            .map_err(Error::from)
    }
}
