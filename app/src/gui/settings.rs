// Constructs all widgets for renderer settings.

use druid::widget::{Flex, Label, TextBox, Widget, WidgetExt, Container, SizedBox, Checkbox};
use druid::{Color};
#[allow(unused_imports)]
use druid::widget::{CrossAxisAlignment, MainAxisAlignment, Align};
use crate::app::*;

pub fn build_ui_settings() -> impl Widget<AppState> {
    let label_settings = Label::new(|_data: &AppState, _env: &druid::Env| {
            format!("[Renderer Settings]")
        });

    // #todo-gui: Use macro to create each row?
    let work_group_size_label = Label::new("work group size: ");
    let work_group_size_input_x = TextBox::new().lens(AppState::work_group_size_x_input);
    let work_group_size_input_y = TextBox::new().lens(AppState::work_group_size_y_input);
    let work_group_size_row = Flex::row()
        .with_child(work_group_size_label)
        .with_child(work_group_size_input_x)
        .with_spacer(5.0)
        .with_child(work_group_size_input_y);

    let exposure_label = Label::new("exposure: ");
    let exposure_input = TextBox::new().lens(AppState::exposure_input);
    let exposure_row = Flex::row().with_child(exposure_label).with_child(exposure_input);

    let gamma_label = Label::new("gamma: ");
    let gamma_input = TextBox::new().lens(AppState::gamma_correction_input);
    let gamma_row = Flex::row().with_child(gamma_label).with_child(gamma_input);

    let stepsize1_label = Label::new("primary step size: ");
    let stepsize1_input = TextBox::new().lens(AppState::primary_step_size_input);
    let stepsize1_row = Flex::row().with_child(stepsize1_label).with_child(stepsize1_input);

    let stepsize2_label = Label::new("secondary step size: ");
    let stepsize2_input = TextBox::new().lens(AppState::secondary_step_size_input);
    let stepsize2_row = Flex::row().with_child(stepsize2_label).with_child(stepsize2_input);

    let sky_checkbox = Checkbox::new("draw sky atmosphere").lens(AppState::draw_sky_input);
    let sky_row = Flex::row().with_child(sky_checkbox);

    let camera_origin_row = Flex::row()
        .with_child(Label::new("camera origin: "))
        .with_child(TextBox::new().lens(AppState::camera_origin_x_input))
        .with_child(TextBox::new().lens(AppState::camera_origin_y_input))
        .with_child(TextBox::new().lens(AppState::camera_origin_z_input));

    let camera_lookat_row = Flex::row()
        .with_child(Label::new("camera lookat: "))
        .with_child(TextBox::new().lens(AppState::camera_lookat_x_input))
        .with_child(TextBox::new().lens(AppState::camera_lookat_y_input))
        .with_child(TextBox::new().lens(AppState::camera_lookat_z_input));

    let fov_row = Flex::row()
        .with_child(Label::new("field of view: "))
        .with_child(TextBox::new().lens(AppState::fov_input));

    let spacer_between_options = 0.2;
    let col = Flex::column()
        .with_flex_spacer(0.4)
        .with_flex_child(label_settings, 1.0)
        .with_flex_spacer(spacer_between_options)
        .with_flex_child(work_group_size_row, 1.0)
        .with_flex_spacer(spacer_between_options)
        .with_flex_child(exposure_row, 1.0)
        .with_flex_spacer(spacer_between_options)
        .with_flex_child(gamma_row, 1.0)
        .with_flex_spacer(spacer_between_options)
        .with_flex_child(stepsize1_row, 1.0)
        .with_flex_spacer(spacer_between_options)
        .with_flex_child(stepsize2_row, 1.0)
        .with_flex_spacer(spacer_between_options)
        .with_flex_child(sky_row, 1.0)
        .with_flex_spacer(spacer_between_options)
        .with_flex_child(camera_origin_row, 1.0)
        .with_flex_spacer(spacer_between_options)
        .with_flex_child(camera_lookat_row, 1.0)
        .with_flex_spacer(spacer_between_options)
        .with_flex_child(fov_row, 1.0)
        .cross_axis_alignment(CrossAxisAlignment::Start);

    let container = Container::new(col)
        .background(Color::rgb(0.5, 0.5, 0.5));

    let sized_container = SizedBox::new(container)
        .expand();

    sized_container
}

// #todo-gui: Add a scroll bar to the output log.
// Still don't know how exactly druid's layout works X(
// See: OUTPUT_LOG_MAX_LINES
pub fn build_ui_output_log() -> impl Widget<AppState> {
    let label = Label::new(|data: &AppState, _env: &druid::Env| {
            data.get_all_log()
        })
        .with_text_color(Color::rgb(0.0, 0.0, 0.0));

    let label_container = SizedBox::new(
            Container::new(label)
                .background(Color::rgb(1.0, 1.0, 1.0))
        )
        .expand();

    Flex::column()
        .with_flex_child(label_container, 1.0)
        .must_fill_main_axis(true)
}
