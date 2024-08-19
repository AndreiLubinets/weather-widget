use std::fs;

use druid::ImageBuf;
use weather_widget::api::image_buf::FromUrl;

#[tokio::test]
async fn image_buf_from_url_test() {
    let mut server = mockito::Server::new_async().await;
    let url = server.url();
    let image_url = String::from(&url) + "/imagepath.png";
    let expected = ImageBuf::from_file("tests/resources/image.png").unwrap();
    let body = fs::read("tests/resources/image.png").unwrap();

    // Create a mock
    let mock = server
        .mock("GET", "/imagepath.png")
        .with_status(200)
        .with_header("content-type", "image/x-png")
        .with_body(body)
        .create();

    let actual = ImageBuf::from_url(image_url).await.unwrap();

    assert_eq!(expected.raw_pixels(), actual.raw_pixels());
    mock.assert();
}
