use std::os::unix::raw::mode_t;
use druid::{BoxConstraints, Color, Env, Event, EventCtx, ImageBuf, KbKey, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, RenderContext, Size, UpdateCtx, Widget};
use druid::piet::ImageFormat;
use nalgebra::{Matrix, Matrix4, OMatrix, U4, Vector3, Vector4};
use crate::AppState;

pub struct Canvas {
    canvas: Vec<u8>,
    a: f64,
    b: f64,
    c: f64,
    m: f64,
    scale: f64,
    rotation: (f64, f64, f64),
    position: (f64, f64, f64),
    accuracy: usize,
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
            rotation: (0.0, 0.0, 0.0),
            position: (0.0, 0.0, 0.0),
            accuracy: 1,
            scale: 1.0,
            width: 0,
            height: 0,
        }
    }

    fn draw(&mut self, a: f64, b: f64, c: f64, m: f64, scale: f64, rotation: (f64, f64, f64), position: (f64, f64, f64), accuracy: usize, width: usize, height: usize) {
        if !self.update(a, b, c, m, scale, rotation, position, accuracy, width, height) {
            return;
        }

        self.canvas.resize(width * height * 4, 0);
        
        let m = m as i32;
        
        let d = self.get_d();
        let ap = d.column(0).iter().sum::<f32>();
        let bp = d.column(1).iter().sum::<f32>();
        let cp = d.column(2).iter().sum::<f32>();
        let dp = d.column(3).iter().sum::<f32>();

        for i in 0..width {
            for j in 0..height {
                if i % self.accuracy != 0 || j % self.accuracy != 0 {
                    let colored_index = (j / self.accuracy * self.accuracy * width + i / self.accuracy * self.accuracy) * 4;
                    let index = (j * width + i) * 4;
                    (self.canvas[index], self.canvas[index + 1], self.canvas[index + 2], self.canvas[index + 3]) = 
                        (self.canvas[colored_index], self.canvas[colored_index + 1], self.canvas[colored_index + 2], self.canvas[colored_index + 3]);
                    continue;
                } 
                
                let x = (i as i32 - (width as i32 / 2)) as f32 / ((width / 2) as f32);
                let y = (j as i32 - (height as i32 / 2)) as f32 / ((height / 2) as f32) * (-1.0);
                let index = (j * width + i) * 4;

                let a = d.m33;
                let b = d.m13*x + d.m23*y + d.m31*x + d.m32*y + d.m34;
                let b2 = b * b;
                
                let delta =
                    b2
                    - 4.0
                    * (
                        d.m11*x*x + d.m12*x*y + d.m14*x
                        + d.m21*x*y + d.m22*y*y + d.m24*y
                        + d.m41*x + d.m42*y + d.m44
                    )
                    * a;
                
                
                if delta >= 0.0 {
                    let z = if a > 0.0  
                        { (-b + delta.sqrt()) / (2.0 * a) } 
                        else 
                        { (-b - delta.sqrt()) / (2.0 * a) };

                    let n = Vector3
                        ::new(
                            2.0*d.m11*x + d.m12*y + d.m13*z + d.m14 + d.m21*y + d.m31*z + d.m41,
                            d.m12*x + d.m21*x + 2.0*d.m22*y + d.m23*z + d.m24 + d.m32*z + d.m42,
                            d.m13*x + d.m23*y + d.m31*x + d.m32*y + 2.0*d.m33*z + d.m34 + d.m43
                        )
                        .normalize();
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

    fn update(&mut self, a: f64, b: f64, c: f64, m: f64, scale: f64, rotation: (f64, f64, f64), position: (f64, f64, f64), accuracy: usize, width: usize, height: usize) -> bool {
        let result = self.a != a 
            || self.b != b 
            || self.c != c 
            || self.m != m 
            || self.scale != scale 
            || self.rotation != rotation 
            || self.position != position
            || self.accuracy != accuracy
            || self.width != width 
            || self.height != height;

        self.a = a;
        self.b = b;
        self.c = c;
        self.m = m;
        self.scale = scale;
        self.rotation = rotation;
        self.position = position;
        self.accuracy = accuracy;
        self.width = width;
        self.height = height;

        result
    }
    
    fn get_d(&self) -> Matrix4<f32> {
        let d = Matrix4::from_diagonal(&Vector4::new(self.a as f32, self.b as f32, self.c as f32, -1.0));
        let m = self.get_position_matrix() 
            * self.get_rotation_matrix() 
            * Matrix4::from_diagonal(&Vector4::new(self.scale as f32, self.scale as f32, self.scale as f32, 1.0));
        let mi = m.try_inverse().unwrap_or_else(|| Matrix4::identity());
        return mi.transpose() * d * mi;
    }
    
    fn get_rotation_matrix(&self) -> Matrix4<f32> {
        let x = self.rotation.0 as f32;
        let y = self.rotation.1 as f32;
        let z = self.rotation.2 as f32;
        let rx = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, x.cos(), -x.sin(), 0.0,
            0.0, x.sin(), x.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        let ry = Matrix4::new(
            y.cos(), 0.0, y.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            -y.sin(), 0.0, y.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        let rz = Matrix4::new(
            z.cos(), -z.sin(), 0.0, 0.0,
            z.sin(), z.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        return rx * ry * rz;
    }
    
    fn get_position_matrix(&self) -> Matrix4<f32> {
        Matrix4::new(
            1.0, 0.0, 0.0, self.position.0 as f32,
            0.0, 1.0, 0.0, self.position.1 as f32,
            0.0, 0.0, 1.0, self.position.2 as f32,
            0.0, 0.0, 0.0, 1.0
        )
    }
}

impl Widget<AppState> for Canvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::KeyDown(k) => {
                match k.key {
                    KbKey::Control => data.ctrl_clicked = true,
                    KbKey::Shift => data.shift_clicked = true,
                    _ => {},
                }
            }
            Event::KeyUp(k) => {
                match k.key {
                    KbKey::Control => data.ctrl_clicked = false,
                    KbKey::Shift => data.shift_clicked = false,
                    _ => {},
                }
            }
            Event::Wheel(m) => {
                ctx.request_focus();
                if data.ctrl_clicked  {
                    data.rotation.2 += m.wheel_delta.y / -1000.0;
                    println!("rotation: {:?}", data.rotation);
                } else if data.shift_clicked {
                    data.position.2 += m.wheel_delta.x / -1000.0;
                    println!("position: {:?}", data.position);
                } else {
                    data.scale += m.wheel_delta.y / -1000.0;
                    println!("scale: {}", data.scale);
                }
            }
            Event::MouseDown(m) => {
                match m.button {
                    MouseButton::Right => {
                        data.right_button_clicked = true;
                        data.right_button_position = (m.pos.x, m.pos.y);
                    }
                    MouseButton::Left => {
                        data.left_button_clicked = true;
                        data.left_button_position = (m.pos.x, m.pos.y);
                    }
                    _ => {},
                }
            }
            Event::MouseUp(m) => {
                match m.button {
                    MouseButton::Right => data.right_button_clicked = false,
                    MouseButton::Left => data.left_button_clicked = false,
                    _ => {},
                }
            }
            Event::MouseMove(m) => {
                if m.buttons.contains(MouseButton::Right) {
                    data.rotation.0 += (data.right_button_position.1 - m.pos.y) / 10000.0;
                    data.rotation.1 += (m.pos.x - data.right_button_position.0) / 10000.0;
                    println!("rotation: {:?}", data.rotation);
                }
                if m.buttons.contains(MouseButton::Left) {
                    data.position.0 += (m.pos.x - data.left_button_position.0) / self.width as f64 / 50.0;
                    data.position.1 += (data.left_button_position.1 - m.pos.y) / self.height as f64 / 50.0;
                    println!("position: {:?}", data.position);
                }
            }
            _ => {}
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppState, _env: &Env) { }

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

        self.draw(data.a, data.b, data.c, data.m, data.scale, data.rotation, data.position, data.accuracy, width, height);

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
