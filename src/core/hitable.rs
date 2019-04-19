use super::vec3::*;
use super::ray::*;
use super::material::*;
use super::aabb::*;

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

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
    fn clone_to_box(&self) -> Box<dyn Hitable>;
}

impl Clone for Box<dyn Hitable> {
    fn clone(&self) -> Box<dyn Hitable> {
        self.clone_to_box()
    }
}