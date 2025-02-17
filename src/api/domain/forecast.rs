use std::ops::Add;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct WeatherData {
    pub location: Location,

    pub forecast: Forecast,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Location {
    name: String,
    country: String,
}

impl Location {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, country: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            country: country.into(),
        }
    }
}

impl ToString for Location {
    fn to_string(&self) -> String {
        String::new().add(&self.name).add(", ").add(&self.country)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    pub text: String,
    pub icon: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Forecast {
    pub forecastday: Vec<Forecastday>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Forecastday {
    pub date: String,
    pub day: Day,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Day {
    pub maxtemp_c: f64,
    pub mintemp_c: f64,
    pub condition: Condition,
}

impl Eq for Day {}
