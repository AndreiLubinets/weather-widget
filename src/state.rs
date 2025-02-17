use derive_builder::Builder;
use druid::{im::Vector, Data, Lens};

use crate::api::domain::forecast::{Forecastday, WeatherData};

#[derive(Clone, Data, Lens, Debug, PartialEq, Eq, Default)]
pub struct State {
    pub location: String,

    #[data(eq)]
    pub day_states: Vector<DayState>,

    pub error: Option<String>,
}

impl State {
    pub fn initial(location: impl Into<String>) -> Self {
        State {
            location: location.into(),
            ..Default::default()
        }
    }
}

impl From<WeatherData> for State {
    fn from(value: WeatherData) -> Self {
        State {
            location: value.location.to_string(),
            day_states: value
                .forecast
                .forecastday
                .iter()
                .map(|day| DayState::from(day.clone()))
                .collect(),
            error: None,
        }
    }
}

#[derive(Clone, Data, Lens, PartialEq, Eq, Default, Debug, Builder)]
pub struct DayState {
    pub image: String,
    pub image_tooltip: String,
    max_temp: String,
    min_temp: String,
    date: String,
}

impl From<Forecastday> for DayState {
    fn from(value: Forecastday) -> Self {
        DayState {
            max_temp: value.day.maxtemp_c.to_string() + "°C",
            min_temp: value.day.mintemp_c.to_string() + "°C",
            image: String::from("http:") + &value.day.condition.icon,
            image_tooltip: value.day.condition.text,
            date: value.date,
        }
    }
}
