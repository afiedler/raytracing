use image::{Rgba, RgbaImage};
use raylib::raytracer;

fn main() {
    println!("Raytracing...");
    let mut image = RgbaImage::new(400, 225);
    let buf = raytracer();
    let mut cursor = 0usize;
    for j in 0..225u32 {
        for i in 0..400u32 {
            image.put_pixel(
                i,
                j,
                Rgba([
                    buf[cursor],
                    buf[cursor + 1],
                    buf[cursor + 2],
                    buf[cursor + 3],
                ]),
            );

            cursor += 4;
        }
    }

    image.save_with_format("./output.png", image::ImageFormat::Png);
}
