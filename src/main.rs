use wavefront_obj::obj::Primitive;
use drawable::Image;
use color::Color;

mod color;
mod drawable;

use drawable::Drawable;

fn draw_obj_wireframe(image: &mut Image, obj: &wavefront_obj::obj::Object, color: Color) {
    let width = image.width();
    let height = image.height();
    for geometry in &obj.geometry {
        for shape in &geometry.shapes {
            match shape.primitive {
                Primitive::Point(_) => {}
                Primitive::Line(_, _) => {}
                Primitive::Triangle((idx1, _, _),
                                    (idx2, _, _),
                                    (idx3, _, _)) => {
                    let v1 = &obj.vertices[idx1];
                    let v2 = &obj.vertices[idx2];
                    let v3 = &obj.vertices[idx3];
                    let vertices = [v1, v2, v3];
                    for i in 0..vertices.len() {
                        let u = vertices[i];
                        let v = vertices[(i + 1) % vertices.len()];
                        let x0 = (u.x + 1.0) * width as f64 / 2.0;
                        let y0 = (-u.y + 1.0) * height as f64 / 2.0;
                        let x1 = (v.x + 1.0) * width as f64 / 2.0;
                        let y1 = (-v.y + 1.0) * height as f64 / 2.0;
                        image.line((x0 as u32).min(width - 1),
                                   (y0 as u32).min(height - 1),
                                   (x1 as u32).min(width - 1),
                                   (y1 as u32).min(height - 1),
                                   color);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut image = Image::new(512, 512);

    image.clear(Color(50, 50, 50));

    let white = Color(255, 255, 255);

    let obj_path = std::env::args().skip(1).next();
    if let Some(path) = obj_path {
        if let Ok(content) = std::fs::read_to_string(path) {
            let obj_set = wavefront_obj::obj::parse(content).expect("obj parsing error");
            for obj in &obj_set.objects {
                draw_obj_wireframe(&mut image, obj, white);
            }
        }
    }

    if let Err(e) = image.save("output.png") {
        eprintln!("Error: {}", e);
    }
}
