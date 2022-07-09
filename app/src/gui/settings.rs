// Constructs all widgets for renderer settings.

use druid::widget::{Flex, Label, TextBox, Widget, Container, SizedBox};
use druid::{LensWrap, Color};
#[allow(unused_imports)]
use druid::widget::{CrossAxisAlignment, MainAxisAlignment, Align};
use crate::app::*;

pub fn build_ui_settings() -> impl Widget<AppState> {
    let label_settings = Label::new(|_data: &AppState, _env: &druid::Env| {
            format!("Settings")
        });
        // #todo-gui: padding and align are unavailable after
        // settings construction is separated to this function?
        //.padding(5.0)
        //.align_left();

    // #todo-gui: Use macro to create each row?
    let exposure_label = Label::new("exposure: ");
    let exposure_input = LensWrap::new(TextBox::new(), AppState::exposure_input);
    let exposure_row = Flex::row().with_child(exposure_label).with_child(exposure_input);

    let gamma_label = Label::new("gamma: ");
    let gamma_input = LensWrap::new(TextBox::new(), AppState::gamma_correction_input);
    let gamma_row = Flex::row().with_child(gamma_label).with_child(gamma_input);

    let stepsize1_label = Label::new("primary step size: ");
    let stepsize1_input = LensWrap::new(TextBox::new(), AppState::primary_step_size_input);
    let stepsize1_row = Flex::row().with_child(stepsize1_label).with_child(stepsize1_input);

    let stepsize2_label = Label::new("secondary step size: ");
    let stepsize2_input = LensWrap::new(TextBox::new(), AppState::secondary_step_size_input);
    let stepsize2_row = Flex::row().with_child(stepsize2_label).with_child(stepsize2_input);

    Flex::column()
        .with_child(label_settings)
        .with_spacer(20.0)
        .with_child(exposure_row)
        .with_spacer(20.0)
        .with_child(gamma_row)
        .with_spacer(20.0)
        .with_child(stepsize1_row)
        .with_spacer(20.0)
        .with_child(stepsize2_row)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        // #todo-gui: Align with the viewport at top?
        //.main_axis_alignment(MainAxisAlignment::Start)
        //.must_fill_main_axis(true)
}

pub fn build_ui_output_log() -> impl Widget<AppState> {
    let label = Label::new(|data: &AppState, _env: &druid::Env| {
            data.get_all_log()
        })
        .with_text_color(Color::rgb(0, 0, 0));

    let label_container = SizedBox::new(
            Container::new(label)
            .background(Color::rgb(255, 255, 255))
        )
        .expand();

    Flex::column()
        .with_flex_child(label_container, 1.0)
        .must_fill_main_axis(true)
}
