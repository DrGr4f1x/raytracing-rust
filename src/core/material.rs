use super::vec3::*;
use super::ray::*;
use super::hitable::*;
use super::sampling::*;
use rand::random;

pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
                
        Some((self.albedo, Ray::new(rec.p, target - rec.p, r_in.time())))
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let mut uv = v;
    uv.normalize();
    let dt = dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - n * dt) - n * f32::sqrt(discriminant);
        true
    }
    else {
        false
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0)
}

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Metal { albedo }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut unit_vector = r_in.direction();
        unit_vector.normalize();
        let reflected = reflect(unit_vector, rec.normal);
        let scattered = Ray::new(rec.p, reflected, r_in.time());
                
        if  dot(scattered.direction(), rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        }
        else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric{ ref_idx }
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r_in.direction(), rec.normal);
        let scattered: Ray;
        let reflect_prob: f32;
        let dir_dot_normal = dot(r_in.direction(), rec.normal);
        let cosine = if dir_dot_normal > 0.0 { 
                self.ref_idx * dir_dot_normal / r_in.direction().length()
            } 
            else { 
                -dir_dot_normal / r_in.direction().length()
            };

        let outward_normal = if dir_dot_normal > 0.0 { -1.0 * rec.normal } else { rec.normal };
        let ni_over_nt = if dir_dot_normal > 0.0 { self.ref_idx } else { 1.0 / self.ref_idx };

        let mut refracted = Vec3::zero();
        if refract(r_in.direction(), outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = schlick(cosine, self.ref_idx);
        }
        else {
            reflect_prob = 1.0;
        }

        if random::<f32>() < reflect_prob {
            scattered = Ray::new(rec.p, reflected, r_in.time());
        }
        else {
            scattered = Ray::new(rec.p, refracted, r_in.time());
        }

        Some((Vec3::one(), scattered))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn lambertian(albedo: Vec3) -> Self {
        Material::Lambertian(Lambertian::new(albedo))
    }

    pub fn metal(albedo: Vec3) -> Self {
        Material::Metal(Metal::new(albedo))
    }

    pub fn dielectric(ref_idx: f32) -> Self {
        Material::Dielectric(Dielectric::new(ref_idx))
    }
}

impl Scatterable for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(r_in, rec),
            Material::Metal(ref inner) => inner.scatter(r_in, rec),
            Material::Dielectric(ref inner) => inner.scatter(r_in, rec),
        }
    }
}