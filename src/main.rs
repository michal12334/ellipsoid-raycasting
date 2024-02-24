mod pixel;

use std::fmt::Formatter;
use druid::widget::prelude::*;
use druid::{AppLauncher, Color, Lens, lens, LocalizedString, PlatformError, Rect, UnitPoint, Widget, WidgetExt, WindowDesc};
use druid::piet::{ImageBuf, ImageFormat};
use druid::platform_menus::mac::file::default;
use druid::text::ParseFormatter;
use druid::widget::{BackgroundBrush, Button, Container, Flex, Label, SizedBox, Stepper, TextBox, ValueTextBox};
use im::Vector;
use nalgebra::{DMatrix, Matrix, Matrix4, OMatrix, SquareMatrix, Vector3, Vector4};
use crate::pixel::Pixel;

#[derive(Clone, Data, Lens)]
struct AppState {
    a: f64,
    b: f64,
    c: f64,
    m: f64,
}

impl AppState {
    fn new() -> Self {
        AppState { a: 1.0, b: 1.0, c: 1.0, m: 1.0, }
    }
}

struct CanvasWidget;

impl Widget<AppState> for CanvasWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppState, _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(&mut self, _layout_ctx: &mut LayoutCtx, _bc: &BoxConstraints, _data: &AppState, _env: &Env) -> Size {
        _bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        let rect = ctx.size().to_rect();
        let width = rect.width() as usize;
        let height = rect.height() as usize;
        let mut canvas = Vector::from((0..height).map(|_| Vector::from(vec![Pixel::new(); width])).collect::<Vec<Vector<Pixel>>>());

        for i in 0..width {
            for j in 0..height {
                let x = (i as i32 - (width as i32 / 2)) as f32 / ((width / 2) as f32);
                let y = (j as i32 - (height as i32 / 2)) as f32 / ((height / 2) as f32) * (-1.0);

                let a = data.a as f32;
                let b = data.b as f32;
                let c = data.c as f32;
                let m = data.m as i32;

                let l = a * x * x + b * y * y;
                if l < 1.0 {
                    let z = (1.0 - l).sqrt() / c;

                    let n = Vector3::new(2.0 * a * x, 2.0 * b * y, 2.0 * c * z).normalize();
                    let v = Vector3::new(-x, -y, 100.0 - z).normalize();

                    let intensity = n.dot(&v).powi(m) as f64;

                    let yellow = Color::YELLOW.as_rgba();
                    let color = Color::rgb(yellow.0 * intensity, yellow.1 * intensity, yellow.2 * intensity);

                    canvas[j][i] = Pixel::from(color.as_rgba8());
                }
            }
        }

        let image = ImageBuf
            ::from_raw(
                canvas.iter()
                    .flat_map(|f| f)
                    .map(|f| [f.r, f.g, f.b, f.a])
                    .flatten()
                    .collect::<Vec<u8>>(),
                ImageFormat::RgbaSeparate,
                width,
                height)
            .to_image(ctx.render_ctx);
        ctx.draw_image(&image, rect, druid::piet::InterpolationMode::Bilinear);
    }
}

fn build_ui() -> impl Widget<AppState> {
    Flex::row()
        .with_flex_child(CanvasWidget { }.expand(), 5.0)
        .with_flex_child(
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

fn main() -> Result<(), PlatformError> {
    let width = 800usize;
    let height = 600usize;

    let main_window = WindowDesc::new(build_ui())
        .title(LocalizedString::new("Raycasting"))
        .window_size((width as f64, height as f64));

    let mut initial_state = AppState::new();

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)?;

    Ok(())
}
