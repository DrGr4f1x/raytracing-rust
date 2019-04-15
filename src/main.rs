use std::fs::{File};
use std::io::{BufWriter, Write};
use std::f32;
use rand::random;

pub mod core;
use crate::core::vec3::*;
use crate::core::ray::*;
use crate::core::hitable::*;
use crate::core::sphere::*;
use crate::core::hitable_list::*;
use crate::core::camera::*;
use crate::core::material::*;


fn color(r: Ray, world: &Hitable, depth: i32) -> Vec3 {
    let mut rec = HitRecord::new();

    if world.hit(r, 0.001, f32::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        if (depth < 50) && rec.material.scatter(&r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * color(scattered, world, depth + 1)
        }
        else {
            return Vec3::new(0.0, 0.0, 0.0)
        }
    }
    else
    {
        let mut unit_direction: Vec3 = r.direction();
        unit_direction.normalize();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 1280;
    let ny = 720;
    let ns = 16;

    let mut world = HitableList::new();
    let sphere0 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Material::as_lambertian(Vec3::new(0.8, 0.3, 0.3)));
    let sphere1 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Material::as_lambertian(Vec3::new(0.8, 0.8, 0.0)));
    let sphere2 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Material::as_metal(Vec3::new(0.8, 0.6, 0.2)));
    let sphere3 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Material::as_dielectric(1.5));
    let sphere4 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Material::as_dielectric(1.5));
    world.list.push(Box::new(sphere0));
    world.list.push(Box::new(sphere1));
    world.list.push(Box::new(sphere2));
    world.list.push(Box::new(sphere3));
    world.list.push(Box::new(sphere4));

    let fovy :f32 = 20.0;
    let aspect = (nx as f32) / (ny as f32);
    let pos = Vec3::new(3.0, 3.0, 2.0);
    let target = Vec3::new(0.0, 0.0, -1.0);
    let up = Vec3::unit_y();
    let dist_to_focus = (pos - target).length();
    let aperture = 2.0;
    let cam = Camera::look_at(pos, target, up, fovy, aspect, aperture, dist_to_focus);

    let file = File::create("image.ppm").expect("Unable to create file");
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);

            for _s in 0..ns {
                let u = ((i as f32) + random::<f32>()) / (nx as f32);
                let v = ((j as f32) + random::<f32>()) / (ny as f32);
                let r = cam.get_ray(u, v);
                
                col += color(r, &world, 0);
            }

            col /= ns as f32;
            col = Vec3::new(f32::sqrt(col.r()), f32::sqrt(col.g()), f32::sqrt(col.b()));

            let ir : i32 = (255.99 * col.r()) as i32;
            let ig : i32 = (255.99 * col.g()) as i32;
            let ib : i32 = (255.99 * col.b()) as i32;

            buf_writer.write_fmt(format_args!("{} {} {}\n", ir, ig, ib));
        }
        
    }
}