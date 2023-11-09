use druid::text::{FontDescriptor, FontWeight};
use druid::{Widget, WidgetExt, Env, ImageBuf, Color};
use druid::widget::{Align, Flex, Label, Image, ViewSwitcher};
use crate::state::State;
use crate::api::image_buf::FromUrl;

const LOCATION_TEXT_SIZE: f64 = 18.0;
const CONDITION_IMAGE_SIZE: f64 = 64.0;

//TODO: Implement background color
const BACKGROUND_COLOR: &str = "#2a2a3e";

pub fn build_view() -> impl Widget<State> {
    // a label that will determine its text based on the current app data.
    let location_label = Label::new(|data: &State, _env: &Env| data.location.to_string())
        .with_font(FontDescriptor::default().with_weight(FontWeight::BOLD))
        .with_text_size(LOCATION_TEXT_SIZE);


    //TODO: Implement handling for missing image
    let condition_image = ViewSwitcher::new(
        |data: &State, _env: &Env| true,
        |f, data: &State, _env: &Env| { 
            if *f {
                Box::new(Image::new(ImageBuf::from_url(data.image.clone()).unwrap()))
            } else {
                Box::new(Image::new(ImageBuf::empty()))
            }
        }
    )
        .fix_size(CONDITION_IMAGE_SIZE, CONDITION_IMAGE_SIZE);

    let temp_label = Label::new(|data: &State, _env: &Env| data.temp.clone() + "°C");

    let layout = Flex::column()
        .with_child(location_label)
        .with_child(condition_image)
        //.with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(temp_label);

    Align::centered(layout)
}

