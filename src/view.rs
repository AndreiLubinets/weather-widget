use crate::api::api::WeatherApi;
use crate::api::image_buf::FromUrl;
use crate::state::{DayState, State};
use druid::text::{FontDescriptor, FontWeight};
use druid::widget::{Align, BackgroundBrush, Flex, Image, Label, List, Spinner};
use druid::{Color, Env, ImageBuf, Key, Widget, WidgetExt};
use druid_widget_nursery::FutureWidget;

const LOCATION_TEXT_SIZE: f64 = 18.;
const SPINNER_SIZE: f64 = 32.;
const ERROR_TEXT_SIZE: f64 = 14.;
const CONDITION_IMAGE_SIZE: f64 = 64.;
pub const BACKGROUND_COLOR_KEY: Key<Color> = Key::new("org.weather-widget.background");

pub fn build_view() -> impl Widget<State> {
    let error_label = Label::new(|data: &State, _env: &Env| data.error.clone().unwrap_or_default())
        .with_font(FontDescriptor::default().with_weight(FontWeight::BOLD))
        .with_text_color(Color::RED)
        .with_text_size(ERROR_TEXT_SIZE);

    let location_label = Label::new(|data: &State, _env: &Env| data.location.to_string())
        .with_font(FontDescriptor::default().with_weight(FontWeight::BOLD))
        .with_text_size(LOCATION_TEXT_SIZE);

    let day_list = build_day_list();

    let layout = Flex::column()
        .with_spacer(10.0)
        .with_child(error_label)
        .with_child(location_label)
        .with_child(day_list);

    Align::centered(layout).background(BackgroundBrush::from(BACKGROUND_COLOR_KEY))
}

fn build_day_list() -> impl Widget<State> {
    FutureWidget::new(
        |data: &State, _env| WeatherApi::global().get(data.location.clone()),
        Spinner::new().fix_size(SPINNER_SIZE, SPINNER_SIZE),
        |result, data, _env| {
            let error_flag = match *result {
                Ok(res) => {
                    *data = res.into();
                    false
                }
                Err(err) => {
                    println!("{}", err);
                    data.error = Some("Unable to get data from the api".to_string());
                    true
                }
            };
            List::new(build_day_widget)
                .horizontal()
                .with_spacing(10.0)
                .lens(State::day_states)
                .disabled_if(move |_state, _env| error_flag)
                .boxed()
        },
    )
    .center()
}

fn build_day_widget() -> impl Widget<DayState> {
    let date_label = Label::new(|data: &DayState, _env: &Env| data.date.clone());

    let condition_icon = get_condition_icon();

    let max_temp_label = Label::new(|data: &DayState, _env: &Env| data.max_temp.clone());
    let min_temp_label = Label::new(|data: &DayState, _env: &Env| data.min_temp.clone());

    let layout = Flex::column()
        .with_spacer(1.0)
        .with_child(date_label)
        .with_child(condition_icon)
        .with_child(max_temp_label)
        .with_child(min_temp_label);

    Align::centered(layout)
}

fn get_condition_icon() -> impl Widget<DayState> {
    let condition_icon = FutureWidget::new(
        |data: &DayState, _env| ImageBuf::from_url(data.image.clone()),
        Image::new(ImageBuf::empty()),
        |result, _data, _env| {
            match *result {
                Ok(res) => Image::new(res),
                Err(err) => {
                    println!("{}", err);
                    Image::new(ImageBuf::empty())
                }
            }
            .boxed()
        },
    )
    .fix_size(CONDITION_IMAGE_SIZE, CONDITION_IMAGE_SIZE);

    druid_widget_nursery::WidgetExt::tooltip(condition_icon, |data: &DayState, _env: &_| {
        data.image_tooltip.clone()
    })
}
