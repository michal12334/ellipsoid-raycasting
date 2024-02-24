use druid::{BoxConstraints, Color, Env, Event, EventCtx, ImageBuf, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, RenderContext, Size, UpdateCtx, Widget};
use druid::piet::ImageFormat;
use nalgebra::Vector3;
use crate::AppState;

pub struct Canvas {
    canvas: Vec<u8>,
    a: f64,
    b: f64,
    c: f64,
    m: f64,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            canvas: Vec::new(),
            a: 1.0,
            b: 1.0,
            c: 1.0,
            m: 1.0,
            width: 0,
            height: 0,
        }
    }

    fn draw(&mut self, a: f64, b: f64, c: f64, m: f64, width: usize, height: usize) {
        if !self.update(a, b, c, m, width, height) {
            return;
        }

        self.canvas.resize(width * height * 4, 0);

        let a = a as f32;
        let b = b as f32;
        let c = c as f32;
        let m = m as i32;

        for i in 0..width {
            for j in 0..height {
                let x = (i as i32 - (width as i32 / 2)) as f32 / ((width / 2) as f32);
                let y = (j as i32 - (height as i32 / 2)) as f32 / ((height / 2) as f32) * (-1.0);
                let index = (j * width + i) * 4;

                let l = a * x * x + b * y * y;
                if l < 1.0 {
                    let z = (1.0 - l).sqrt() / c;

                    let n = Vector3::new(2.0 * a * x, 2.0 * b * y, 2.0 * c * z).normalize();
                    let v = Vector3::new(-x, -y, 100.0 - z).normalize();

                    let intensity = (n.dot(&v).powi(m) as f64 + 0.1).clamp(0.0, 1.0);

                    let yellow = Color::YELLOW.as_rgba();
                    let color = Color::rgb(yellow.0 * intensity, yellow.1 * intensity, yellow.2 * intensity);

                    (self.canvas[index], self.canvas[index + 1], self.canvas[index + 2], self.canvas[index + 3]) = color.as_rgba8();
                } else {
                    (self.canvas[index], self.canvas[index + 1], self.canvas[index + 2], self.canvas[index + 3]) = (0, 0, 0, 255);
                }
            }
        }
    }

    fn update(&mut self, a: f64, b: f64, c: f64, m: f64, width: usize, height: usize) -> bool {
        let result = self.a != a || self.b != b || self.c != c || self.m != m || self.width != width || self.height != height;

        self.a = a;
        self.b = b;
        self.c = c;
        self.m = m;
        self.width = width;
        self.height = height;

        result
    }
}

impl Widget<AppState> for Canvas {
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

        self.draw(data.a, data.b, data.c, data.m, width, height);

        let image = ImageBuf
        ::from_raw(
            self.canvas.clone(),
            ImageFormat::RgbaSeparate,
            width,
            height
        )
            .to_image(ctx.render_ctx);
        ctx.draw_image(&image, rect, druid::piet::InterpolationMode::Bilinear);
    }
}
