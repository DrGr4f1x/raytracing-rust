use super::vec3::*;
use super::ray::*;
use super::hitable::*;
use super::material::*;
use super::aabb::*;
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
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = dot(r.direction(), r.direction());
        let b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - f32::sqrt(b * b - a * c)) / a;
            if (temp < t_max) && (temp > t_min) {

                let pos = r.point_at_parameter(temp);
                let rec = HitRecord {
                    t: temp,
                    p: pos,
                    normal: (pos - self.center) / self.radius,
                    material: self.material };

                return Some(rec)
            }
            let temp = (-b + f32::sqrt(b * b - a * c)) / a;
            if (temp < t_max) && (temp > t_min) {

                let pos = r.point_at_parameter(temp);
                let rec = HitRecord {
                    t: temp,
                    p: pos,
                    normal: (pos - self.center) / self.radius,
                    material: self.material };

                return Some(rec)
            }
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let aabb = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(aabb)
    }

    fn clone_to_box(&self) -> Box<dyn Hitable> {
        Box::new(*self)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MovableSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub radius: f32,
    pub time0: f32,
    pub time1: f32,
    pub material: Material,
}

impl MovableSphere {
    pub fn new(cen0: Vec3, cen1: Vec3, r: f32, t0: f32, t1: f32, mat: Material) -> Self {
        MovableSphere { 
            center0: cen0,
            center1: cen1,
            radius: r, 
            time0: t0,
            time1: t1,
            material: mat }
    }

    pub fn center(&self, t: f32) -> Vec3 {
        self.center0 + ((t - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovableSphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = dot(r.direction(), r.direction());
        let b = dot(oc, r.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - f32::sqrt(b * b - a * c)) / a;
            if (temp < t_max) && (temp > t_min) {

                let pos = r.point_at_parameter(temp);
                let rec = HitRecord {
                    t: temp,
                    p: pos,
                    normal: (pos - self.center(r.time())) / self.radius,
                    material: self.material };

                return Some(rec)
            }
            let temp = (-b + f32::sqrt(b * b - a * c)) / a;
            if (temp < t_max) && (temp > t_min) {

                let pos = r.point_at_parameter(temp);
                let rec = HitRecord {
                    t: temp,
                    p: pos,
                    normal: (pos - self.center(r.time())) / self.radius,
                    material: self.material };

                return Some(rec)
            }
        }

        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let aabb0 = AABB::new(
            self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        );

        let aabb1 = AABB::new(
            self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(surrounding_box(&aabb0, &aabb1))
    }

    fn clone_to_box(&self) -> Box<dyn Hitable> {
        Box::new(*self)
    }
}