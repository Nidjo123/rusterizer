use color::Color;
use drawable::Image;
use wavefront_obj::obj::Primitive;

mod color;
mod drawable;

use crate::drawable::{Drawable, ScreenPoint};

enum DrawStyle {
    Wireframe,
    Filled(Color),
    FilledRandom,
}

fn draw_obj(image: &mut Image, obj: &wavefront_obj::obj::Object, draw_style: DrawStyle) {
    for geometry in &obj.geometry {
        for shape in &geometry.shapes {
            match shape.primitive {
                Primitive::Triangle((idx1, _, _), (idx2, _, _), (idx3, _, _)) => {
                    let v1 = &obj.vertices[idx1];
                    let v2 = &obj.vertices[idx2];
                    let v3 = &obj.vertices[idx3];
                    let scale_x = image.width() as f64 / 2.0;
                    let scale_y = image.height() as f64 / 2.0;
                    let transform_component = |x, offset, scale| ((x + offset) * scale) as u32;
                    let x1 = transform_component(v1.x, 1.0, scale_x);
                    let y1 = transform_component(-v1.y, 1.0, scale_y);
                    let x2 = transform_component(v2.x, 1.0, scale_x);
                    let y2 = transform_component(-v2.y, 1.0, scale_y);
                    let x3 = transform_component(v3.x, 1.0, scale_x);
                    let y3 = transform_component(-v3.y, 1.0, scale_y);
                    let color = match draw_style {
                        DrawStyle::Wireframe => Color(255, 255, 255),
                        DrawStyle::Filled(color) => color,
                        DrawStyle::FilledRandom => Color::random(),
                    };
                    image.triangle(
                        &ScreenPoint::new(x1, y1),
                        &ScreenPoint::new(x2, y2),
                        &ScreenPoint::new(x3, y3),
                        color,
                        matches!(draw_style, DrawStyle::Wireframe),
                    );
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let mut image = Image::new(512, 512);

    image.clear(Color(50, 50, 50));

    let obj_path = std::env::args().skip(1).next();
    if let Some(path) = obj_path {
        if let Ok(content) = std::fs::read_to_string(path) {
            let obj_set = wavefront_obj::obj::parse(content).expect("obj parsing error");
            for obj in &obj_set.objects {
                draw_obj(&mut image, obj, DrawStyle::FilledRandom);
            }
        }
    }

    if let Err(e) = image.save("output.png") {
        eprintln!("Error: {}", e);
    }
}
