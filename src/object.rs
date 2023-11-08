#[cxx_qt::bridge]
mod my_object {
    use crate::api::api::{TestApi, WeatherApi, Api, SyncTestApi, ApiSync};

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject(qml_uri = "demo", qml_version = "1.0")]
    pub struct AppState {
        #[qproperty]
        temp: QString,

        #[qproperty]
        image: QString,

        #[qproperty]
        location: QString
    }

    impl Default for AppState {
        fn default() -> Self {
            let data = SyncTestApi::default()
                .get("Mogilev Belarus".to_owned())
                .unwrap();

            AppState { 
                temp: QString::from(&data.weather.temp_c.to_string()), 
                image: QString::from(&data.weather.condition.icon),
                location: QString::from(&data.location.to_string()) 
            }
        }
    }

    impl qobject::AppState {
        #[qinvokable]
        pub fn get_weather(&self) {
            todo!()
        }
    }
}
