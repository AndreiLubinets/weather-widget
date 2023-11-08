use std::fs::File;

use anyhow::{Error, Ok};
use async_trait::async_trait;
use mockall::automock;
use reqwest::{Client, blocking};

use super::domain::{WeatherData, Location};


#[async_trait]
#[automock]
pub trait Api: Send + Sync {
    async fn get(&self, location: String) -> Result<WeatherData, anyhow::Error>;

    async fn get_locations(&self) -> Result<Vec<Location>, anyhow::Error>;
}

pub trait ApiSync {
    fn get(&self, location: String) -> Result<WeatherData, anyhow::Error>;

    fn get_locations(&self) -> Result<Vec<Location>, anyhow::Error>;
}

pub struct WeatherApi {
    client: Client,
    key: String
}

impl Default for WeatherApi {
    fn default() -> Self {
        WeatherApi { client: Client::new(), key: "acd6be4c7fec4cfbb48122154230411".to_owned() }
    }
}

#[async_trait]
impl Api for WeatherApi {

    async fn get(&self, location: String) -> Result<WeatherData, anyhow::Error> {
        println!("Fetching");
        let uri = "http://api.weatherapi.com/v1/current.json";
        self.client.get(uri)
            .query(&[("key", &self.key), ("q", &location)])
            .send()
            .await?
            .json::<WeatherData>()
            .await
            .map_err(Error::from)
    }

    async fn get_locations(&self) -> Result<Vec<Location>, anyhow::Error> {
        todo!()
    }
}

pub struct TestApi {
    client: Client,
}

#[async_trait]
impl Api for TestApi {

    async fn get(&self, _location: String) -> Result<WeatherData, anyhow::Error> {
        let file = File::open("resources/response.json")?;
        let res: Result<WeatherData, Error> = serde_json::from_reader(file)
            .map_err(Error::from);

        println!("{}", res.as_ref().unwrap());

        res  
    }

    async fn get_locations(&self) -> Result<Vec<Location>, anyhow::Error> {
        Ok(vec![
            Location::new("London", "United Kingdom"), 
            Location::new("Paris", "France")
        ])
    }
}

pub struct SyncTestApi {
    client: blocking::Client,
    key: String
}

impl Default for SyncTestApi {
    fn default() -> Self {
        SyncTestApi { client: blocking::Client::new(), key: "acd6be4c7fec4cfbb48122154230411".to_owned() }
    }
}

impl ApiSync for SyncTestApi {
    fn get(&self, location: String) -> Result<WeatherData, anyhow::Error> {
        println!("Fetching");
        let uri = "http://api.weatherapi.com/v1/current.json";
        self.client.get(uri)
            .query(&[("key", &self.key), ("q", &location)])
            .send()?
            .json::<WeatherData>()
            .map_err(Error::from)
    }

    fn get_locations(&self) -> Result<Vec<Location>, anyhow::Error> {
        Ok(vec![
            Location::new("London", "United Kingdom"), 
            Location::new("Paris", "France")
        ])
    }
}