mod canvas;

use druid::widget::prelude::*;
use druid::{AppLauncher, Lens, LocalizedString, UnitPoint, Widget, WidgetExt, WindowDesc};
use druid::text::ParseFormatter;
use druid::widget::{Container, Flex, Label, LensWrap, Stepper, TextBox};
use crate::canvas::Canvas;

#[derive(Clone, Data, Lens)]
struct AppState {
    a: f64,
    b: f64,
    c: f64,
    m: f64,
    scale: (f64, f64, f64),
    rotation: (f64, f64, f64),
    translation: (f64, f64, f64),
    accuracy: usize,
    min_accuracy: usize,
    right_button_clicked: bool,
    right_button_position: (f64, f64),
    ctrl_clicked: bool,
    left_button_clicked: bool,
    left_button_position: (f64, f64),
    shift_clicked: bool,
}

impl AppState {
    fn new() -> Self {
        AppState { 
            a: 1.0, b: 1.0, c: 1.0, m: 1.0,
            scale: (1.0, 1.0, 1.0),
            rotation: (0.0, 0.0, 0.0),
            translation: (0.0, 0.0, 0.0),
            accuracy: 1,
            min_accuracy: 32,
            right_button_clicked: false,
            right_button_position: (0.0, 0.0),
            ctrl_clicked: false,
            left_button_clicked: false,
            left_button_position: (0.0, 0.0),
            shift_clicked: false,
        }
    }
}

fn build_ui() -> impl Widget<AppState> {
    Flex::row()
        .with_flex_child(Canvas::new().expand(), 5.0)
        .with_flex_child(
            Flex::column()
                .with_flex_child(
                    Container::new(
                        Flex::column()
                            .with_flex_child(
                                build_variable_menu("a:", AppState::a, AppState::a, (0.1, 10.0), 0.1),
                                1.0
                            )
                            .with_flex_child(
                                build_variable_menu("b:", AppState::b, AppState::b, (0.1, 10.0), 0.1),
                                1.0
                            )
                            .with_flex_child(
                                build_variable_menu("c:", AppState::c, AppState::c, (0.1, 10.0), 0.1),
                                1.0
                            )
                            .with_flex_child(
                                build_variable_menu("m:", AppState::m, AppState::m, (1.0, 100.0), 1.0),
                                1.0
                            )
                    ).expand(),
                    1.0
                )
                .with_flex_child(
                    Container::new(
                        Flex::column()
                            .with_flex_child(
                                Container::new(
                                    LensWrap::new(
                                        Flex::column()
                                            .with_flex_child(
                                                Label::dynamic(|data: &(f64, f64, f64), _| format!("ScaleX: {}", data.0)).expand_width(),
                                                1.0
                                            )
                                            .with_flex_child(
                                                Label::dynamic(|data: &(f64, f64, f64), _| format!("ScaleY: {}", data.1)).expand_width(),
                                                1.0
                                            )
                                            .with_flex_child(
                                                Label::dynamic(|data: &(f64, f64, f64), _| format!("ScaleZ: {}", data.2)).expand_width(),
                                                1.0
                                            ),
                                        AppState::scale,
                                    ),
                                ).expand(),
                                3.0
                            )
                            .with_flex_child(
                                Container::new(
                                    LensWrap::new(
                                        Flex::column()
                                            .with_flex_child(
                                                Label::dynamic(|data: &(f64, f64, f64), _| format!("RotationX: {}", data.0)).expand_width(),
                                                1.0
                                            )
                                            .with_flex_child(
                                                Label::dynamic(|data: &(f64, f64, f64), _| format!("RotationY: {}", data.1)).expand_width(),
                                                1.0
                                            )
                                            .with_flex_child(
                                                Label::dynamic(|data: &(f64, f64, f64), _| format!("RotationZ: {}", data.2)).expand_width(),
                                                1.0
                                            ),
                                        AppState::rotation,
                                    ),
                                ).expand(),
                                3.0
                            )
                            .with_flex_child(
                                Container::new(
                                    LensWrap::new(
                                        Flex::column()
                                            .with_flex_child(
                                                Label::dynamic(|data: &(f64, f64, f64), _| format!("TranslationX: {}", data.0)).expand_width(),
                                                1.0
                                            )
                                            .with_flex_child(
                                                Label::dynamic(|data: &(f64, f64, f64), _| format!("TranslationY: {}", data.1)).expand_width(),
                                                1.0
                                            )
                                            .with_flex_child(
                                                Label::dynamic(|data: &(f64, f64, f64), _| format!("TranslationZ: {}", data.2)).expand_width(),
                                                1.0
                                            ),
                                        AppState::translation,
                                    ),
                                ).expand(),
                                3.0
                            )
                            .with_flex_child(
                                Flex::column()
                                    .with_flex_child(
                                        LensWrap::new(
                                            Label::dynamic(|data: &usize, _| format!("Accuracy: {}", data)).expand_width(),
                                            AppState::accuracy,
                                        ).expand(),
                                        1.0,
                                    )
                                    .with_flex_child(
                                        LensWrap::new(
                                            Label::dynamic(|data: &usize, _| format!("Min accuracy: {}", data)).expand_width(),
                                            AppState::min_accuracy,
                                        ).expand(),
                                        1.0,
                                    )
                                    .expand(),
                                2.0
                            )
                            .expand()
                    ).expand_width(),
                    1.0
                )
                .expand(),
            1.0
        )
}

fn build_variable_menu(
    text: &str,
    lens_text_box: impl Lens<AppState, f64> + 'static,
    lens_stepper: impl Lens<AppState, f64> + 'static,
    range: (f64, f64),
    step: f64,
) -> impl Widget<AppState> {
    Flex::row()
        .with_child(
            Label::new(text)
        )
        .with_flex_child(
            TextBox::new()
                .with_formatter(ParseFormatter::new())
                .lens(lens_text_box)
                .expand_width(),
            1.0
        )
        .with_child(
            Stepper::new()
                .with_range(range.0, range.1)
                .with_step(step)
                .lens(lens_stepper)
        )
        .expand_width()
        .align_vertical(UnitPoint::TOP)
}

fn main() {
    let width = 800usize;
    let height = 600usize;

    let main_window = WindowDesc::new(build_ui())
        .title(LocalizedString::new("Raycasting"))
        .window_size((width as f64, height as f64));

    let initial_state = AppState::new();

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}
