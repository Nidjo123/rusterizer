use wavefront_obj::obj::{Object, Primitive, Vertex};

use color::Color;
use drawable::Image;
use math::Vec3f;

use crate::drawable::{Drawable, Point3f};

mod color;
mod drawable;
mod math;

pub type Intensity = f64;

#[allow(unused)]
pub enum DrawStyle<'a, 'b> {
    Wireframe(Color),
    Filled(Color),
    FilledRandom,
    Textured(&'a image::RgbImage, (&'b Point3f, &'b Point3f, &'b Point3f)),
}

fn calculate_intensity(v1: &Vertex, v2: &Vertex, v3: &Vertex, light_dir: &Vec3f) -> f64 {
    let u = Vec3f::new(v3.x - v1.x, v3.y - v1.y, v3.z - v1.z);
    let v = Vec3f::new(v2.x - v1.x, v2.y - v1.y, v2.z - v1.z);
    let normal = math::cross(&u, &v).normalized();
    math::dot(&normal, light_dir)
}

fn draw_obj(image: &mut Image, obj: &Object, draw_style: &DrawStyle) {
    let light_dir = Vec3f::new(0., 0., -1.);
    let scale_x = image.width() as f64 / 2.0;
    let scale_y = image.height() as f64 / 2.0;
    for geometry in &obj.geometry {
        for shape in &geometry.shapes {
            match shape.primitive {
                Primitive::Triangle((idx1, tidx1, _), (idx2, tidx2, _), (idx3, tidx3, _)) => {
                    let v1 = &obj.vertices[idx1];
                    let v2 = &obj.vertices[idx2];
                    let v3 = &obj.vertices[idx3];
                    let intensity = calculate_intensity(v1, v2, v3, &light_dir);
                    if intensity < 0.0 {
                        // not visible
                        continue;
                    }
                    let transform_component = |x, offset, scale| -> f64 { (x + offset) * scale };
                    let x1 = transform_component(v1.x, 1.0, scale_x);
                    let y1 = transform_component(v1.y, 1.0, scale_y);
                    let x2 = transform_component(v2.x, 1.0, scale_x);
                    let y2 = transform_component(v2.y, 1.0, scale_y);
                    let x3 = transform_component(v3.x, 1.0, scale_x);
                    let y3 = transform_component(v3.y, 1.0, scale_y);

                    if let DrawStyle::Textured(tex, _) = draw_style {
                        let tidx1 = tidx1.unwrap();
                        let tidx2 = tidx2.unwrap();
                        let tidx3 = tidx3.unwrap();
                        let tx1 = &obj.tex_vertices[tidx1];
                        let tx2 = &obj.tex_vertices[tidx2];
                        let tx3 = &obj.tex_vertices[tidx3];
                        let tx1 = Point3f::new(tx1.u, tx1.v, tx1.w);
                        let tx2 = Point3f::new(tx2.u, tx2.v, tx2.w);
                        let tx3 = Point3f::new(tx3.u, tx3.v, tx3.w);
                        image.triangle(
                            &Point3f::new(x1, y1, v1.z),
                            &Point3f::new(x2, y2, v2.z),
                            &Point3f::new(x3, y3, v3.z),
                            &DrawStyle::Textured(tex, (&tx1, &tx2, &tx3)),
                            intensity,
                        );
                    } else {
                        image.triangle(
                            &Point3f::new(x1, y1, v1.z),
                            &Point3f::new(x2, y2, v2.z),
                            &Point3f::new(x3, y3, v3.z),
                            draw_style,
                            intensity,
                        );
                    }
                }
                primitive => eprintln!("Skipping unknown shape {:?}", primitive),
            }
        }
    }
}

fn main() {
    let mut image = Image::new(512, 512);

    image.clear(Color(50, 50, 50));

    let mut args = std::env::args().skip(1);
    let obj_path = args.next();
    let tex_path = args.next();
    if let Some(path) = obj_path {
        if let Ok(content) = std::fs::read_to_string(path) {
            let obj_set = wavefront_obj::obj::parse(content).expect("obj parsing error");
            if let Some(path) = tex_path {
                if let Ok(dyn_image) = image::open(path) {
                    // flip it as we are drawing object flipped
                    let rgb_image = dyn_image.flipv().to_rgb8();
                    let p1 = Point3f::new(0., 0., 0.);
                    let draw_style = DrawStyle::Textured(&rgb_image, (&p1, &p1, &p1));
                    for obj in &obj_set.objects {
                        draw_obj(&mut image, obj, &draw_style);
                    }
                }
            } else {
                for obj in &obj_set.objects {
                    draw_obj(&mut image, obj, &DrawStyle::Filled(color::WHITE));
                }
            }
        }
    }

    if let Err(e) = image.save("output.png") {
        eprintln!("Error: {}", e);
    }
}
