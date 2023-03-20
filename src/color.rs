use image::Rgb;

#[derive(Clone, Copy, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn random() -> Self {
        Color(rand::random(), rand::random(), rand::random())
    }

    pub fn scale(&self, x: f64) -> Self {
        let x = x.max(0.0);
        let r = (self.0 as f64) * x;
        let g = (self.1 as f64) * x;
        let b = (self.2 as f64) * x;
        Color(r as u8, g as u8, b as u8)
    }
}

impl Into<Rgb<u8>> for Color {
    fn into(self) -> Rgb<u8> {
        Rgb([self.0, self.1, self.2])
    }
}

impl From<Rgb<u8>> for Color {
    fn from(value: Rgb<u8>) -> Self {
        Color(value[0], value[1], value[2])
    }
}

pub const WHITE: Color = Color(255, 255, 255);
