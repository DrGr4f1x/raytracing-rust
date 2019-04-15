use super::vec3::*;
use super::ray::*;
use super::hitable::*;
use super::material::*;
use std::f32;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f32, mat: Material) -> Self {
        Sphere { center: cen, radius: r, material: mat }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - f32::sqrt(b * b - a * c)) / a;
            if (temp < t_max) && (temp > t_min) {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material;

                return true
            }
            let temp = (-b + f32::sqrt(b * b - a * c)) / a;
            if (temp < t_max) && (temp > t_min) {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material;

                return true
            }
        }

        return false
    }
}