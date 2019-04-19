use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use std::ops::{Index, IndexMut};
use std::f32;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e:[f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { e: [x, y, z] }
    }

    pub fn zero() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn one() -> Self {
        Vec3 { e: [1.0, 1.0, 1.0] }
    }

    pub fn unit_x() -> Self {
        Vec3 { e: [1.0, 0.0, 0.0] }
    }

    pub fn unit_y() -> Self {
        Vec3 { e: [0.0, 1.0, 0.0] }
    }

    pub fn unit_z() -> Self {
        Vec3 { e: [0.0, 0.0, 1.0] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.squared_length())
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        self.e[0] /= len;
        self.e[1] /= len;
        self.e[2] /= len;
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    let len = v.length();
    Vec3 { e: [v.x() / len, v.y() / len, v.z() / len] }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self {
        Vec3 { e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()] }
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self {
        Vec3 { e: [self.x() + rhs, self.y() + rhs, self.z() + rhs] }
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [self + rhs.x(), self + rhs.y(), self + rhs.z()] }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.x();
        self.e[1] += rhs.y();
        self.e[2] += rhs.z();
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, rhs: f32) {
        self.e[0] += rhs;
        self.e[1] += rhs;
        self.e[2] += rhs;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self {
        Vec3 { e: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()] }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Self {
        Vec3 { e: [self.x() - rhs, self.y() - rhs, self.z() - rhs] }
    }
}

impl Sub<Vec3> for f32 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [self - rhs.x(), self - rhs.y(), self - rhs.z()] }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0] -= rhs.x();
        self.e[1] -= rhs.y();
        self.e[2] -= rhs.z();
    }
}

impl SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.e[0] -= rhs;
        self.e[1] -= rhs;
        self.e[2] -= rhs;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self {
        Vec3 { e: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()] }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self {
        Vec3 { e: [self.x() * rhs, self.y() * rhs, self.z() * rhs] }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [self * rhs.x(), self * rhs.y(), self * rhs.z()] }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.e[0] *= rhs.x();
        self.e[1] *= rhs.y();
        self.e[2] *= rhs.z();
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self {
        Vec3 { e: [self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z()] }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self {
        let k: f32 = 1.0 / rhs;
        Vec3 { e: [self.x() * k, self.y() * k, self.z() * k] }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 { e: [self / rhs.x(), self / rhs.y(), self / rhs.z()] }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.e[0] /= rhs.x();
        self.e[1] /= rhs.y();
        self.e[2] /= rhs.z();
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        let k: f32 = 1.0 / rhs;
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.e[index]
    }
}

pub fn dot(lhs: Vec3, rhs: Vec3) -> f32 {
    lhs.x()*rhs.x() + lhs.y()*rhs.y() + lhs.z()*rhs.z()
}

pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3 { e: [
        lhs.y()*rhs.z() - lhs.z()*rhs.y(), 
        lhs.z()*rhs.x() - lhs.x()*rhs.z(), 
        lhs.x()*rhs.y() - lhs.y()*rhs.x() ] }
}