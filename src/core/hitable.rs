use super::vec3::*;
use super::ray::*;
use super::material::*;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord { 
            t: 0.0, 
            p: Vec3::new(0.0, 0.0, 0.0), 
            normal: Vec3::new(0.0, 0.0, 1.0), 
            material: Material::Lambertian(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))) }
    }
}

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}