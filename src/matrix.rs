use std::{fmt, ops::{Index, IndexMut, Mul}};

use crate::vector::Vec4;

pub type Float = crate::vector::Float;

#[derive(Debug, PartialEq)]
pub struct Mat4
{
    d: [Float; 16],
}

impl Default for Mat4
{
    fn default() -> Self {
        Self::new()
    }
}

impl Mat4
{
    pub fn new() -> Self {
        Mat4 { d: [0.0; 16] }
    }

    pub fn i() -> Self {
        let one = 1.0;
        let zero = 0.0;

        Mat4 {
            d: [
                one, zero, zero, zero, zero, one, zero, zero, zero, zero, one, zero, zero, zero,
                zero, one,
            ],
        }
    }

    pub fn camera(fwd: &Vec4, right: &Vec4, up: &Vec4, pos: &Vec4) -> Self {
        let zero = 0.0;
        let one = 1.0;
        Mat4 {
            d: [
                right.x, up.x, fwd.x, pos.x, right.y, up.y, fwd.y, pos.y, right.z, up.z, fwd.z,
                pos.z, zero, zero, zero, one,
            ],
        }
    }

    pub fn look(position: &Vec4, look_at: &Vec4) -> Self {
        let direction = (look_at - position).normalized();
        let temp_up = Vec4::direction(0.0, 1.0, 0.0);
        let right = temp_up.cross_product(&direction).normalized();
        let up = direction.cross_product(&right).normalized();

        Mat4::camera(&direction, &right, &up, position)
    }

    pub fn translation(translation: &Vec4) -> Self {
        let mut t: Mat4 = Mat4::i();
        t[(3, 0)] = translation.x;
        t[(3, 1)] = translation.y;
        t[(3, 2)] = translation.z;

        t
    }

    pub fn scale(factor: &Vec4) -> Self {
        let mut s: Mat4 = Mat4::i();
        s[(0, 0)] = factor.x;
        s[(1, 1)] = factor.y;
        s[(2, 2)] = factor.z;

        s
    }

    pub fn inverse(&self) -> Self {
        let mut inv = Mat4::new();

        inv[0] = self[5] * self[10] * self[15]
            - self[5] * self[11] * self[14]
            - self[9] * self[6] * self[15]
            + self[9] * self[7] * self[14]
            + self[13] * self[6] * self[11]
            - self[13] * self[7] * self[10];

        inv[4] = -self[4] * self[10] * self[15]
            + self[4] * self[11] * self[14]
            + self[8] * self[6] * self[15]
            - self[8] * self[7] * self[14]
            - self[12] * self[6] * self[11]
            + self[12] * self[7] * self[10];

        inv[8] = self[4] * self[9] * self[15]
            - self[4] * self[11] * self[13]
            - self[8] * self[5] * self[15]
            + self[8] * self[7] * self[13]
            + self[12] * self[5] * self[11]
            - self[12] * self[7] * self[9];

        inv[12] = -self[4] * self[9] * self[14]
            + self[4] * self[10] * self[13]
            + self[8] * self[5] * self[14]
            - self[8] * self[6] * self[13]
            - self[12] * self[5] * self[10]
            + self[12] * self[6] * self[9];

        inv[1] = -self[1] * self[10] * self[15]
            + self[1] * self[11] * self[14]
            + self[9] * self[2] * self[15]
            - self[9] * self[3] * self[14]
            - self[13] * self[2] * self[11]
            + self[13] * self[3] * self[10];

        inv[5] = self[0] * self[10] * self[15]
            - self[0] * self[11] * self[14]
            - self[8] * self[2] * self[15]
            + self[8] * self[3] * self[14]
            + self[12] * self[2] * self[11]
            - self[12] * self[3] * self[10];

        inv[9] = -self[0] * self[9] * self[15]
            + self[0] * self[11] * self[13]
            + self[8] * self[1] * self[15]
            - self[8] * self[3] * self[13]
            - self[12] * self[1] * self[11]
            + self[12] * self[3] * self[9];

        inv[13] = self[0] * self[9] * self[14]
            - self[0] * self[10] * self[13]
            - self[8] * self[1] * self[14]
            + self[8] * self[2] * self[13]
            + self[12] * self[1] * self[10]
            - self[12] * self[2] * self[9];

        inv[2] = self[1] * self[6] * self[15]
            - self[1] * self[7] * self[14]
            - self[5] * self[2] * self[15]
            + self[5] * self[3] * self[14]
            + self[13] * self[2] * self[7]
            - self[13] * self[3] * self[6];

        inv[6] = -self[0] * self[6] * self[15]
            + self[0] * self[7] * self[14]
            + self[4] * self[2] * self[15]
            - self[4] * self[3] * self[14]
            - self[12] * self[2] * self[7]
            + self[12] * self[3] * self[6];

        inv[10] = self[0] * self[5] * self[15]
            - self[0] * self[7] * self[13]
            - self[4] * self[1] * self[15]
            + self[4] * self[3] * self[13]
            + self[12] * self[1] * self[7]
            - self[12] * self[3] * self[5];

        inv[14] = -self[0] * self[5] * self[14]
            + self[0] * self[6] * self[13]
            + self[4] * self[1] * self[14]
            - self[4] * self[2] * self[13]
            - self[12] * self[1] * self[6]
            + self[12] * self[2] * self[5];

        inv[3] = -self[1] * self[6] * self[11]
            + self[1] * self[7] * self[10]
            + self[5] * self[2] * self[11]
            - self[5] * self[3] * self[10]
            - self[9] * self[2] * self[7]
            + self[9] * self[3] * self[6];

        inv[7] = self[0] * self[6] * self[11]
            - self[0] * self[7] * self[10]
            - self[4] * self[2] * self[11]
            + self[4] * self[3] * self[10]
            + self[8] * self[2] * self[7]
            - self[8] * self[3] * self[6];

        inv[11] = -self[0] * self[5] * self[11]
            + self[0] * self[7] * self[9]
            + self[4] * self[1] * self[11]
            - self[4] * self[3] * self[9]
            - self[8] * self[1] * self[7]
            + self[8] * self[3] * self[5];

        inv[15] = self[0] * self[5] * self[10]
            - self[0] * self[6] * self[9]
            - self[4] * self[1] * self[10]
            + self[4] * self[2] * self[9]
            + self[8] * self[1] * self[6]
            - self[8] * self[2] * self[5];

        let mut det = self[0] * inv[0] + self[1] * inv[4] + self[2] * inv[8] + self[3] * inv[12];

        if det == 0.0 {
            return Mat4::i();
        }

        det = 1.0 / det;

        for i in 0..16 {
            inv[i] *= det;
        }

        inv
    }
}

