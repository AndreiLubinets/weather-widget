use crate::api::image_buf::FromUrl;
use crate::state::State;
use druid::text::{FontDescriptor, FontWeight};
use druid::widget::{Align, BackgroundBrush, Container, Flex, Image, Label, ViewSwitcher};
use druid::{Color, Env, ImageBuf, Widget, WidgetExt};

const LOCATION_TEXT_SIZE: f64 = 18.0;
const CONDITION_IMAGE_SIZE: f64 = 64.0;
const BACKGROUND_COLOR: &str = "#2a2a3e";

pub fn build_view() -> impl Widget<State> {
    let location_label = Label::new(|data: &State, _env: &Env| data.location.to_string())
        .with_font(FontDescriptor::default().with_weight(FontWeight::BOLD))
        .with_text_size(LOCATION_TEXT_SIZE);

    let date_label = Label::new(|data: &State, _env: &Env| data.date.clone());

    let condition_image = ViewSwitcher::new(
        |data: &State, _env| data.image.clone(),
        |url, _data, _env| match ImageBuf::from_url(url) {
            Ok(res) => Box::new(Image::new(res)),
            Err(_) => Box::new(Image::new(ImageBuf::empty())),
        },
    )
    .fix_size(CONDITION_IMAGE_SIZE, CONDITION_IMAGE_SIZE);

    let temp_label = Label::new(|data: &State, _env: &Env| data.temp.clone() + "°C");

    let layout = Flex::column()
        .with_child(location_label)
        .with_child(date_label)
        .with_child(condition_image)
        .with_spacer(1.0)
        .with_child(temp_label);

    let background_color =
        Color::from_hex_str(BACKGROUND_COLOR).expect("Unable to parse background color");

    Align::centered(layout).background(BackgroundBrush::Color(background_color))
}
