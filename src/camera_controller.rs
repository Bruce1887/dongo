use crate::common::*;
use three_d::*;

pub(crate) fn move_camera(camera: &mut Camera, direction: Vec3, speed: f32) {
    let pos_clone = camera.position().clone();

    let new_pos = pos_clone + direction * speed;

    let mut new_target = new_pos + CAM_START_TARGET;
    new_target.z = 0.0;

    let up_clone = camera.up().clone();

    camera.set_view(new_pos, new_target, up_clone);
}

pub(crate) fn zoom_camera(camera: &mut Camera, delta: &(f32, f32)) {
    let mut pos_clone = camera.position().clone();
    let target_clone = camera.target().clone();
    let up_clone = camera.up().clone();

    pos_clone.z -= delta.1; // delta.1 is positive when scrolling "up" (zooming in)
    pos_clone.z = pos_clone.z.clamp(CAMERA_MIN_HEIGHT, CAMERA_MAX_HEIGHT);

    camera.set_view(pos_clone, target_clone, up_clone);
}

pub(crate) fn rotate_camera(camera: &mut Camera) {
    let pos = camera.position().clone();
    let target = camera.target().clone();
    let up = camera.up().clone();
    let distance = Vector3::distance(pos, target);

    let theta = (pos.y - target.y).atan2(pos.x - target.x);
    let delta = 0.001;
    let delta_theta = theta + delta;

    let new_pos = Vec3::new(
        target.x + distance * delta_theta.cos(),
        target.y + distance * delta_theta.sin(),
        pos.z,
    );

    camera.set_view(new_pos, target, up);

    /*
    let cos_val = 0.0;
    let sin_val = 1.0;
    let rotation_matrix = Mat3::new(
        cos_val, -sin_val, 0.0,
        sin_val, cos_val, 0.0,
        0.0, 0.0, 1.0
    );

    // Calculate the direction from the camera to the target
    let direction = _camera.position() - _camera.target();

    // Rotate the direction vector
    let rotated_direction = rotation_matrix * direction;

    // Update the camera's position to maintain the distance to the target
    let new_pos = _camera.target() + rotated_direction;

    let target = _camera.target().clone();
    let up = _camera.up().clone();
    _camera.set_view(new_pos, target, up);
    return;
    */
}
