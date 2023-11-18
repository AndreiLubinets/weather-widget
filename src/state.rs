use druid::{im::Vector, Data, Lens};

use crate::api::domain::{Forecastday, WeatherData};

#[derive(Clone, Data, Lens, Debug, PartialEq, Eq)]
pub struct State {
    pub location: String,

    #[data(eq)]
    pub day_states: Vector<DayState>,
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
        }
    }
}

#[derive(Clone, Data, Lens, PartialEq, Eq, Default, Debug)]
pub struct DayState {
    pub temp: String,
    pub image: String,
    pub image_tooltip: String,
    pub date: String,
}

impl From<Forecastday> for DayState {
    fn from(value: Forecastday) -> Self {
        DayState {
            temp: value.day.maxtemp_c.to_string(),
            image: String::from("http:") + &value.day.condition.icon,
            image_tooltip: value.day.condition.text,
            date: value.date,
        }
    }
}
