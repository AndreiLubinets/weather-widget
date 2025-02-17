use std::{env, process::exit};

use api::api::WeatherApi;
use clap::Parser;
use cli::Args;
use config::Config;
use druid::{AppLauncher, Env, WindowDesc};
use state::State;
use view::build_view;

mod api;
mod cli;
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

    WeatherApi::new(key, &config.uri).set_as_global();

    let args = Args::parse();
    if args.nogui {
        let weather_data = WeatherApi::global().current(&config.location).await;
        match weather_data {
            Ok(data) => {
                print!("{}", data);
                exit(0);
            }
            Err(_) => exit(1),
        };
    };

    let main_window = WindowDesc::new(build_view())
        .title(APPLICATION_TITLE)
        .show_titlebar(false)
        .window_size(config.get_window_size());

    let initial_state = State::initial(&config.location);

    AppLauncher::with_window(main_window)
        .configure_env(move |env: &mut Env, _data| config.set_env(env))
        .launch(initial_state)
        .expect("Failed to launch application");
}
