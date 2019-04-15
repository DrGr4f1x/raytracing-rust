use super::vec3::*;
use super::ray::*;
use super::hitable::*;
use super::sampling::*;
use rand::random;

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

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let mut uv = v;
    uv.normalize();
    let dt = dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - n * dt) - n * f32::sqrt(discriminant);
        return true
    }
    else {
        return false
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
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(a: f32) -> Self {
        Dielectric{ ref_idx: a }
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let outward_normal: Vec3;
        let reflected = reflect(r_in.direction(), rec.normal);
        let ni_over_nt: f32;
        *attenuation = Vec3::one();
        let reflect_prob: f32;
        let cosine: f32;
        if dot(r_in.direction(), rec.normal) > 0.0 {
            outward_normal = -1.0 * rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * dot(r_in.direction(), rec.normal) / r_in.direction().length();
        }
        else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -dot(r_in.direction(), rec.normal) / r_in.direction().length();
        }

        let mut refracted = Vec3::zero();
        if refract(r_in.direction(), outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = schlick(cosine, self.ref_idx);
        }
        else {
            *scattered = Ray::new(rec.p, reflected);
            reflect_prob = 1.0;
        }

        if random::<f32>() < reflect_prob {
            *scattered = Ray::new(rec.p, reflected);
        }
        else {
            *scattered = Ray::new(rec.p, refracted);
        }

        return true;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn as_lambertian(a: Vec3) -> Self {
        Material::Lambertian(Lambertian::new(a))
    }

    pub fn as_metal(a: Vec3) -> Self {
        Material::Metal(Metal::new(a))
    }

    pub fn as_dielectric(a: f32) -> Self {
        Material::Dielectric(Dielectric::new(a))
    }
}

impl Scatterable for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(r_in, rec, attenuation, scattered),
            Material::Metal(ref inner) => inner.scatter(r_in, rec, attenuation, scattered),
            Material::Dielectric(ref inner) => inner.scatter(r_in, rec, attenuation, scattered),
        }
    }
}