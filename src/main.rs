use drawable::Image;
use color::Color;

mod color;
mod drawable;

use drawable::Drawable;

fn main() {
    let mut image = Image::new(256, 256);

    image.clear(Color(50, 50, 50));

    let max_x = image.width() - 1;
    let max_y = image.height() - 1;

    let white = Color(255, 255, 255);
    let red = Color(255, 0, 0);
    image.line(5, 5, 100, 250, red);
    image.line(5, 100, 100, 5, red);
    image.line(0, 0, max_x, max_y, white);
    image.line(0, max_y, max_x, 0, white);

    if let Err(e) = image.save("/home/nikola/Downloads/image.png") {
        eprintln!("Error: {}", e);
    }
}
