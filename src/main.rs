use api::api::{TestApi, WeatherApi, Api};
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

mod object;
mod api;


fn main() {
    let mut app =  QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/main.qml"));
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
