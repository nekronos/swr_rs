
use std::f64;
use std::ops::{Add, Sub, Mul};
use super::vector::Vector3;
use super::quaternion::Quaternion;

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Matrix4 {
    pub m11: f64,
    pub m12: f64,
    pub m13: f64,
    pub m14: f64,
    pub m21: f64,
    pub m22: f64,
    pub m23: f64,
    pub m24: f64,
    pub m31: f64,
    pub m32: f64,
    pub m33: f64,
    pub m34: f64,
    pub m41: f64,
    pub m42: f64,
    pub m43: f64,
    pub m44: f64,
}

impl Matrix4 {
    pub fn new(m11: f64,
               m12: f64,
               m13: f64,
               m14: f64,
               m21: f64,
               m22: f64,
               m23: f64,
               m24: f64,
               m31: f64,
               m32: f64,
               m33: f64,
               m34: f64,
               m41: f64,
               m42: f64,
               m43: f64,
               m44: f64)
               -> Matrix4 {
        Matrix4 {
            m11: m11,
            m12: m12,
            m13: m13,
            m14: m14,
            m21: m21,
            m22: m22,
            m23: m23,
            m24: m24,
            m31: m31,
            m32: m32,
            m33: m33,
            m34: m34,
            m41: m41,
            m42: m42,
            m43: m43,
            m44: m44,
        }
    }

    pub fn identity() -> Matrix4 {
        Matrix4::new(1.0,
                     0.0,
                     0.0,
                     0.0,
                     0.0,
                     1.0,
                     0.0,
                     0.0,
                     0.0,
                     0.0,
                     1.0,
                     0.0,
                     0.0,
                     0.0,
                     0.0,
                     1.0)
    }


    pub fn look_at_lh(eye: Vector3, target: Vector3, up: Vector3) -> Matrix4 {
        let zaxis = (target - eye).normalize();
        let xaxis = Vector3::cross(up, zaxis).normalize();
        let yaxis = Vector3::cross(zaxis, xaxis).normalize();

        let mut result = Matrix4::identity();

        result.m11 = xaxis.x;
        result.m21 = xaxis.y;
        result.m31 = xaxis.z;

        result.m12 = yaxis.x;
        result.m22 = yaxis.y;
        result.m32 = yaxis.z;

        result.m13 = zaxis.x;
        result.m23 = zaxis.y;
        result.m33 = zaxis.z;

        result.m41 = -Vector3::dot(xaxis, eye);
        result.m41 = -Vector3::dot(yaxis, eye);
        result.m43 = -Vector3::dot(zaxis, eye);

        result
    }

    pub fn perspective_rh(fov: f64, aspect: f64, znear: f64, zfar: f64) -> Matrix4 {
        let y_half_scale = 0.5 / (fov * 0.5).tan();
        let x_half_scale = y_half_scale / aspect;
        let width = znear / x_half_scale;
        let height = znear / y_half_scale;
        let length = zfar - znear;
        let znear_doubled = znear * 2.0;

        let mut result = Matrix4::identity();

        result.m11 = znear_doubled / width;
        result.m22 = znear_doubled / height;
        result.m33 = (-zfar - znear) / length;
        result.m43 = (-znear_doubled * zfar) / length;
        result.m44 = 0.0;
        result.m34 = -1.0;

        result
    }

    pub fn translation(offset: Vector3) -> Matrix4 {
        let mut result = Matrix4::identity();
        result.m41 = offset.x;
        result.m42 = offset.y;
        result.m43 = offset.z;
        result
    }

    pub fn rotation(quat: Quaternion) -> Matrix4 {

        let n = quat.w * quat.w + quat.x * quat.x + quat.y * quat.y + quat.z * quat.z;

        let s = if n <= f64::EPSILON { 0.0 } else { 2.0 / n };
        let wx = s * quat.w * quat.x;
        let wy = s * quat.w * quat.y;
        let wz = s * quat.w * quat.z;
        let xx = s * quat.x * quat.x;
        let xy = s * quat.x * quat.y;
        let xz = s * quat.x * quat.z;
        let yy = s * quat.y * quat.y;
        let yz = s * quat.y * quat.z;
        let zz = s * quat.z * quat.z;

        Matrix4::new(1.0 - (yy + zz),
                     xy - wz,
                     xz + wy,
                     0.0,
                     xy + wz,
                     1.0 - (xx + zz),
                     yz - wx,
                     0.0,
                     xz - wy,
                     yz + wx,
                     1.0 - (xx + yy),
                     0.0,
                     0.0,
                     0.0,
                     0.0,
                     1.0)
    }
}

impl Add for Matrix4 {
    type Output = Self;

    fn add(self, rhs: Matrix4) -> Matrix4 {
        Matrix4::new(self.m11 + rhs.m11,
                     self.m12 + rhs.m12,
                     self.m13 + rhs.m13,
                     self.m14 + rhs.m14,
                     self.m21 + rhs.m21,
                     self.m22 + rhs.m22,
                     self.m23 + rhs.m23,
                     self.m24 + rhs.m24,
                     self.m31 + rhs.m31,
                     self.m32 + rhs.m32,
                     self.m33 + rhs.m33,
                     self.m34 + rhs.m34,
                     self.m41 + rhs.m41,
                     self.m42 + rhs.m42,
                     self.m43 + rhs.m43,
                     self.m44 + rhs.m44)
    }
}

