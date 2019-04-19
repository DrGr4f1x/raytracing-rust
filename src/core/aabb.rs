use super::vec3::*;
use super::ray::*;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false
            }
        }
        true
    }
}

fn ffmax(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

fn ffmin(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Vec3::new(
        ffmin(box0.min().x(), box1.min().x()),
        ffmin(box0.min().y(), box1.min().y()),
        ffmin(box0.min().z(), box1.min().z())
    );

    let big = Vec3::new(
        ffmax(box0.max().x(), box1.max().x()),
        ffmax(box0.max().y(), box1.max().y()),
        ffmax(box0.max().z(), box1.max().z())
    );

    AABB::new(small, big)
}