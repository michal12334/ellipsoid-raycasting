mod pixel;

use druid::widget::prelude::*;
use druid::{AppLauncher, Color, LocalizedString, PlatformError, Rect, Widget, WindowDesc};
use druid::piet::{ImageBuf, ImageFormat};
use im::Vector;
use crate::pixel::Pixel;

#[derive(Clone, Data)]
struct AppState {
    canvas: Vector<Vector<Pixel>>,
    width: usize,
    height: usize,
}

impl AppState {
    fn new(width: usize, height: usize) -> Self {
        let canvas = Vector::from((0..height).map(|_| Vector::from(vec![Pixel::new(); width])).collect::<Vec<Vector<Pixel>>>());
        AppState { canvas, width, height }
    }

    fn change_pixel_color(&mut self, x: usize, y: usize, color: Color) {
        self.canvas[y][x] = Pixel::from(color.as_rgba8());
    }
}

struct CanvasWidget;

impl Widget<AppState> for CanvasWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::MouseDown(mouse) => {
                let (x, y) = (mouse.pos.x as usize, mouse.pos.y as usize);
                data.change_pixel_color(x, y, Color::BLUE);
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppState, _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(&mut self, _layout_ctx: &mut LayoutCtx, _bc: &BoxConstraints, _data: &AppState, _env: &Env) -> Size {
        _bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        let image = ImageBuf
            ::from_raw(
                data.canvas.iter()
                    .flat_map(|f| f)
                    .map(|f| [f.r, f.g, f.b, f.a])
                    .flatten()
                    .collect::<Vec<u8>>(),
                ImageFormat::RgbaSeparate,
                data.width,
                data.height)
            .to_image(ctx.render_ctx);
        let rect = ctx.size().to_rect();
        ctx.draw_image(&image, rect, druid::piet::InterpolationMode::Bilinear);
    }
}

fn main() -> Result<(), PlatformError> {
    let width = 800usize;
    let height = 600usize;

    let main_window = WindowDesc::new(CanvasWidget { })
        .title(LocalizedString::new("Raycasting"))
        .window_size((width as f64, height as f64));

    let mut initial_state = AppState::new(width, height);

    for i in 0..width {
        for j in 0..height {
            let x = (i as i32 - (width as i32 / 2)) as f32 / ((width / 2) as f32);
            let y = (j as i32 - (height as i32 / 2)) as f32 / ((height / 2) as f32) * (-1.0);

            if x*x + y*y < 0.25 {
                initial_state.change_pixel_color(i, j, Color::YELLOW);
            }
        }
    }

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)?;

    Ok(())
}
