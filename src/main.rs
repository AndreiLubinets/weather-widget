use druid::WindowDesc;

mod api;
mod state;
mod view;


fn main() {
    let main_window = WindowDesc::new(build_login_widget())
        .title(WINDOW_TITLE)
        .window_size((300.0, 400.0));

    // create the initial app state
    /*let initial_state = State {
        name: "".into(),
        password: "".into()
    };*/

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