impl fmt::Display for Mat4
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..4 {
            f.write_str("[ ")?;

            for col in 0..4 {
                f.write_fmt(format_args!("{:.3} ", self[(col, row)]))?;
            }

            f.write_str("]")?;
        }
        Ok(())
    }
}

impl Index<usize> for Mat4
{
    type Output = Float;

    fn index(&self, index: usize) -> &Float {
        &self.d[index]
    }
}

impl IndexMut<usize> for Mat4
{
    fn index_mut(&mut self, index: usize) -> &mut Float {
        &mut self.d[index]
    }
}

impl Index<(usize, usize)> for Mat4
{
    type Output = Float;

    fn index(&self, index: (usize, usize)) -> &Float {
        let (x, y) = index;
        if x > 3 || y > 3 {
            core::panic!("Matrix index out of bounds");
        }
        &self.d[x + y * 4]
    }
}

impl IndexMut<(usize, usize)> for Mat4
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Float {
        let (x, y) = index;
        if x > 3 || y > 3 {
            core::panic!("Matrix index out of bounds");
        }
        &mut self.d[x + y * 4]
    }
}

impl Mul for &Mat4
{
    type Output = Mat4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Mat4::new();
        for row in 0..4 {
            for col in 0..4 {
                result[(row, col)] = self[(row, 0)] * rhs[(0, col)]
                    + self[(row, 1)] * rhs[(1, col)]
                    + self[(row, 2)] * rhs[(2, col)]
                    + self[(row, 3)] * rhs[(3, col)];
            }
        }
        result
    }
}

impl Mul<&Vec4> for &Mat4
{
    type Output = Vec4;

    fn mul(self, rhs: &Vec4) -> Self::Output {
        let mut r = Vec4::position(0.0, 0.0, 0.0);

        for row in 0..4 {
            r[row] = self[(0, row)] * rhs[0]
                + self[(1, row)] * rhs[1]
                + self[(2, row)] * rhs[2]
                + self[(3, row)] * rhs[3];
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let _m = Mat4::i();
    }

    #[test]
    fn access_ro() {
        let m = Mat4::i();
        assert_eq!(1.0, m[(0, 0)]);
        assert_eq!(1.0, m[(1, 1)]);
        assert_eq!(1.0, m[(2, 2)]);
        assert_eq!(1.0, m[(3, 3)]);
    }

    #[test]
    #[should_panic]
    fn access_ro_oob() {
        let m = Mat4::i();
        let _f = m[(5, 5)];
    }

    #[test]
    fn access_rw() {
        let mut m = Mat4::i();
        m[(0, 0)] = 2.0;
        m[(1, 1)] = 2.0;
        m[(2, 2)] = 2.0;
        m[(3, 3)] = 2.0;
        m[(0, 0)] = 2.0;
    }

    #[test]
    #[should_panic]
    fn access_rw_oob() {
        let mut m = Mat4::i();
        m[(5, 5)] = 2.0;
    }

    #[test]
    fn multiply_identity() {
        let i: Mat4 = Mat4::i();

        let result = &i * &i;
        assert_eq!(i, result);
    }

    #[test]
    fn look_at() {
        let pos = Vec4::position(0.0, 0.0, -10.0);
        let origin = Vec4::position(0.0, 0.0, 0.0);
        let camera = Mat4::look(&pos, &origin);

        println!("{}", camera)
    }
}
