use druid::Data;

#[derive(Clone, Data)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub fn new() -> Self {
        Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn from(v: (u8, u8, u8, u8)) -> Self {
        Pixel {
            r: v.0,
            g: v.1,
            b: v.2,
            a: v.3,
        }
    }

    pub fn as_rgba8(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}
