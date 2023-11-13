use api::api::{Api, WeatherApi};
use config::Config;
use druid::{AppLauncher, WindowDesc};
use view::build_view;

mod api;
mod config;
mod state;
mod view;

fn main() {
    let main_window = WindowDesc::new(build_view()).window_size((300.0, 400.0));

    let config = Config::load().expect("Cannot load the configuration file");

    let initial_state = WeatherApi::new(&config.key)
        .get(&config.location)
        .expect("Failed to get data from api")
        .into();

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
