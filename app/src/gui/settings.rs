// Constructs all widgets for renderer settings.

use druid::widget::{Flex, Label, TextBox, Widget};
use druid::LensWrap;
#[allow(unused_imports)]
use druid::widget::{CrossAxisAlignment, MainAxisAlignment, Align};
use crate::app::*;

pub fn build_ui_settings() -> impl Widget<AppState> {
    let label_settings = Label::new(|_data: &AppState, _env: &druid::Env| {
        format!("Settings (wip; doesn't work)")
        });
        // #todo-gui: padding and align are unavailable after
        // settings construction is separated to this function?
        //.padding(5.0)
        //.align_left();
    
    let gamma_label = Label::new(|_data: &AppState, _env: &druid::Env| {
            format!("gamma: ")
        });
        //.padding(5.0);

    let gamma_input = LensWrap::new(
        TextBox::new()
            .with_placeholder("gamma: ")
        , AppState::dummy_gamma_string);

    let gamma_row = Flex::row()
        .with_child(gamma_label)
        .with_child(gamma_input);
        //.align_left();

    Flex::column()
        .with_child(label_settings)
        .with_spacer(20.0)
        .with_child(gamma_row)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        // #todo-gui: Align with the viewport at top?
        //.main_axis_alignment(MainAxisAlignment::Start)
        //.must_fill_main_axis(true)
}
