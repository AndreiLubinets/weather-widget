use api::api::WeatherApi;
use config::Config;
use druid::{AppLauncher, Env, WindowDesc};
use state::State;
use view::build_view;

extern crate derive_builder;

mod api;
mod config;
mod state;
mod view;

const APPLICATION_TITLE: &str = "Weather Widget";

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let config = Config::load("Config.toml")
        .or_else(|_| Config::load_from_os_config())
        .or_else(|_| Config::new())
        .expect("Unable to create configuration file");
    let key = env!("API_KEY");

    let main_window = WindowDesc::new(build_view())
        .title(APPLICATION_TITLE)
        .show_titlebar(false)
        .window_size(config.get_window_size());

    let initial_state = State::initial(&config.location);

    WeatherApi::new(key, &config.uri).set_as_global();

    AppLauncher::with_window(main_window)
        .configure_env(move |env: &mut Env, _data| config.set_env(env))
        .launch(initial_state)
        .expect("Failed to launch application");
}
