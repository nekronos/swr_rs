
use super::matrix::Matrix4;

use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x: x, y: y }
    }

    pub fn zero() -> Vector2 {
        Vector2::new(0.0, 0.0)
    }
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector4 {
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn zero() -> Vector4 {
        Vector4::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn xyz(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}


impl Div<f64> for Vector4 {
    type Output = Self;

    fn div(self, rhs: f64) -> Vector4 {
        Vector4::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn unit_x() -> Vector3 {
        Vector3::new(1.0, 0.0, 0.0)
    }

    pub fn unit_y() -> Vector3 {
        Vector3::new(0.0, 1.0, 0.0)
    }

    pub fn unit_z() -> Vector3 {
        Vector3::new(0.0, 0.0, 1.0)
    }

    pub fn dot(lhs: Vector3, rhs: Vector3) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: Vector3, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

    pub fn length_sqr(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_sqr().sqrt()
    }

    pub fn normalize(self) -> Vector3 {
        self / self.length()
    }

    pub fn transform_coordinate(coord: &Vector3, transform: &Matrix4) -> Vector3 {
        let x = Vector3::transform(coord, transform);
        x.xyz() / x.w
    }

    pub fn transform(vec: &Vector3, mat: &Matrix4) -> Vector4 {
        Vector4::new((vec.x * mat.m11) + (vec.y * mat.m21) + (vec.z * mat.m31) + mat.m41,
                     (vec.x * mat.m12) + (vec.y * mat.m22) + (vec.z * mat.m32) + mat.m42,
                     (vec.x * mat.m13) + (vec.y * mat.m23) + (vec.z * mat.m33) + mat.m43,
                     (vec.x * mat.m14) + (vec.y * mat.m24) + (vec.z * mat.m34) + mat.m44)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Vector3 {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
mod tests {

    use std::f64;
    use super::Vector3;

    #[test]
    fn add() {
        let a = Vector3::new(10.0, 20.0, 30.0);
        let b = Vector3::new(30.0, 20.0, 10.0);
        let c = a + b;
        assert_eq!(Vector3::new(40.0, 40.0, 40.0), c);
    }

    #[test]
    fn sub() {
        let a = Vector3::new(10.0, 20.0, 30.0);
        let b = Vector3::new(30.0, 20.0, 10.0);
        let c = a - b;
        assert_eq!(Vector3::new(-20.0, 0.0, 20.0), c);
    }

    #[test]
    fn mul() {
        let a = Vector3::new(10.0, 20.0, 30.0);
        let b = a * 0.5;
        assert_eq!(Vector3::new(5.0, 10.0, 15.0), b);
    }

    #[test]
    fn div() {
        let a = Vector3::new(10.0, 20.0, 30.0);
        let b = a / 2.0;
        assert_eq!(Vector3::new(5.0, 10.0, 15.0), b);
    }

    #[test]
    fn cross() {
        let a = Vector3::new(2.0, 3.0, 4.0);
        let b = Vector3::new(5.0, 6.0, 7.0);
        let c = Vector3::cross(a, b);
        assert_eq!(Vector3::new(-3.0, 6.0, -3.0), c);
    }

    #[test]
    fn dot() {
        let a = Vector3::new(9.0, 2.0, 7.0);
        let b = Vector3::new(4.0, 8.0, 10.0);
        let c = Vector3::dot(a, b);
        assert_eq!(122.0, c);
    }

    #[test]
    fn length_sqr() {
        let a = Vector3::new(2.0, 3.0, 4.0);
        let b = a.length_sqr();
        assert_eq!(29.0, b);
    }

    #[test]
    fn length() {
        let a = Vector3::new(2.0, 3.0, 4.0);
        let b = a.length();
        let b = (b * 100000.0).round() / 100000.0;
        let expected: f64 = 5.38516;
        let result = (b - expected).abs() < f64::EPSILON;
        assert!(result);
    }

    #[test]
    fn normalize() {
        let a = Vector3::new(2.0, 3.0, 4.0);
        let b = a.normalize();
        let b = b * 1000000.0;
        let b = Vector3::new(b.x.round(), b.y.round(), b.z.round());
        let b = b / 1000000.0;
        let expected = Vector3::new(0.371391, 0.557086, 0.742781);
        assert_eq!(expected, b);
    }

}
