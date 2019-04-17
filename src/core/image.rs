use std::fs::{File};
use std::io::{BufWriter, Write};
use super::vec3::*;

pub struct Image {
    width: usize,
    height: usize,
    inv_width: f32,
    inv_height: f32,
    image_data: Vec<Vec3>,
}

impl Image {
    pub fn new(w: usize, h: usize) -> Self {
        Image {
            width: w,
            height: h,
            inv_width: 1.0 / w as f32,
            inv_height: 1.0 / h as f32,
            image_data: vec![Vec3::zero(); w * h],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn inv_width(&self) -> f32 {
        self.inv_width
    }

    pub fn inv_height(&self) -> f32 {
        self.inv_height
    }

    pub fn set_pixel(&mut self, i: usize, j: usize, color: Vec3) {
        self.image_data[i + j * self.width] = color;
    }

    pub fn save_as(&self, filename: &str) {
        let file = File::create(filename.to_string()).expect("Unable to create file");
        let mut buf_writer = BufWriter::new(file);
        write!(buf_writer, "P3\n{} {}\n255\n", self.width, self.height);

        for j in (0..self.height).rev() {
            for i in 0..self.width {
                let col = self.image_data[i + j * self.width];

                let ir : i32 = (255.99 * col.r()) as i32;
                let ig : i32 = (255.99 * col.g()) as i32;
                let ib : i32 = (255.99 * col.b()) as i32;

                writeln!(buf_writer, "{} {} {}", ir, ig, ib);
            }
        }
    }
}