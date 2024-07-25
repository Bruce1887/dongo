use crate::common::*;
use three_d::*;

pub(crate) fn move_camera(camera: &mut Camera, direction: Vec3, speed: f32) {
    // Get the forward and right vector. throw away the Z component and normalize
    let mut forward = camera.view_direction();
    forward.z = 0.0;
    forward = forward.normalize();
    let mut right = camera.right_direction();
    right.z = 0.0;
    right = right.normalize();

    // Calculate the new position based on the direction and speed
    let mut new_pos = camera.position().clone();
    new_pos += right * direction.x * speed;
    new_pos += forward * direction.y * speed;

    // Update target based on new position and current target distance
    let target_distance = camera.target() - camera.position();
    let new_target = new_pos + target_distance;

    // Keep the up vector unchanged
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

pub(crate) fn rotate_camera(camera: &mut Camera, rotation_direction: f32) {
    assert!(rotation_direction == -1.0 || rotation_direction == 1.0);
    let pos = camera.position().clone();
    let target = camera.target().clone();

    // rate of rotation-change
    let rotation = CAMERA_ROTATE_SPEED * rotation_direction;
    let rotation_matrix = Mat3::new(
        rotation.cos(),
        -rotation.sin(),
        0.0,
        rotation.sin(),
        rotation.cos(),
        0.0,
        0.0,
        0.0,
        1.0,
    );

    // Calculate the direction-vector from the camera to the target
    let cam_to_target = pos - target;

    // Rotate the direction vector
    let rotated_cam_to_target = rotation_matrix * cam_to_target;

    // Update the camera's position to maintain the distance to the target
    let new_pos = camera.target() + rotated_cam_to_target;

    // also rotate the up-direction
    let new_up = rotation_matrix * camera.up().clone();

    camera.set_view(new_pos, target, new_up);
}
