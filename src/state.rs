use druid::{Data, Lens};

use crate::api::domain::WeatherData;

#[derive(Clone, Data, Lens)]
pub struct State {
    pub temp: String,
    pub image: String,
    pub location: String
}

impl From<WeatherData> for State {
    fn from(value: WeatherData) -> Self {
        State { 
            temp: value.weather.temp_c.to_string(), 
            image: value.weather.condition.icon.clone(), 
            location: value.location.to_string() 
        }
    }
}