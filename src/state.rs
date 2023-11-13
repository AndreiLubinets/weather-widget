use std::ops::Add;

use druid::{Data, Lens};

use crate::api::domain::WeatherData;

#[derive(Clone, Data, Lens)]
pub struct State {
    pub temp: String,
    pub image: String,
    pub image_tooltip: String,
    pub location: String,
    pub date: String,
}

impl From<WeatherData> for State {
    fn from(value: WeatherData) -> Self {
        State {
            temp: value.weather.temp_c.to_string(),
            image: String::new()
                .add("http:")
                .add(value.weather.condition.icon.as_str()),
            image_tooltip: value.weather.condition.text,
            location: value.location.to_string(),
            date: value.weather.last_updated,
        }
    }
}
