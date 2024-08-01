use three_d::*;

use crate::dongo_entity::*;
use crate::dongo_entity_manager::DongoEntityManager;
use crate::shapes::*;

const SELECTION_BOX_COLOR: Srgba = Srgba::new(255, 255, 0, 150); // a yellow color thats kinda transparent
const SELECTION_MARKER_COLOR: Srgba = Srgba::new(255, 255, 0, 170); // a yellow color thats a bit less transparent
const SELECTION_HEIGHT_EXTRA: f32 = 10.0;

pub fn get_selected(entities: &mut DongoEntityManager){

}

// caller is responsible for removing the selection box.
// This is because an end-position can only be attained if three_d::renderer::pick is succesfull.
// if mb1 is released and pick is unsuccessfull (e.g. outside of map), this function cant be invoked.
// wont be a problem if we can guarantee that all picks are successfull, i.e. that the user never sees anything outside of the map.
// as of know we cant gurantee that.
pub fn select_in_bounds(entities: &mut DongoEntityManager, start: Vec3, end: Vec3, context: &Context) {
    let inside = entities.get_all_within_bounds(start, end);
    inside.iter().for_each(|tuple| match tuple {
        (Some(id), DongoEntityType::Selectable { entity: _ }) => {
            let entity = entities.get_entity_by_id(*id).unwrap();

            let mut pos = entity.pos();

            pos.z += 10.0;

            let mut marker_trimesh = create_marker_trimesh(pos, 3.0, 3.0, SELECTION_MARKER_COLOR);
            marker_trimesh.compute_normals();
            let mut selectionmarker_gm = Gm::new(
                Mesh::new(&context, &marker_trimesh),
                ColorMaterial::new_transparent(
                    &context,
                    &CpuMaterial {
                        albedo: SELECTION_MARKER_COLOR,
                        ..Default::default()
                    },
                ),
            );

            selectionmarker_gm.set_animation(|time| Mat4::from_angle_z(radians(time * 0.005)));

            entities.add_object_from_mmp(
                Box::new(selectionmarker_gm),
                DongoEntityType::NonSelectable {
                    entity: NonSelectableEntity::SelectionMarker(*id),
                },
            );
        }
        _ => (),
    });
}

// removes the selection box, and creates a new one
pub fn resize_selection(
    entities: &mut DongoEntityManager,
    mut start: Vec3,
    mut end: Vec3,
    context: &Context,
) {
    entities.take_object(SELECTION_ID);

    match entities.get_object_by_id(SELECTION_ID) {
        Some(selection_box) => {
            let positions = create_box_positions(start, end).to_vec();
            dbg!(selection_box
                .mm_provider
                .mesh_mut()
                .update_positions(&positions))
        }
        None => {
            start.z = crate::common::MAP_MIN_HEIGHT as f32;
            end.z = crate::common::MAP_MAX_HEIGHT as f32 + SELECTION_HEIGHT_EXTRA;
            let box_trimesh = create_box_trimesh(start, end, SELECTION_BOX_COLOR);
            //CpuMesh::cube();
            let selectionbox_gm = Gm::new(
                Mesh::new(&context, &box_trimesh),
                ColorMaterial::new_transparent(
                    &context,
                    &CpuMaterial {
                        albedo: SELECTION_BOX_COLOR,
                        ..Default::default()
                    },
                ),
            );

            entities.add_object_with_id(
                SELECTION_ID,
                Box::new(selectionbox_gm),
                DongoEntityType::NonSelectable {
                    entity: NonSelectableEntity::SelectionBox,
                },
            )
        }
    }
}
