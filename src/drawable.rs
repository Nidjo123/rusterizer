use std::path::Path;

use image::{ImageResult, RgbImage};

use crate::color::Color;
use crate::DrawStyle;

#[derive(Debug)]
pub struct Point<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point { x, y, z }
    }
}

fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

impl<T> Point<T>
where
    T: Copy + PartialOrd,
{
    pub fn min(&self, other: &Point<T>) -> Self {
        Point {
            x: min(self.x, other.x),
            y: min(self.y, other.y),
            z: min(self.z, other.z),
        }
    }

    pub fn max(&self, other: &Point<T>) -> Self {
        Point {
            x: max(self.x, other.x),
            y: max(self.y, other.y),
            z: max(self.z, other.z),
        }
    }
}

impl From<&ScreenPoint> for Point3f {
    fn from(point: &ScreenPoint) -> Self {
        Point {
            x: point.x as f64,
            y: point.y as f64,
            z: point.z as f64,
        }
    }
}

impl From<ScreenPoint> for Point3f {
    fn from(point: ScreenPoint) -> Self {
        Point {
            x: point.x as f64,
            y: point.y as f64,
            z: point.z as f64,
        }
    }
}

impl From<&Point3f> for ScreenPoint {
    fn from(point: &Point3f) -> Self {
        Point {
            x: point.x as u32,
            y: point.y as u32,
            z: point.z as u32,
        }
    }
}

impl From<Point3f> for ScreenPoint {
    fn from(point: Point3f) -> Self {
        Point {
            x: point.x as u32,
            y: point.y as u32,
            z: point.z as u32,
        }
    }
}

pub type ScreenPoint = Point<u32>;
pub type Point3f = Point<f64>;

pub trait Drawable {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn clear(&mut self, color: Color);
    fn point(&mut self, x: u32, y: u32, color: Color);
    fn line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: Color);
    fn triangle(
        &mut self,
        a: &Point3f,
        b: &Point3f,
        c: &Point3f,
        draw_style: &DrawStyle,
        intensity: f64,
    );
    fn check_and_set_zbuf(&mut self, x: u32, y: u32, z_value: f64) -> bool;
}

pub struct Image {
    image: RgbImage,
    z_buffer: Vec<f64>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            image: RgbImage::new(width, height),
            z_buffer: vec![f64::NEG_INFINITY; (width * height) as usize],
        }
    }

    pub fn save<Q: AsRef<Path>>(&self, path: Q) -> ImageResult<()> {
        image::DynamicImage::from(self.image.clone())
            .flipv()
            .save(path)
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

    fn point(&mut self, x: u32, y: u32, color: Color) {
        self.image.put_pixel(x, y, color.into());
    }

    fn line(&mut self, mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32, color: Color) {
        // TODO: clip inside drawable bounds
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

    fn triangle(
        &mut self,
        a: &Point3f,
        b: &Point3f,
        c: &Point3f,
        draw_style: &DrawStyle,
        intensity: f64,
    ) {
        match draw_style {
            &DrawStyle::Wireframe(color) => {
                triangle_wireframe(self, &a.into(), &b.into(), &c.into(), color)
            }
            _ => triangle_barycentric(self, a, b, c, draw_style, intensity),
        };
    }

    fn check_and_set_zbuf(&mut self, x: u32, y: u32, z_value: f64) -> bool {
        let idx = (y * self.height() + x) as usize;
        if self.z_buffer[idx] < z_value {
            self.z_buffer[idx] = z_value;
            true
        } else {
            false
        }
    }
}

#[allow(unused)]
fn triangle_wireframe(
    image: &mut Image,
    u: &ScreenPoint,
    v: &ScreenPoint,
    w: &ScreenPoint,
    color: Color,
) {
    image.line(u.x, u.y, v.x, v.y, color);
    image.line(v.x, v.y, w.x, w.y, color);
    image.line(u.x, u.y, w.x, w.y, color);
}

#[allow(unused)]
fn triangle_line_sweep(
    image: &mut Image,
    u: &ScreenPoint,
    v: &ScreenPoint,
    w: &ScreenPoint,
    color: Color,
) {
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
        image.line(left_x, y, right_x, y, color);
    }
}

const LIMIT: f64 = 1e-9;

