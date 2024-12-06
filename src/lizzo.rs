use crate::*;
use three_d::*;

pub fn move_lizzo(
    croc_id: ENTITYID,
    terrain_id: ENTITYID,
    entities: &mut DongoEntityManager,
    camera: &Camera,
) -> bool {
    let terrain = entities.get_entity_by_id(terrain_id).unwrap();

    let croc_pos = entities.get_entity_by_id(croc_id).unwrap().pos().clone();
    let height = terrain.get_height_at(croc_pos.x, croc_pos.y);
    let direction = camera.position() - croc_pos;

    let mut new_pos = croc_pos + direction.normalize() * 1.2; // TODO: Extract speed from somewhere else
    new_pos.z = height + 100.0; // TODO: Extract height offset from somewhere else

    let croc = entities.get_entity_by_id_mut(croc_id).unwrap();

    let angle_to_camera = direction.y.atan2(direction.x); // Angle in radians
    let flipped_angle = (angle_to_camera + std::f32::consts::PI).rem_euclid(2.0 * std::f32::consts::PI);

    // Extract the scale from the current transformation matrix    
    
    let new_transform = Matrix4::identity()
    * Matrix4::from_translation(new_pos)
    * Matrix4::from_scale(200.0) // TODO: Extract the scale from the current transformation matrix, or some other source
    * Matrix4::from_angle_z(radians(flipped_angle));

    croc.set_transform(new_transform);

    true
}


pub fn temp(croc: &mut DongoEntity){
    print!("{}", croc.metadata().desc());
}