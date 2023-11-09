use druid::{Widget, WidgetExt};
use druid::widget::{Align, Button, Flex, Label, TextBox};
use crate::state::State;

pub fn build_login_widget() -> impl Widget<State> {
    // a label that will determine its text based on the current app data.
    let label = Label::new("Sign in to Yggdrasil");

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(username_textbox)
        .with_child(password_textbox)
        .with_child(button);


    // center the two widgets in the available space
    Align::centered(layout)
}