fn determine_color(bary_coords: (f64, f64, f64), draw_style: &DrawStyle, intensity: f64) -> Color {
    match draw_style {
        &DrawStyle::Textured(tex, (tp1, tp2, tp3)) => {
            let (a, b, c) = bary_coords;
            let u = a * tp1.x + b * tp2.x + c * tp3.x;
            let v = a * tp1.y + b * tp2.y + c * tp3.y;
            let x = (u * tex.width() as f64) as u32;
            let y = (v * tex.height() as f64) as u32;
            let color = tex.get_pixel(x, y);
            Color::from(*color).scale(intensity)
        }
        DrawStyle::Filled(color) => color.scale(intensity),
        DrawStyle::FilledRandom => Color::random().scale(intensity),
        DrawStyle::Wireframe(_) => panic!("should not end here"),
    }
}

fn triangle_barycentric(
    image: &mut Image,
    p1: &Point3f,
    p2: &Point3f,
    p3: &Point3f,
    draw_style: &DrawStyle,
    intensity: f64,
) {
    let min_p: ScreenPoint = ScreenPoint::from(p1.min(p2).min(p3));
    let max_p: ScreenPoint = ScreenPoint::from(p1.max(p2).max(p3));

    let width = image.width();
    let height = image.height();
    let min_p = ScreenPoint::new(min_p.x.min(width - 1), min_p.y.min(height - 1), min_p.z);
    let max_p = ScreenPoint::new(max_p.x.min(width - 1), max_p.y.min(height - 1), max_p.z);

    for y in min_p.y..=max_p.y {
        for x in min_p.x..=max_p.x {
            let p = ScreenPoint::new(x, y, 0).into();
            let (a, b, c) = barycentric(&p1, &p2, &p3, &p);
            if a >= -LIMIT && b >= -LIMIT && c >= -LIMIT {
                let z = a * p1.z + b * p2.z + c * p3.z;
                if image.check_and_set_zbuf(x, y, z) {
                    let color = determine_color((a, b, c), draw_style, intensity);
                    image.point(x, y, color);
                }
            }
        }
    }
}

fn barycentric(p1: &Point3f, p2: &Point3f, p3: &Point3f, p: &Point3f) -> (f64, f64, f64) {
    let denom = (p1.x - p3.x) * (p2.y - p3.y) - (p1.y - p3.y) * (p2.x - p3.x);
    let lambda1 = ((p.x - p3.x) * (p2.y - p3.y) + (p3.x - p2.x) * (p.y - p3.y)) / denom;
    let lambda2 = ((p3.x - p.x) * (p1.y - p3.y) + (p3.x - p1.x) * (p3.y - p.y)) / denom;
    (lambda1, lambda2, 1.0 - lambda1 - lambda2)
}

#[test]
fn test_barycentric() {
    let p1 = Point3f::new(5., 5., 0.);
    let p2 = Point3f::new(10., 5., 0.);
    let p3 = Point3f::new(10., 7., 0.);

    fn close_enough(res: (f64, f64, f64), ref_vals: (f64, f64, f64)) -> bool {
        const EPS: f64 = 1e-6;
        (res.0 - ref_vals.0).abs() <= EPS
            && (res.1 - ref_vals.1).abs() <= EPS
            && (res.2 - ref_vals.2).abs() <= EPS
    }
    assert!(close_enough(
        barycentric(&p1, &p2, &p3, &p1),
        (1.0, 0.0, 0.0)
    ));
    assert!(close_enough(
        barycentric(&p1, &p2, &p3, &p2),
        (0.0, 1.0, 0.0)
    ));
    assert!(close_enough(
        barycentric(&p1, &p2, &p3, &p3),
        (0.0, 0.0, 1.0)
    ));

    let outside = Point3f::new(100., 100., 0.);
    let (a, b, c) = barycentric(&p1, &p2, &p3, &outside);
    assert!([a, b, c].iter().any(|&x| x < 0.0));
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
    let p1 = ScreenPoint::new(5, 10, 0);
    let p2 = ScreenPoint::new(5, 20, 0);
    assert_eq!(intersect_y(&p1, &p2, 15), 5.0);

    let p1 = ScreenPoint::new(5, 10, 0);
    let p2 = ScreenPoint::new(20, 20, 0);
    assert_eq!(intersect_y(&p1, &p2, 15), 12.5);
}
