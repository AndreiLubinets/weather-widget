use std::{fmt::Display, ops::Add};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    pub location: Location,

    #[serde(rename = "current")]
    pub weather: Weather,

    pub forecast: Forecast,
}

impl Display for WeatherData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Location: {}, Weather: Temp: {}",
            self.location.name, self.weather.temp_c
        ))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    name: String,
    country: String,
}

impl ToString for Location {
    fn to_string(&self) -> String {
        String::new().add(&self.name).add(", ").add(&self.country)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Condition {
    pub text: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub last_updated: String,
    pub temp_c: f32,
    pub condition: Condition,
}

#[derive(Debug, Deserialize)]
pub struct Forecast {
    pub forecastday: Vec<Forecastday>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Forecastday {
    pub date: String,
    #[serde(rename = "date_epoch")]
    pub date_epoch: i64,
    pub day: Day,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    #[serde(rename = "maxtemp_c")]
    pub maxtemp_c: f64,

    #[serde(rename = "mintemp_c")]
    pub mintemp_c: f64,

    pub condition: Condition,
}
