use crate::color::Color;
use image::{ImageResult, RgbImage};
use std::path::Path;

pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

pub type ScreenPoint = Point<u32>;

pub trait Drawable {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn clear(&mut self, color: Color);
    fn line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: Color);
    fn triangle(&mut self, u: &ScreenPoint, v: &ScreenPoint, w: &ScreenPoint, color: Color);
}

pub struct Image {
    image: RgbImage,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            image: RgbImage::new(width, height),
        }
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

    fn triangle(&mut self, u: &ScreenPoint, v: &ScreenPoint, w: &ScreenPoint, color: Color) {
        let mut points = [u, v, w];
        points.sort_by_key(|p| p.y);
        let min_y = points[0].y;
        let max_y = points[2].y;

        for y in min_y..=max_y {
            let x0 = intersect_y(points[0], points[2], y);
            let x1 = if y <= points[1].y {
                intersect_y(points[0], points[1], y)
            } else {
                intersect_y(points[1], points[2], y)
            };
            let left_x = x0.min(x1).ceil() as u32;
            let right_x = x0.max(x1) as u32;
            self.line(left_x, y, right_x, y, color);
        }

        let line_color = Color(255, 50, 255);
        self.line(u.x, u.y, v.x, v.y, line_color);
        self.line(v.x, v.y, w.x, w.y, line_color);
        self.line(u.x, u.y, w.x, w.y, line_color);
    }
}

fn intersect_y(p1: &ScreenPoint, p2: &ScreenPoint, y: u32) -> f64 {
    if p1.x == p2.x {
        return p1.x as f64;
    }
    let x1 = p1.x as f64;
    let y1 = p1.y as f64;
    let x2 = p2.x as f64;
    let y2 = p2.y as f64;
    let delta = (y2 - y1) / (x2 - x1);
    (y as f64 - y1) / delta + x1
}

#[test]
fn test_intersect_y() {
    let p1 = ScreenPoint::new(5, 10);
    let p2 = ScreenPoint::new(5, 20);
    assert_eq!(intersect_y(&p1, &p2, 15), 5.0);

    let p1 = ScreenPoint::new(5, 10);
    let p2 = ScreenPoint::new(20, 20);
    assert_eq!(intersect_y(&p1, &p2, 15), 12.5);
}
