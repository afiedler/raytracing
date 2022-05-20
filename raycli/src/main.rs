use image::{Rgba, RgbaImage};
use raylib::{random_scene, raytracer};

fn main() {
    println!("Raytracing...");
    let world = random_scene();

    rayon::scope(|s| {
        s.spawn(move |_| {
            let (width, height, buf) = raytracer(&world);
            let mut image = RgbaImage::new(width, height);
            let mut cursor = 0usize;
            for j in 0..height {
                for i in 0..width {
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

            image.save_with_format("./output-draft.png", image::ImageFormat::Png);
        });
    });
}
