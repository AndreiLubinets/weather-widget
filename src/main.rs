use api::api::{WeatherApi, Api};
use druid::{WindowDesc, AppLauncher};
use view::build_view;

mod api;
mod state;
mod view;


fn main() {
    let main_window = WindowDesc::new(build_view())
        .window_size((300.0, 400.0));

    let initial_state = WeatherApi::default()
        .get("London")
        .expect("Failed to get data from api")
        .into();

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
