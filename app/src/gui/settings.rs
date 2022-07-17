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

    // #todo-gui: Use macro to create each row?
    let work_group_size_label = Label::new("work group size: ");
    let work_group_size_input_x = LensWrap::new(TextBox::new(), AppState::work_group_size_x_input);
    let work_group_size_input_y = LensWrap::new(TextBox::new(), AppState::work_group_size_y_input);
    let work_group_size_row = Flex::row()
        .with_child(work_group_size_label)
        .with_child(work_group_size_input_x)
        .with_spacer(5.0)
        .with_child(work_group_size_input_y);

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

    let col = Flex::column()
        .with_spacer(10.0)
        .with_child(label_settings)
        .with_spacer(20.0)
        .with_child(work_group_size_row)
        .with_spacer(20.0)
        .with_child(exposure_row)
        .with_spacer(20.0)
        .with_child(gamma_row)
        .with_spacer(20.0)
        .with_child(stepsize1_row)
        .with_spacer(20.0)
        .with_child(stepsize2_row)
        .cross_axis_alignment(CrossAxisAlignment::Start);
        
    SizedBox::new(
        Container::new(col)
            .background(Color::rgb(0.5, 0.5, 0.5))
        )
        .expand()
}

// #todo-gui: Add a scroll bar to the output log.
// Still don't know how exactly druid's layout works X(
// See: OUTPUT_LOG_MAX_LINES
pub fn build_ui_output_log() -> impl Widget<AppState> {
    let label = Label::new(|data: &AppState, _env: &druid::Env| {
            data.get_all_log()
        })
        .with_text_color(Color::rgb(0, 0, 0));

    let label_container = SizedBox::new(
            Container::new(label)
                .background(Color::rgb(1.0, 1.0, 1.0))
        )
        .expand();

    Flex::column()
        .with_flex_child(label_container, 1.0)
        .must_fill_main_axis(true)
}
