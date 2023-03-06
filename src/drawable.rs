use crate::color::Color;
use image::{ImageResult, RgbImage};
use std::path::Path;

#[derive(Debug)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Copy + Ord,
{
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn min(&self, other: &Point<T>) -> Self {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(&self, other: &Point<T>) -> Self {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

impl From<&ScreenPoint> for Point<f64> {
    fn from(point: &ScreenPoint) -> Self {
        Point {
            x: point.x as f64,
            y: point.y as f64,
        }
    }
}

pub type ScreenPoint = Point<u32>;

pub trait Drawable {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn clear(&mut self, color: Color);
    fn point(&mut self, x: u32, y: u32, color: Color);
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

    fn point(&mut self, x: u32, y: u32, color: Color) {
        self.image.put_pixel(x, y, color.into());
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
        // triangle_line_sweep(self, u, v, w, color);
        triangle_barycentric(self, u, v, w, color);
    }
}

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

    let line_color = Color(255, 50, 255);
    image.line(u.x, u.y, v.x, v.y, line_color);
    image.line(v.x, v.y, w.x, w.y, line_color);
    image.line(u.x, u.y, w.x, w.y, line_color);
}

fn triangle_barycentric(
    image: &mut Image,
    u: &ScreenPoint,
    v: &ScreenPoint,
    w: &ScreenPoint,
    color: Color,
) {
    let min_p = u.min(v).min(w);
    let max_p = u.max(v).max(w);

    for y in min_p.y..=max_p.y {
        for x in min_p.x..=max_p.x {
            let p = ScreenPoint::new(x, y);
            if is_point_inside_triangle(u, v, w, &p) {
                image.point(x, y, color);
            }
        }
    }

    let line_color = Color(255, 50, 255);
    image.line(u.x, u.y, v.x, v.y, line_color);
    image.line(v.x, v.y, w.x, w.y, line_color);
    image.line(u.x, u.y, w.x, w.y, line_color);
}

fn barycentric(
    p1: &ScreenPoint,
    p2: &ScreenPoint,
    p3: &ScreenPoint,
    p: &ScreenPoint,
) -> (f64, f64, f64) {
    let p1: Point<f64> = p1.into();
    let p2: Point<f64> = p2.into();
    let p3: Point<f64> = p3.into();
    let p: Point<f64> = p.into();

    let denom = (p1.x - p3.x) * (p2.y - p3.y) - (p1.y - p3.y) * (p2.x - p3.x);
    let lambda1 = ((p.x - p3.x) * (p2.y - p3.y) + (p3.x - p2.x) * (p.y - p3.y)) / denom;
    let lambda2 = ((p3.x - p.x) * (p1.y - p3.y) + (p3.x - p1.x) * (p3.y - p.y)) / denom;
    (lambda1, lambda2, 1.0 - lambda1 - lambda2)
}

fn is_point_inside_triangle(
    p1: &ScreenPoint,
    p2: &ScreenPoint,
    p3: &ScreenPoint,
    p: &ScreenPoint,
) -> bool {
    let (a, b, c) = barycentric(p1, p2, p3, p);
    a >= 0.0 && b >= 0.0 && c >= 0.0
}

#[test]
fn test_is_point_inside_triangle() {
    let p1 = ScreenPoint::new(5, 5);
    let p2 = ScreenPoint::new(10, 5);
    let p3 = ScreenPoint::new(10, 7);
    assert!(is_point_inside_triangle(&p1, &p2, &p3, &p1));
    assert!(is_point_inside_triangle(&p1, &p2, &p3, &p2));
    assert!(is_point_inside_triangle(&p1, &p2, &p3, &p3));
    assert!(is_point_inside_triangle(
        &p1,
        &p2,
        &p3,
        &ScreenPoint::new(8, 6)
    ));

    assert!(!is_point_inside_triangle(
        &p1,
        &p2,
        &p3,
        &ScreenPoint::new(5, 4)
    ));
    assert!(!is_point_inside_triangle(
        &p1,
        &p2,
        &p3,
        &ScreenPoint::new(100, 100)
    ));
    assert!(!is_point_inside_triangle(
        &p1,
        &p2,
        &p3,
        &ScreenPoint::new(5, 7)
    ));
}

#[test]
fn test_barycentric() {
    let p1 = ScreenPoint::new(5, 5);
    let p2 = ScreenPoint::new(10, 5);
    let p3 = ScreenPoint::new(10, 7);

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

    let outside = ScreenPoint::new(100, 100);
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
    let p1 = ScreenPoint::new(5, 10);
    let p2 = ScreenPoint::new(5, 20);
    assert_eq!(intersect_y(&p1, &p2, 15), 5.0);

    let p1 = ScreenPoint::new(5, 10);
    let p2 = ScreenPoint::new(20, 20);
    assert_eq!(intersect_y(&p1, &p2, 15), 12.5);
}
