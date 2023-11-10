use std::{fmt::Display, ops::Add};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData {
    pub location: Location,

    #[serde(rename = "current")] 
    pub weather: Weather
}

impl Display for WeatherData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Location: {}, Weather: Temp: {}", self.location.name, self.weather.temp_c))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    name: String,
    country: String
}

impl Location {
    pub fn new(name: impl Into<String>, country: impl Into<String>) -> Self {
        Location { name: name.into(), country: country.into() }
    }
}

impl ToString for Location {
    fn to_string(&self) -> String {
        String::new()
            .add(&self.name)
            .add(", ")
            .add(&self.country)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    text: String,
    pub icon: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub last_updated: String,
    pub temp_c: f32,
    pub condition: Condition
}
