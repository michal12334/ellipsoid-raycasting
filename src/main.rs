use druid::widget::prelude::*;
use druid::{AppLauncher, Color, LocalizedString, PlatformError, Rect, Widget, WindowDesc};
use druid::piet::{ImageBuf, ImageFormat};
use im::Vector;

#[derive(Clone, Data)]
struct AppState {
    canvas: Vector<u8>,
    width: usize,
    height: usize,
}

impl AppState {
    fn new(width: usize, height: usize) -> Self {
        let canvas = Vector::from(vec![255 as u8; width * height * 4]);
        AppState { canvas, width, height }
    }

    fn change_pixel_color(&mut self, x: usize, y: usize, color: Color) {
        let index = y * self.width + x;
        (self.canvas[4 * index], self.canvas[4 * index + 1], self.canvas[4 * index + 2], self.canvas[4 * index + 3]) = color.as_rgba8();
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
        let image = ImageBuf::from_raw(data.canvas.iter().map(|&f| f).collect::<Vec<u8>>(), ImageFormat::RgbaSeparate, data.width, data.height).to_image(ctx.render_ctx);
        let rect = ctx.size().to_rect();
        ctx.draw_image(&image, rect, druid::piet::InterpolationMode::Bilinear);
    }
}

fn main() -> Result<(), PlatformError> {
    let width = 800 as usize;
    let height = 600 as usize;

    let main_window = WindowDesc::new(CanvasWidget { })
        .title(LocalizedString::new("Raycasting"))
        .window_size((width as f64, height as f64));

    let initial_state = AppState::new(width, height);

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)?;

    Ok(())
}
