use consts::{ DEG2RAD };
use {Vector3, Quaternion, ApproxEq};

const RIGHT_QUAT: Quaternion = Quaternion{ x: 0.0, y: 0.7071068, z: 0.0, w: 0.7071068 };
const LEFT_QUAT: Quaternion = Quaternion{ x: 0.0, y: -0.7071068, z: 0.0, w: 0.7071068 };

#[test]
fn constants() {
    assert_eq!(Quaternion::IDENTITY, Quaternion::new(0.0, 0.0, 0.0, 1.0));
}

#[test]
fn construct() {
    assert_eq!(Quaternion::new(0.0, 0.0, 0.0, 1.0), Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 });
}

#[test]
fn from_direction() {
    let q = Quaternion::from_direction(Vector3::RIGHT);

    assert_approx_eq!(q, RIGHT_QUAT);
}

#[test]
fn from_orientation() {
    let q = Quaternion::from_orientation(Vector3::FORWARD, Vector3::RIGHT);

    assert_approx_eq!(q.up(), Vector3::RIGHT);
}

#[test]
fn from_euler() {
    let q = Quaternion::from_euler(Vector3::new(0.0, 90.0 * DEG2RAD, 0.0));

    assert_approx_eq!(q, RIGHT_QUAT);
}

#[test]
fn from_euler_components() {
    let q = Quaternion::from_euler_components(0.0, 90.0 * DEG2RAD, 0.0);

    assert_approx_eq!(q, RIGHT_QUAT);
}

#[test]
fn from_angle_axis() {
    let q = Quaternion::from_angle_axis(90.0 * DEG2RAD, Vector3::UP);

    assert_approx_eq!(q, RIGHT_QUAT);
}

#[test]
fn forward() {
    let q = RIGHT_QUAT;

    assert_approx_eq!(q.forward(), Vector3::RIGHT);
}

#[test]
fn right() {
    let q = RIGHT_QUAT;

    assert_approx_eq!(q.right(), -Vector3::FORWARD);
}

#[test]
fn up() {
    let q = RIGHT_QUAT;

    assert_approx_eq!(q.up(), Vector3::UP);
}

#[test]
fn to_euler() {
    let euler = RIGHT_QUAT.to_euler();

    assert_approx_eq!(euler, Vector3::new(0.0, 90.0 * DEG2RAD, 0.0));
}

#[test]
fn to_angle_axis() {
    let mut angle = 0.0;
    let mut axis = Vector3::ZERO;

    RIGHT_QUAT.to_angle_axis(&mut angle, &mut axis);

    assert_approx_eq!(angle, 90.0 * DEG2RAD);
    assert_approx_eq!(axis, Vector3::UP);
}

#[test]
fn dot() {
    let dot = Quaternion::dot(RIGHT_QUAT, RIGHT_QUAT);
    assert_approx_eq!(dot, 1.0);

    let dot2 = Quaternion::dot(RIGHT_QUAT, LEFT_QUAT);
    assert_approx_eq!(dot2, 0.0);
}

#[test]
fn scale() {
    let q = Quaternion::new(0.0, 0.0, 0.0, 1.0);
    let q_scaled = Quaternion::scale(q, 2.0);

    assert_approx_eq!(q_scaled, Quaternion::new(0.0, 0.0, 0.0, 2.0));
}

#[test]
fn inverse() {
    let q = RIGHT_QUAT.inverse();
    let v = q * Vector3::FORWARD;

    assert_approx_eq!(v, -Vector3::RIGHT);
}

#[test]
fn conjugate() {
    let qa = RIGHT_QUAT;
    let qb = LEFT_QUAT;

    assert_approx_eq!(qa.conjugate() * qb.conjugate(), (qa * qb).conjugate());
}

#[test]
fn magnitude() {
    let q = RIGHT_QUAT;

    assert_approx_eq!(q.magnitude(), (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt());
}

#[test]
fn sqr_magnitude() {
    let q = RIGHT_QUAT;

    assert_approx_eq!(q.sqr_magnitude(), q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w);
}

#[test]
fn normalized() {
    let q_norm = RIGHT_QUAT.normalized();

    assert_approx_eq!(q_norm.magnitude(), 1.0);
}

#[test]
fn normalize() {
    let mut q_norm = RIGHT_QUAT;
    q_norm.normalize();

    assert_approx_eq!(q_norm.magnitude(), 1.0);
}

#[test]
fn lerp() {
    let q0 = LEFT_QUAT;
    let q2 = RIGHT_QUAT;

    assert_approx_eq!(q0, Quaternion::lerp(q0, q2, 0.0).normalized());
    assert_approx_eq!(q2, Quaternion::lerp(q0, q2, 1.0).normalized());
}

#[test]
fn lerp_unclamped() {
    let q0 = LEFT_QUAT;
    let q2 = RIGHT_QUAT;

    assert_approx_eq!(q0, Quaternion::lerp(q0, q2, 0.0).normalized());
    assert_approx_eq!(q2, Quaternion::lerp(q0, q2, 1.0).normalized());
}

#[test]
fn slerp() {
    let q0 = LEFT_QUAT;
    let q2 = RIGHT_QUAT;

    assert_approx_eq!(q0, Quaternion::slerp(q0, q2, 0.0).normalized());
    assert_approx_eq!(q2, Quaternion::slerp(q0, q2, 1.0).normalized());
}

#[test]
fn slerp_unclamped() {
    let q0 = LEFT_QUAT;
    let q2 = RIGHT_QUAT;

    assert_approx_eq!(q0, Quaternion::slerp(q0, q2, 0.0).normalized());
    assert_approx_eq!(q2, Quaternion::slerp(q0, q2, 1.0).normalized());
}

#[test]
fn add_quaternion() {
    let q = RIGHT_QUAT + RIGHT_QUAT;

    assert_approx_eq!(q, Quaternion::new(0.0, 1.4142136, 0.0, 1.4142136));
}

#[test]
fn sub_quaternion() {
    let q = RIGHT_QUAT - RIGHT_QUAT;

    assert_approx_eq!(q.x, 0.0);
    assert_approx_eq!(q.y, 0.0);
    assert_approx_eq!(q.z, 0.0);
    assert_approx_eq!(q.w, 0.0);
}

#[test]
fn mul_quaternion() {
    let q = RIGHT_QUAT;

    assert_approx_eq!(q * q, Quaternion::from_euler_components(0.0, 180.0 * DEG2RAD, 0.0));
}

#[test]
fn mul_quaternion_vector() {
    let v = Vector3::FORWARD;
    let q = Quaternion::from_euler_components(0.0, 90.0 * DEG2RAD, 0.0);

    let v_rot = q * v;

    assert_approx_eq!(v_rot, Vector3::RIGHT);
}

#[test]
fn mul_quaternion_scalar() {
    let q = Quaternion::new(0.0, 0.0, 0.0, 1.0);

    let q_scaled = q * 2.0;

    assert_approx_eq!(q_scaled, Quaternion::new(0.0, 0.0, 0.0, 2.0));
}