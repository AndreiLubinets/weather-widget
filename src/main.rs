use api::api::{Api, WeatherApi, GLOBAL_WEBAPI};
use config::Config;
use druid::{AppLauncher, WindowDesc};
use view::build_view;

mod api;
mod config;
mod state;
mod view;

const APPLICATION_TITLE: &str = "Weather Widget";

fn main() {
    let config = Config::load("Config.toml").expect("Cannot load the configuration file");

    let main_window = WindowDesc::new(build_view())
        .title(APPLICATION_TITLE)
        .show_titlebar(false)
        .window_size((config.width, config.height));

    WeatherApi::new(&config.key, &config.uri).set_as_global();

    let initial_state = GLOBAL_WEBAPI
        .get()
        .unwrap()
        .get(&config.location)
        .expect("Failed to get data from api")
        .into();

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
