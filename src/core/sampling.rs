use rand::random;
use super::vec3::*;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random::<f32>(), random::<f32>(), random::<f32>()) - Vec3::one();
        if p.squared_length() < 1.0 {
            return p
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random::<f32>(), random::<f32>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.squared_length() < 1.0 {
            return p
        }
    }
}