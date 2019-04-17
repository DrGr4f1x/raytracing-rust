use super::ray::*;
use super::hitable::*;

pub struct HitableList {
    pub list: Vec<Box<Hitable>>,
}

impl HitableList {
    pub fn new() -> Self {
        HitableList { list: Vec::new() }
    }
}

impl Default for HitableList {
    fn default() -> Self {
        Self::new()
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut best_hit: Option<HitRecord> = None;

        for i in 0..self.list.len() {
            let maybe_hit = self.list[i].hit(r, t_min, closest_so_far);
            match maybe_hit {
                Some(hit) => {
                    closest_so_far = hit.t;
                    best_hit = maybe_hit;
                },
                None => (),
            }
        }

        best_hit
    }
}