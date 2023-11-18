use std::{fs, path::PathBuf};

use qweather::config::Config;

#[test]
fn load_test() {
    let expected = Config {
        uri: "uri".to_owned(),
        key: "key".to_owned(),
        location: "location".to_owned(),
        width: 200.0,
        height: 300.0,
    };
    let path = PathBuf::from("tests/resources/Config.toml");
    let serialized = toml::to_string(&expected).unwrap();
    let _ = fs::write(&path, serialized);

    let actual = Config::load(path).unwrap();

    assert_eq!(&expected.uri, &actual.uri);
    assert_eq!(&expected.key, &actual.key);
    assert_eq!(&expected.location, &actual.location);
}
