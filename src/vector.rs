use std::ops::{Add, Index, IndexMut, Sub};

pub type Float = f64;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec4
{
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub w: Float,
}

impl Vec4
{
    pub fn position(x: Float, y: Float, z: Float) -> Vec4 {
        Vec4 {
            x,
            y,
            z,
            w: 1.0,
        }
    }

    pub fn direction(x: Float, y: Float, z: Float) -> Vec4 {
        Vec4 {
            x,
            y,
            z,
            w: 0.0,
        }
    }
    
    pub fn as_direction(&self) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 0.0,
        }
    }

    pub fn dot_product(&self, other: &Vec4) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(&self, other: &Vec4) -> Vec4 {
        Vec4 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
            w: 0.0,
        }
    }

    pub fn scale(&self, other: Float) -> Vec4 {
        Vec4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w,
        }
    }

    pub fn mag(&self) -> Float {
        Float::sqrt(self.dot_product(self))
    }

    pub fn normalized(&self) -> Vec4 {
        let mag = self.mag();
        Vec4 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w,
        }
    }

    pub fn reverse(&self) -> Vec4 {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}

impl Add for &Vec4
{
    type Output = Vec4;

    fn add(self, other: &Vec4) -> Vec4 {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w,
        }
    }
}

impl Sub for &Vec4
{
    type Output = Vec4;

    fn sub(self, other: &Vec4) -> Vec4 {
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: 0.0,
        }
    }
}

impl Index<usize> for Vec4
{
    type Output = Float;

    fn index(&self, index: usize) -> &Float {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => std::panic!("Vector index must be 0..3"),
        }
    }
}

impl IndexMut<usize> for Vec4
{
    fn index_mut(&mut self, index: usize) -> &mut Float {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => std::panic!("Vector index must be 0..3"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let _ = Vec4::position(1.0, 1.0, 1.0);
        let _ = Vec4::direction(1.0, 1.0, 1.0);
    }

    #[test]
    fn add_position() {
        let u = Vec4::position(1.0, 2.0, 3.0);
        let v = Vec4::position(-1.0, -2.0, -3.0);
        assert_eq!(Vec4::position(2.0, 4.0, 6.0), &u + &u);
        assert_eq!(Vec4::position(0.0, 0.0, 0.0), &u + &v);
    }

    #[test]
    fn add_direction() {
        let u = Vec4::direction(1.0, 2.0, 3.0);
        let v = Vec4::direction(-1.0, -2.0, -3.0);
        assert_eq!(Vec4::direction(2.0, 4.0, 6.0), &u + &u);
        assert_eq!(Vec4::direction(0.0, 0.0, 0.0), &u + &v);
    }

    #[test]
    fn add_direction_to_position() {
        let u = Vec4::position(1.0, 2.0, 3.0);
        let v = Vec4::direction(-1.0, -2.0, -3.0);
        assert_eq!(Vec4::position(2.0, 4.0, 6.0), &u + &u);
        assert_eq!(Vec4::position(0.0, 0.0, 0.0), &u + &v);
    }

    #[test]
    fn sub() {
        let u = Vec4::position(1.0, 2.0, 3.0);
        let v = Vec4::position(-1.0, -2.0, -3.0);
        assert_eq!(Vec4::direction(2.0, 4.0, 6.0), &u - &v);
        assert_eq!(Vec4::direction(0.0, 0.0, 0.0), &u - &u);
    }

    #[test]
    fn mul() {
        let u = Vec4::position(1.0, 2.0, 3.0);
        let v = Vec4::position(-1.0, -2.0, -3.0);
        assert_eq!(Vec4::position(2.0, 4.0, 6.0), u.scale(2.0));
        assert_eq!(Vec4::position(-3.0, -6.0, -9.0), v.scale(3.0));
    }

    #[test]
    fn dot_product() {
        let u = Vec4::direction(1.0, 2.0, 3.0);
        let v = Vec4::direction(-1.0, -2.0, -3.0);

        assert_eq!(-14.0, u.dot_product(&v));
        assert_eq!(-14.0, v.dot_product(&u));
        assert_eq!(u.dot_product(&v), v.dot_product(&u));
    }

    #[test]
    fn cross_product_unitvecs() {
        let u = Vec4::direction(1.0, 0.0, 0.0);
        let v = Vec4::direction(0.0, 1.0, 0.0);
        let w = Vec4::direction(0.0, 0.0, 1.0);

        assert_eq!(w, u.cross_product(&v));
        assert_eq!(u, v.cross_product(&w));
        assert_eq!(v, w.cross_product(&u));
    }

    #[test]
    fn mag() {
        // Probably shouldn't use direct equality for floats here

        assert_eq!(1.0, Vec4::direction(1.0, 0.0, 0.0).mag());
        assert_eq!(Float::sqrt(2.0), Vec4::direction(1.0, 1.0, 0.0).mag());
        assert_eq!(Float::sqrt(12.0), Vec4::direction(2.0, 2.0, 2.0).mag());
    }

    #[test]
    fn norm() {
        let u = Vec4::direction(1.0, 0.0, 0.0);
        assert_eq!(u, u.normalized());
        assert_eq!(u, (u.scale(2.0)).normalized());
    }

    #[test]
    fn combinatoric() {
        let u = Vec4::direction(2.0, 3.0, 0.0);
        let v = Vec4::direction(-3.0, 2.0, 0.0);
        let w = u.cross_product(&v).normalized();
        assert_eq!(Vec4::direction(0.0, 0.0, 1.0), w);
    }

    #[test]
    fn index_ro() {
        let u = Vec4::position(1.0, 2.0, 3.0);
        assert_eq!(1.0, u[0]);
        assert_eq!(2.0, u[1]);
        assert_eq!(3.0, u[2]);
    }

    #[test]
    fn index_rw() {
        let mut u = Vec4::position(1.0, 2.0, 3.0);
        u[0] = u[1];
        assert_eq!(u[0], u[1]);
    }
}
