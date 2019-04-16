use super::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub a: Vec3,
    pub b: Vec3,
    pub t: f32,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3, t: f32) -> Self {
        Ray { a: a, b: b, t: t }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + t * self.b
    }

    pub fn time(&self) -> f32 {
        self.t
    }
}