impl Sub for Matrix4 {
    type Output = Self;

    fn sub(self, rhs: Matrix4) -> Matrix4 {
        Matrix4::new(self.m11 - rhs.m11,
                     self.m12 - rhs.m12,
                     self.m13 - rhs.m13,
                     self.m14 - rhs.m14,
                     self.m21 - rhs.m21,
                     self.m22 - rhs.m22,
                     self.m23 - rhs.m23,
                     self.m24 - rhs.m24,
                     self.m31 - rhs.m31,
                     self.m32 - rhs.m32,
                     self.m33 - rhs.m33,
                     self.m34 - rhs.m34,
                     self.m41 - rhs.m41,
                     self.m42 - rhs.m42,
                     self.m43 - rhs.m43,
                     self.m44 - rhs.m44)
    }
}

impl Mul<f64> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Matrix4 {
        Matrix4::new(self.m11 * rhs,
                     self.m12 * rhs,
                     self.m13 * rhs,
                     self.m14 * rhs,
                     self.m21 * rhs,
                     self.m22 * rhs,
                     self.m23 * rhs,
                     self.m24 * rhs,
                     self.m31 * rhs,
                     self.m32 * rhs,
                     self.m33 * rhs,
                     self.m34 * rhs,
                     self.m41 * rhs,
                     self.m42 * rhs,
                     self.m43 * rhs,
                     self.m44 * rhs)
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Matrix4) -> Matrix4 {
        Matrix4::new((self.m11 * rhs.m11) + (self.m12 * rhs.m21) + (self.m13 * rhs.m31) +
                     (self.m14 * rhs.m41),
                     (self.m11 * rhs.m12) + (self.m12 * rhs.m22) + (self.m13 * rhs.m32) +
                     (self.m14 * rhs.m42),
                     (self.m11 * rhs.m13) + (self.m12 * rhs.m23) + (self.m13 * rhs.m33) +
                     (self.m14 * rhs.m43),
                     (self.m11 * rhs.m14) + (self.m12 * rhs.m24) + (self.m13 * rhs.m34) +
                     (self.m14 * rhs.m44),
                     (self.m21 * rhs.m11) + (self.m22 * rhs.m21) + (self.m23 * rhs.m31) +
                     (self.m24 * rhs.m41),
                     (self.m21 * rhs.m12) + (self.m22 * rhs.m22) + (self.m23 * rhs.m32) +
                     (self.m24 * rhs.m42),
                     (self.m21 * rhs.m13) + (self.m22 * rhs.m23) + (self.m23 * rhs.m33) +
                     (self.m24 * rhs.m43),
                     (self.m21 * rhs.m14) + (self.m22 * rhs.m24) + (self.m23 * rhs.m34) +
                     (self.m24 * rhs.m44),
                     (self.m31 * rhs.m11) + (self.m32 * rhs.m21) + (self.m33 * rhs.m31) +
                     (self.m34 * rhs.m41),
                     (self.m31 * rhs.m12) + (self.m32 * rhs.m22) + (self.m33 * rhs.m32) +
                     (self.m34 * rhs.m42),
                     (self.m31 * rhs.m13) + (self.m32 * rhs.m23) + (self.m33 * rhs.m33) +
                     (self.m34 * rhs.m43),
                     (self.m31 * rhs.m14) + (self.m32 * rhs.m24) + (self.m33 * rhs.m34) +
                     (self.m34 * rhs.m44),
                     (self.m41 * rhs.m11) + (self.m42 * rhs.m21) + (self.m43 * rhs.m31) +
                     (self.m44 * rhs.m41),
                     (self.m41 * rhs.m12) + (self.m42 * rhs.m22) + (self.m43 * rhs.m32) +
                     (self.m44 * rhs.m42),
                     (self.m41 * rhs.m13) + (self.m42 * rhs.m23) + (self.m43 * rhs.m33) +
                     (self.m44 * rhs.m43),
                     (self.m41 * rhs.m14) + (self.m42 * rhs.m24) + (self.m43 * rhs.m34) +
                     (self.m44 * rhs.m44))
    }
}

#[test]
fn matrix_mul() {

    let a = Matrix4::new(1.0,
                         2.0,
                         3.0,
                         4.0,
                         5.0,
                         6.0,
                         7.0,
                         8.0,
                         9.0,
                         10.0,
                         11.0,
                         12.0,
                         13.0,
                         14.0,
                         15.0,
                         16.0);
    let b = Matrix4::new(16.0,
                         15.0,
                         14.0,
                         13.0,
                         12.0,
                         11.0,
                         10.0,
                         9.0,
                         8.0,
                         7.0,
                         6.0,
                         5.0,
                         4.0,
                         3.0,
                         2.0,
                         1.0);

    let expected = Matrix4::new(80.0,
                                70.0,
                                60.0,
                                50.0,
                                240.0,
                                214.0,
                                188.0,
                                162.0,
                                400.0,
                                358.0,
                                316.0,
                                274.0,
                                560.0,
                                502.0,
                                444.0,
                                386.0);

    let result = a * b;

    assert_eq!(expected, result);

}