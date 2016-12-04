
use super::vector::Vector3;
use super::vector::Vector4;

pub type Quaternion = Vector4;

impl Quaternion {
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
}
