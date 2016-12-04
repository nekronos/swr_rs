
use super::vector::Vector3;
use super::vector::Vector4;

pub type Quaternion = Vector4;

impl Quaternion {
    // https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles
    pub fn from_euler_angle(euler_angle: Vector3) -> Quaternion {
        let pitch = euler_angle.x;
        let yaw = euler_angle.y;
        let roll = euler_angle.z;

        let t0 = (yaw * 0.5).cos();
        let t1 = (yaw * 0.5).sin();
        let t2 = (roll * 0.5).cos();
        let t3 = (roll * 0.5).sin();
        let t4 = (pitch * 0.5).cos();
        let t5 = (pitch * 0.5).sin();

        let mut q = Quaternion::zero();
        q.w = (t0 * t2 * t4) + (t1 * t3 * t5);
        q.x = (t0 * t3 * t4) - (t1 * t2 * t5);
        q.y = (t0 * t2 * t5) + (t1 * t3 * t4);
        q.z = (t1 * t2 * t4) - (t0 * t3 * t5);
        q
    }

    pub fn to_euler_angle(quaternion: Quaternion) -> Vector3 {
        let q = quaternion;
        let ysqr = q.y * q.y;
        let t0 = -2.0 * (ysqr + q.z * q.z) + 1.0;
        let t1 = 2.0 * (q.x * q.y - q.w * q.z);
        let t2 = -2.0 * (q.x * q.z - q.w * q.y);
        let t3 = 2.0 * (q.y * q.z - q.w * q.x);
        let t4 = -2.0 * (q.x * q.x + ysqr) + 1.0;

        let t2 = if t2 > 1.0 { 1.0 } else { t2 };
        let t2 = if t2 < -1.0 { -1.0 } else { t2 };

        let pitch = t2.asin();
        let roll = t3.atan2(t4);
        let yaw = t1.atan2(t0);

        Vector3::new(pitch, yaw, roll)
    }
}
