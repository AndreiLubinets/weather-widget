use std::{sync::OnceLock, time::Duration};

use anyhow::{Error, Result};
use reqwest::{Client, Url};

use super::domain::WeatherData;

pub(crate) static GLOBAL_WEBAPI: OnceLock<WeatherApi> = OnceLock::new();

#[derive(Debug)]
pub struct WeatherApi {
    client: Client,
    key: String,
    url: String,
}

impl WeatherApi {
    pub fn new(key: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            key: key.into(),
            url: url.into(),
        }
    }

    pub async fn get(&self, location: impl Into<String>) -> Result<WeatherData> {
        let uri = Url::parse(&self.url)?.join("forecast.json")?;
        println!("Fetching data from: {}", &uri);
        let forecast_days = 4;

        self.client
            .get(uri)
            .query(&[
                ("key", &self.key),
                ("q", &location.into()),
                ("days", &forecast_days.to_string()),
            ])
            .timeout(Duration::from_secs(10))
            .send()
            .await?
            .json::<WeatherData>()
            .await
            .map_err(Error::from)
    }

    pub fn set_as_global(self) {
        GLOBAL_WEBAPI.set(self).unwrap();
    }

    pub fn global() -> &'static Self {
        GLOBAL_WEBAPI
            .get()
            .expect("Unable to get api client instance")
    }
}
