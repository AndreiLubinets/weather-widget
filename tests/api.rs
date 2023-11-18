use std::{fs, io};

use mockito::Matcher;
use qweather::api::{
    api::{Api, WeatherApi},
    domain::{Condition, Day, Forecast, Forecastday, Location, WeatherData},
};

fn load_response() -> io::Result<String> {
    fs::read_to_string("tests/resources/response.json")
}

fn build_weather_data() -> WeatherData {
    let location = Location::new("Paris", "France");
    let condition = Condition {
        text: "Patchy rain possible".to_owned(),
        icon: "//cdn.weatherapi.com/weather/64x64/day/176.png".to_owned(),
    };
    let forecast = Forecast {
        forecastday: vec![Forecastday {
            date: "2023-11-17".to_owned(),
            day: Day {
                maxtemp_c: 11.6,
                mintemp_c: 8.3,
                condition,
            },
        }],
    };

    WeatherData { location, forecast }
}

#[test]
fn get_test() {
    let mut server = mockito::Server::new();
    let url = server.url();
    let weather_api = WeatherApi::new("key", url);
    let expected = build_weather_data();

    // Create a mock
    let mock = server
        .mock("GET", "/forecast.json")
        .match_query(Matcher::UrlEncoded("key".into(), "key".into()))
        .match_query(Matcher::UrlEncoded("q".into(), "location".into()))
        .match_query(Matcher::UrlEncoded("days".into(), "4".into()))
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body(load_response().unwrap())
        .create();

    let actual = weather_api.get("location").unwrap();

    assert_eq!(expected, actual);
    mock.assert();
}

#[test]
fn get_invalid_url() {
    let url = "unparsable string";
    let weather_api = WeatherApi::new("key", url);

    let actual = weather_api.get("location");

    assert!(actual.is_err());
}

#[test]
fn get_unparsable_response() {
    let mut server = mockito::Server::new();
    let url = server.url();
    let weather_api = WeatherApi::new("key", url);

    // Create a mock
    let mock = server
        .mock("GET", "/forecast.json")
        .match_query(Matcher::UrlEncoded("key".into(), "key".into()))
        .match_query(Matcher::UrlEncoded("q".into(), "location".into()))
        .match_query(Matcher::UrlEncoded("days".into(), "4".into()))
        .with_status(200)
        .with_header("content-type", "text/json")
        .with_body("")
        .create();

    let actual = weather_api.get("location");

    assert!(actual.is_err());
    mock.assert();
}
