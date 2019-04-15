use super::vec3::*;
use super::ray::*;
use super::hitable::*;
use super::sampling::*;

pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: Vec3) -> Self {
        Lambertian { albedo: a }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        true
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(a: Vec3) -> Self {
        Metal { albedo: a }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut unit_vector = r_in.direction();
        unit_vector.normalize();
        let reflected = reflect(unit_vector, rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        
        dot(scattered.direction(), rec.normal) > 0.0
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Scatterable for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(r_in, rec, attenuation, scattered),
            Material::Metal(ref inner) => inner.scatter(r_in, rec, attenuation, scattered),
        }
    }
}