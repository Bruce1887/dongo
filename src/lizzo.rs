use three_d::*;
use crate::*;

pub fn move_lizzo(croc_id: ENTITYID,
terrain_id: ENTITYID,
                      entities: &mut DongoEntityManager,
                      camera: &Camera)
     -> bool {

        let terrain = entities.get_entity_by_id(terrain_id).unwrap();

        let croc_pos = entities.get_entity_by_id(croc_id).unwrap().pos().clone();
        let height = terrain.get_height_at(croc_pos.x, croc_pos.y);
        let direction = camera.position() - croc_pos;

        let mut new_pos = croc_pos + direction.normalize() * 0.5;
        new_pos.z = height + 300.0;

        let croc = entities.get_entity_by_id_mut(croc_id).unwrap();

        let angle_to_camera = direction.y.atan2(direction.x); // Angle in radians

        // Extract the scale from the current transformation matrix
        let current_transform = croc.transform();

        // Extract scale from the diagonal of the matrix
        let scale = Vector3::new(
            current_transform.x.x.abs(), // Scale on X-axis
            current_transform.y.y.abs(), // Scale on Y-axis
            current_transform.z.z.abs(), // Scale on Z-axis
        );

        // Create a rotation matrix around the Z-axis
        let rotation_matrix = Matrix4::from_angle_z(Rad(angle_to_camera));

        // Remove scale from current transform by normalizing rotation axes
        let mut rotation_only = current_transform.clone();
        rotation_only.x /= scale.x;
        rotation_only.y /= scale.y;
        rotation_only.z /= scale.z;

        // Apply the rotation
        let rotated_transform = rotation_matrix * rotation_only;

        // Reapply the scale
        let mut new_transform = rotated_transform;
        new_transform.x *= scale.x;
        new_transform.y *= scale.y;
        new_transform.z *= scale.z;

        // Retain the original translation (position)
        new_transform.w = current_transform.w;

                

        // Update the crocodile's transformation matrix
        croc.set_transform(new_transform);

        true
    }