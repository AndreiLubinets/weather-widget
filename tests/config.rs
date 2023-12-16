use std::{fs, path::PathBuf};

use druid::{Color, Env};
use weather_widget::{
    config::{Config, ConfigBuilder, Size},
    view::BACKGROUND_COLOR_KEY,
};

fn build_config() -> Config {
    ConfigBuilder::default()
        .uri("uri".to_owned())
        .location("location".to_owned())
        .bg_color(Some("#2a2a3e".to_owned()))
        .size(Some(Size::new(500., 400.)))
        .build()
        .unwrap()
}

#[test]
fn load_test() {
    let expected = build_config();
    let path = PathBuf::from("tests/resources/Config.toml");
    let serialized = toml::to_string(&expected).unwrap();
    let _ = fs::write(&path, serialized);

    let actual = Config::load(path).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn set_env_test() {
    let config = build_config();
    let expected = Color::from_hex_str("#2a2a3e").unwrap();
    let mut actual = Env::empty();

    config.set_env(&mut actual);

    assert_eq!(expected, actual.get(BACKGROUND_COLOR_KEY));
}

#[test]
fn set_env_unparsable_hex_color_test() {
    let config = ConfigBuilder::default()
        .uri("uri".to_owned())
        .location("location".to_owned())
        .bg_color(Some("string".to_owned()))
        .size(None)
        .build()
        .unwrap();
    let expected = Color::rgb8(42, 42, 62);
    let mut actual = Env::empty();

    config.set_env(&mut actual);

    assert_eq!(expected, actual.get(BACKGROUND_COLOR_KEY));
}

#[test]
fn set_env_default_color_test() {
    let config = ConfigBuilder::default()
        .uri("uri".to_owned())
        .location("location".to_owned())
        .bg_color(None)
        .size(None)
        .build()
        .unwrap();
    let expected = Color::rgb8(42, 42, 62);
    let mut actual = Env::empty();

    config.set_env(&mut actual);

    assert_eq!(expected, actual.get(BACKGROUND_COLOR_KEY));
}

#[test]
fn get_window_size_test() {
    let config = build_config();
    let expected = druid::Size {
        width: 500.,
        height: 400.,
    };

    let actual = config.get_window_size();

    assert_eq!(expected, actual);
}

#[test]
fn get_window_size_default_test() {
    let config = ConfigBuilder::default()
        .uri("uri".to_owned())
        .location("location".to_owned())
        .bg_color(None)
        .size(None)
        .build()
        .unwrap();
    let expected = druid::Size {
        width: 400.,
        height: 300.,
    };

    let actual = config.get_window_size();

    assert_eq!(expected, actual);
}
