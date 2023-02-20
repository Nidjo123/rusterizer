use image::Rgb;

#[derive(Clone, Copy, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

impl Into<Rgb<u8>> for Color {
    fn into(self) -> Rgb<u8> {
        Rgb([self.0, self.1, self.2])
    }
}