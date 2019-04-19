use super::vec3::*;
use super::ray::*;
use super::hitable::*;
use super::aabb::*;
use rand::random;
use std::cmp::Ordering;
use std::ops::Deref;

#[derive(Clone)]
pub struct BVHNode {
    pub left: Box<dyn Hitable>,
    pub right: Box<dyn Hitable>,
    pub aabb: AABB,
}

fn box_x_compare(a: &Hitable, b: &Hitable, t0: f32, t1: f32) -> Ordering {
    let maybe_box_left = a.bounding_box(t0, t1);
    let maybe_box_right = b.bounding_box(t0, t1);

    match (maybe_box_left, maybe_box_right) {
        (Some(ref box_left), Some(ref box_right)) => if box_left.min().x() - box_right.min().x() < 0.0 { Ordering::Less } else { Ordering::Greater },
        _ => Ordering::Equal
    }
}

fn box_y_compare(a: &Hitable, b: &Hitable, t0: f32, t1: f32) -> Ordering {
    let maybe_box_left = a.bounding_box(t0, t1);
    let maybe_box_right = b.bounding_box(t0, t1);

    match (maybe_box_left, maybe_box_right) {
        (Some(ref box_left), Some(ref box_right)) => if box_left.min().y() - box_right.min().y() < 0.0 { Ordering::Less } else { Ordering::Greater },
        _ => Ordering::Equal
    }
}

fn box_z_compare(a: &Hitable, b: &Hitable, t0: f32, t1: f32) -> Ordering {
    let maybe_box_left = a.bounding_box(t0, t1);
    let maybe_box_right = b.bounding_box(t0, t1);

    match (maybe_box_left, maybe_box_right) {
        (Some(ref box_left), Some(ref box_right)) => if box_left.min().z() - box_right.min().z() < 0.0 { Ordering::Less } else { Ordering::Greater },
        _ => Ordering::Equal
    }
}

impl BVHNode {
    pub fn new(list: &mut Vec<Box<dyn Hitable>>, t0: f32, t1: f32) -> Self {
        let axis: i32 = (3.0 * random::<f32>()) as i32;
        let n = list.len();

        match axis {
            0 => list.sort_by(|a, b| box_x_compare(a.deref(), b.deref(), t0, t1)),
            1 => list.sort_by(|a, b| box_y_compare(a.deref(), b.deref(), t0, t1)),
            2 => list.sort_by(|a, b| box_z_compare(a.deref(), b.deref(), t0, t1)),
            _ => ()
        };

        let mut left: Box<dyn Hitable>;
        let mut right: Box<dyn Hitable>;

        match n {
            1 => {
                left = list[0].clone();
                right = list[0].clone();
            },
            2 => {
                left = list[0].clone();
                right = list[1].clone();
            },
            _ => {
                let (left_slice, right_slice) = list.split_at(n / 2);
                let mut list_left: Vec<Box<dyn Hitable>> = vec![];
                let mut list_right: Vec<Box<dyn Hitable>> = vec![];
                list_left.extend(left_slice.iter().map(|ref hitable| (**hitable).clone()));
                list_right.extend(right_slice.iter().map(|ref hitable| (**hitable).clone()));
                left = Box::new(BVHNode::new(&mut list_left, t0, t1));
                right = Box::new(BVHNode::new(&mut list_right, t0, t1));
            }
        };

        let maybe_box_left = left.bounding_box(t0, t1);
        let maybe_box_right = right.bounding_box(t0, t1);

        match (maybe_box_left, maybe_box_right) {
            (Some(ref left_box), Some(ref right_box)) => Self { left, right, aabb: surrounding_box(left_box, right_box) },
            _ => Self {left, right, aabb: AABB::new(Vec3::zero(), Vec3::zero()) }
        }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.aabb.hit(r, t_min, t_max) {
            let maybe_hit_left = self.left.hit(r, t_min, t_max);
            let maybe_hit_right = self.right.hit(r, t_min, t_max);

            match (maybe_hit_left, maybe_hit_right) {
                (Some(ref left_hit), Some(ref right_hit)) => return if left_hit.t < right_hit.t { maybe_hit_left } else { maybe_hit_right },
                (Some(_left_hit), None) => return maybe_hit_left,
                (None, Some(_right_hit)) => return maybe_hit_right,
                _ => return None
            };
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.aabb)
    }

    fn clone_to_box(&self) -> Box<dyn Hitable> {
        Box::new(self.clone())
    }
}