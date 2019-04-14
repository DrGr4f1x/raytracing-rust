use std::fs::{File};
use std::io::{BufWriter, Write};

pub mod vec3;

fn main() {
    let nx = 1280;
    let ny = 720;
    
    let file = File::create("image.ppm").expect("Unable to create file");
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = vec3::Vec3::new((i as f32) / (nx as f32), (j as f32) / (ny as f32), 0.2);
            let ir : i32 = (255.99 * col.r()) as i32;
            let ig : i32 = (255.99 * col.g()) as i32;
            let ib : i32 = (255.99 * col.b()) as i32;
            buf_writer.write_fmt(format_args!("{} {} {}\n", ir, ig, ib));
        }
    }
}