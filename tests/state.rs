use druid::im::vector;
use weather_widget::{
    api::domain::forecast::{Condition, Day, Forecast, Forecastday, Location, WeatherData},
    state::{DayStateBuilder, State},
};

fn build_weather_data() -> WeatherData {
    let location = Location::new("London", "United Kingdom");
    let condition = Condition {
        text: "sunny".to_owned(),
        icon: "//cdn.api.com".to_owned(),
    };
    let forecast = Forecast {
        forecastday: vec![Forecastday {
            date: "2023-01-01".to_owned(),
            day: Day {
                maxtemp_c: 20.,
                mintemp_c: 15.,
                condition,
            },
        }],
    };

    WeatherData { location, forecast }
}

#[test]
fn state_from_weather_data_test() {
    let data = build_weather_data();
    let date_states = vector![DayStateBuilder::default()
        .max_temp("20°C".to_owned())
        .min_temp("15°C".to_owned())
        .image("http://cdn.api.com".to_owned())
        .image_tooltip("sunny".to_owned())
        .date("2023-01-01".to_owned())
        .build()
        .unwrap()];
    let expected = State {
        location: "London, United Kingdom".to_owned(),
        day_states: date_states,
        error: None,
    };

    let actual = State::from(data);

    assert_eq!(expected, actual);
}
