use crate::common::*;
use three_d::*;

pub(crate) fn move_camera(camera: &mut Camera, direction: Vec3, speed: f32) {
    // Get the forward vector without modifying it
    let forward = camera.view_direction();

    // Get the right vector without modifying it
    let right = camera.right_direction();

    // Calculate the offset based on forward and right vectors
    let offset = (forward * direction.y * speed) + (right * direction.x * speed);

    // Apply the offset to the camera position, but keep Z intact
    let mut new_pos = camera.position().clone();
    new_pos.x += offset.x;
    new_pos.y += offset.y;

    // Update target based on new position and current target distance
    let target_distance = camera.target() - camera.position();
    let new_target = new_pos + target_distance;

    // Keep the up vector unchanged
    let up_clone = camera.up().clone();

    // Set the new camera view
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
            
    // rate of rotation-change
    let theta = 0.1_f32;
    let rotation_matrix = Mat3::new(
        theta.cos(), -theta.sin(), 0.0,
        theta.sin(), theta.cos(), 0.0,
        0.0, 0.0, 1.0
    );        

    // Calculate the direction-vector from the camera to the target
    let direction = pos - target;

    // Rotate the direction vector
    let rotated_direction = rotation_matrix * direction;
    
    // Update the camera's position to maintain the distance to the target
    let new_pos = camera.target() + rotated_direction;    

    // also rotate the up-direction
    let new_up = rotation_matrix * camera.up().clone();

    camera.set_view(new_pos, target, new_up);    
}
