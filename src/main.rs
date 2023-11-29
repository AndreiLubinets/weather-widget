use api::api::WeatherApi;
use config::Config;
use druid::{AppLauncher, WindowDesc};
use state::State;
use view::build_view;

mod api;
mod config;
mod state;
mod view;

const APPLICATION_TITLE: &str = "Weather Widget";

#[tokio::main]
async fn main() {
    let config = Config::load("Config.toml").expect("Cannot load the configuration file");

    let main_window = WindowDesc::new(build_view())
        .title(APPLICATION_TITLE)
        .show_titlebar(false)
        .window_size((config.width, config.height));

    WeatherApi::new(&config.key, &config.uri).set_as_global();

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(State::initial(&config.location))
        .expect("Failed to launch application");
}
