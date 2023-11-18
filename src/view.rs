use crate::api::image_buf::FromUrl;
use crate::state::{DayState, State};
use druid::text::{FontDescriptor, FontWeight};
use druid::widget::{Align, BackgroundBrush, Flex, Image, Label, List, ViewSwitcher};
use druid::{Color, Env, ImageBuf, Widget, WidgetExt};

const LOCATION_TEXT_SIZE: f64 = 18.0;
const CONDITION_IMAGE_SIZE: f64 = 64.0;
const BACKGROUND_COLOR: &str = "#2a2a3e";

pub fn build_view() -> impl Widget<State> {
    let location_label = Label::new(|data: &State, _env: &Env| data.location.to_string())
        .with_font(FontDescriptor::default().with_weight(FontWeight::BOLD))
        .with_text_size(LOCATION_TEXT_SIZE);

    let day_list = List::new(build_day_widget)
        .horizontal()
        .with_spacing(10.0)
        .lens(State::day_states);

    let layout = Flex::column()
        .with_spacer(3.0)
        .with_child(location_label)
        .with_child(day_list);

    let background_color =
        Color::from_hex_str(BACKGROUND_COLOR).expect("Unable to parse background color");

    Align::centered(layout).background(BackgroundBrush::Color(background_color))
}

pub fn build_day_widget() -> impl Widget<DayState> {
    let date_label = Label::new(|data: &DayState, _env: &Env| data.date.clone());

    let condition_icon = get_condition_icon();

    let temp_label = Label::new(|data: &DayState, _env: &Env| data.temp.clone() + "°C");

    let layout = Flex::column()
        .with_spacer(1.0)
        .with_child(date_label)
        .with_child(condition_icon)
        .with_child(temp_label);

    Align::centered(layout)
}

fn get_condition_icon() -> impl Widget<DayState> {
    let condition_icon = ViewSwitcher::new(
        |data: &DayState, _env| data.image.clone(),
        |url, _data, _env| match ImageBuf::from_url(url) {
            Ok(res) => Box::new(Image::new(res)),
            Err(_) => Box::new(Image::new(ImageBuf::empty())),
        },
    )
    .fix_size(CONDITION_IMAGE_SIZE, CONDITION_IMAGE_SIZE);

    druid_widget_nursery::WidgetExt::tooltip(condition_icon, |data: &DayState, _env: &_| {
        data.image_tooltip.clone()
    })
}
