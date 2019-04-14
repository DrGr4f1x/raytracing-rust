use std::fs::{File};
use std::io::{BufWriter, Write};

pub mod core;
use crate::core::vec3::Vec3;
use crate::core::ray::Ray;

fn color(r: Ray) -> Vec3 {
    let mut unit_direction: Vec3 = r.direction();
    unit_direction.normalize();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 1280;
    let ny = 720;
    
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let file = File::create("image.ppm").expect("Unable to create file");
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f32 = (i as f32) / (nx as f32);
            let v: f32 = (j as f32) / (ny as f32);
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let col = color(r);

            let ir : i32 = (255.99 * col.r()) as i32;
            let ig : i32 = (255.99 * col.g()) as i32;
            let ib : i32 = (255.99 * col.b()) as i32;

            buf_writer.write_fmt(format_args!("{} {} {}\n", ir, ig, ib));
        }
    }
}