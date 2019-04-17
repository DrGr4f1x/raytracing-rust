use super::vec3::*;
use super::ray::*;
use super::sampling::*;
use rand::random;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
    time0: f32,
    time1: f32,
}

impl Camera {
    pub fn look_at(pos: Vec3, target: Vec3, up: Vec3, fovy: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
        let lens_radius = 0.5 * aperture;
        let theta = fovy * std::f32::consts::PI / 180.0;
        let half_height = f32::tan(0.5 * theta);
        let half_width = aspect * half_height;
        let w = unit_vector(pos - target);
        let u = unit_vector(cross(up, w));
        let v = cross(w, u);

        Camera {    
            origin: pos,
            lower_left_corner: pos - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u,
            v,
            w,
            lens_radius,
            time0: 0.0,
            time1: 0.0,
        }
    }

    pub fn look_at_temporal(pos: Vec3, target: Vec3, up: Vec3, fovy: f32, aspect: f32, aperture: f32, focus_dist: f32, t0: f32, t1: f32) -> Self {
        let lens_radius = 0.5 * aperture;
        let theta = fovy * std::f32::consts::PI / 180.0;
        let half_height = f32::tan(0.5 * theta);
        let half_width = aspect * half_height;
        let w = unit_vector(pos - target);
        let u = unit_vector(cross(up, w));
        let v = cross(w, u);

        Camera {    
            origin: pos,
            lower_left_corner: pos - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u,
            v,
            w,
            lens_radius,
            time0: t0,
            time1: t1,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let time = self.time0 + random::<f32>() * (self.time1 - self.time0);
        Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset, time)
    }
}