use anyhow::{Error, Ok};
use reqwest::blocking::Client;

use super::domain::{Location, WeatherData};

pub trait Api {
    fn get(&self, location: impl Into<String>) -> Result<WeatherData, anyhow::Error>;

    fn get_locations(&self) -> Result<Vec<Location>, anyhow::Error>;
}

pub struct WeatherApi {
    client: Client,
    key: String,
}

impl WeatherApi {
    pub fn new(key: impl Into<String>) -> Self {
        Self { client: Client::new(), key: key.into() }
    }
}

impl Api for WeatherApi {
    fn get(&self, location: impl Into<String>) -> Result<WeatherData, anyhow::Error> {
        println!("Fetching");
        let uri = "http://api.weatherapi.com/v1/forecast.json";
        self.client
            .get(uri)
            .query(&[("key", &self.key), ("q", &location.into())])
            .send()?
            .json::<WeatherData>()
            .map_err(Error::from)
    }

    fn get_locations(&self) -> Result<Vec<Location>, anyhow::Error> {
        Ok(vec![
            Location::new("London", "United Kingdom"),
            Location::new("Paris", "France"),
        ])
    }
}
