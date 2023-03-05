use std::path::Path;
use image::{ImageResult, RgbImage};
use crate::color::Color;

pub trait Drawable {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn clear(&mut self, color: Color);
    fn line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: Color);
}

pub struct Image {
    image: RgbImage,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image { image: RgbImage::new(width, height) }
    }

    pub fn save<Q: AsRef<Path>>(&self, path: Q) -> ImageResult<()> {
        self.image.save(path)
    }
}

impl Drawable for Image {
    fn width(&self) -> u32 {
        self.image.width()
    }

    fn height(&self) -> u32 {
        self.image.height()
    }

    fn clear(&mut self, color: Color) {
        for pixel in self.image.pixels_mut() {
            *pixel = color.into();
        }
    }

    fn line(&mut self, mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32, color: Color) {
        let steep;
        if x0.abs_diff(x1) < y0.abs_diff(y1) {
            steep = true;
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
        } else {
            steep = false;
        }

        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let dx = (x1 - x0) as i32;
        let dy = y1 as i32 - y0 as i32;

        let derror2 = dy.abs() * 2;
        let mut error2 = 0;
        let mut y = y0 as i32;
        for x in x0..=x1 {
            if steep {
                self.image.put_pixel(y as u32, x, color.into());
            } else {
                self.image.put_pixel(x, y as u32, color.into());
            }
            error2 += derror2;
            if error2 > dx {
                y += dy.signum();
                error2 -= dx * 2;
            }
        }
    }
}
