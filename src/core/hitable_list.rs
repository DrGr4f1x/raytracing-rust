use super::ray::*;
use super::hitable::*;
use super::aabb::*;

#[derive(Clone)]
pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
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

        for hitable in self.list.iter() {
            let maybe_hit = hitable.hit(r, t_min, closest_so_far);

            if let Some(hit) = maybe_hit {
                closest_so_far = hit.t;
                best_hit = maybe_hit;
            }
        }

        best_hit
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.list.is_empty() {
            return None
        }

        let mut temp_box: AABB;
        let maybe_box = self.list[0].bounding_box(t0, t1);
        match maybe_box {
            Some(ref inner) => temp_box = *inner,
            None => return None
        };

        for i in 1..self.list.len() {
            let maybe_box = self.list[i].bounding_box(t0, t1);
            match maybe_box {
                Some(ref inner) => temp_box = surrounding_box(inner, &temp_box),
                None => return None
            };
        }

        Some(temp_box)
    }

    fn clone_to_box(&self) -> Box<dyn Hitable> {
        Box::new(self.clone())
    }
}