use std::f32;
use rand::random;

extern crate raytracer;
use raytracer::core::vec3::*;
use raytracer::core::ray::*;
use raytracer::core::hitable::*;
use raytracer::core::hitable_list::*;
use raytracer::core::material::*;
use raytracer::core::sphere::*;
use raytracer::core::camera::*;
use raytracer::core::image::*;

fn color(r: Ray, world: &Hitable, depth: i32) -> Vec3 {
    let mut rec = HitRecord::new();

    if world.hit(r, 0.001, f32::MAX, &mut rec) {
        if depth < 50 {
            match rec.material.scatter(&r, &rec) {
                Some((attenuation, scattered)) => attenuation * color(scattered, world, depth + 1),
                None => Vec3::zero()
            }
        }
        else {
            Vec3::zero()
        }
    }
    else
    {
        let unit_direction: Vec3 = unit_vector(r.direction());
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 1280;
    let ny = 720;
    let ns = 16;

    let mut world = HitableList::new();

    // Ground plane
    let sphere = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Material::lambertian(Vec3::new(0.5, 0.5, 0.5)));
    world.list.push(Box::new(sphere));

    // Central spheres
    let sphere = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Material::dielectric(1.5));
    world.list.push(Box::new(sphere));
    let sphere = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Material::lambertian(Vec3::new(0.4, 0.2, 0.1)));
    world.list.push(Box::new(sphere));
    let sphere = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Material::metal(Vec3::new(0.7, 0.6, 0.5)));
    world.list.push(Box::new(sphere));

    // Random spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f32>();
            let center = Vec3::new((a as f32) + 0.9 * random::<f32>(), 0.2, (b as f32) + 0.9 * random::<f32>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let sphere = Sphere::new(center, 0.2, Material::lambertian(Vec3::new(random::<f32>()*random::<f32>(), random::<f32>()*random::<f32>(), random::<f32>()*random::<f32>())));
                    world.list.push(Box::new(sphere));
                }
                else if choose_mat < 0.95 {
                    let sphere = Sphere::new(center, 0.2, Material::metal(Vec3::new(0.5 * (1.0 + random::<f32>()), 0.5 * (1.0 + random::<f32>()), 0.5 * random::<f32>())));
                    world.list.push(Box::new(sphere));
                }
                else {
                    let sphere = Sphere::new(center, 0.2, Material::dielectric(1.5));
                    world.list.push(Box::new(sphere));
                }
            }
        }
    }

    // Camera setup
    let fovy: f32 = 20.0;
    let aspect = (nx as f32) / (ny as f32);
    let pos = Vec3::new(13.0, 2.0, 3.0);
    let target = Vec3::zero();
    let up = Vec3::unit_y();
    let dist_to_focus = (pos - target).length();
    let aperture: f32 = 0.1;
    let cam = Camera::look_at(pos, target, up, fovy, aspect, aperture, dist_to_focus);

    // Output image
    let mut image = Image::new(nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();

            for _s in 0..ns {
                let u = ((i as f32) + random::<f32>()) / (nx as f32);
                let v = ((j as f32) + random::<f32>()) / (ny as f32);
                let r = cam.get_ray(u, v);
                
                col += color(r, &world, 0);
            }

            col /= ns as f32;
            col = Vec3::new(f32::sqrt(col.r()), f32::sqrt(col.g()), f32::sqrt(col.b()));

            image.set_pixel(i, j, col);
        }
    }

    image.save_as("image.ppm");
}