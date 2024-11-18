use three_d::*;

use crate::dongo_entity::*;
use crate::dongo_entity_manager::DongoEntityManager;
use crate::shapes::*;

const SELECTION_BOX_COLOR: Srgba = Srgba::new(255, 255, 0, 150); // a yellow color thats kinda transparente
const SELECTION_MARKER_COLOR: Srgba = Srgba::new(255, 255, 0, 170); // a yellow color thats a bit less transparent
const SELECTION_BOX_HEIGHT_EXTRA: f32 = 10.0;
const SELECTION_MARKER_HEIGHT_EXTRA: f32 = 20.0;

pub struct DongoSelector {
    selected: Vec<ENTITYID>,
    markers: Vec<ENTITYID>,
    selection_box: Option<ENTITYID>,
}

impl Default for DongoSelector {
    fn default() -> Self {
        DongoSelector {
            selected: Vec::new(),
            markers: Vec::new(),
            selection_box: None,
        }
    }
}

impl DongoSelector {
    pub fn get_selected(&self) -> &Vec<ENTITYID> {
        &self.selected
    }
    fn add_markers_to_render(&mut self, entities: &mut DongoEntityManager, context: &Context) {
        self.selected.iter().for_each(|id| {
            let entity = entities.get_entity_by_id(*id).unwrap();
            let mut pos = entity.pos(); 

            pos.z += SELECTION_MARKER_HEIGHT_EXTRA;
            
            let marker_trimesh = create_marker_trimesh(3.0, 3.0, SELECTION_MARKER_COLOR);
            // marker_trimesh.compute_normals();
            let mut marker_gm = Gm::new(
                Mesh::new(&context, &marker_trimesh),
                ColorMaterial::new_transparent(
                    &context,
                    &CpuMaterial {
                        albedo: SELECTION_MARKER_COLOR,
                        ..Default::default()
                    },
                ),
            );

            marker_gm.set_animation(|time| Mat4::from_angle_z(radians(time * 0.005)));

            let mut marker_entity = DongoEntity::from_gm(marker_gm, DongoMetadata::new(Some("Selectionmarker"), vec![TAG_HAS_ANIMATION,TAG_NO_LIGHT,TAG_SELECTION_MARKER]));
            marker_entity.set_pos(pos);

            let marker_id = entities.add_entity(marker_entity);
            //println!("added marker with id: {} and target {}", marker_id, id);
            self.markers.push(marker_id);
        });
    }

    pub fn remove_selection_box(&mut self, entities: &mut DongoEntityManager) {
        match self.selection_box {
            Some(id) => {
                entities.take_entity_by_id(id);
            }
            None => (),
        }
        self.selection_box = None;
    }

    fn remove_markers(&mut self, entities: &mut DongoEntityManager) {
        self.markers.iter().for_each(|id| {            
            entities.take_entity_by_id(*id);
        });
        self.markers.clear();
    }

    pub fn clear_selection(&mut self, entities: &mut DongoEntityManager) {
        self.remove_selection_box(entities);
        self.remove_markers(entities);
        self.selected.clear();
    }

    // caller is responsible for removing the selection box.
    // This is because an end-position can only be attained if three_d::renderer::pick is succesfull.
    // if mb1 is released and pick is unsuccessfull (e.g. outside of map), this function cant be invoked.
    // wont be a problem if we can guarantee that all picks are successfull, i.e. that the user never sees anything outside of the map.
    // as of know we cant gurantee that.
    pub fn select_in_bounds(
        &mut self,
        entities: &mut DongoEntityManager,
        start: Vec3,
        end: Vec3,
        context: &Context,
    ) {
        // remove previous selection
        self.clear_selection(entities);

        let inside = entities.get_all_within_bounds(start, end);
        inside.iter().for_each(|e| {
            if e.has_tag(TAG_SELECTABLE) {
                self.selected.push(e.id().expect("Tried to select an entity without an id!"));
            }
            // dbg!(e.id());
        });


        self.add_markers_to_render(entities, context);
    }

    // removes the selection box, and creates a new one
    pub fn resize_selection(
        &mut self,
        entities: &mut DongoEntityManager,
        mut start: Vec3,
        mut end: Vec3,
        context: &Context,
    ) {
        start.z = crate::common::MAP_MIN_HEIGHT as f32;
        end.z = crate::common::MAP_MAX_HEIGHT as f32 + SELECTION_BOX_HEIGHT_EXTRA;

        match self.selection_box {
            Some(selection_box_id) => {
                let selection_box = entities.get_entity_by_id_mut(selection_box_id).unwrap();
                if let DongoEntity::Object(mmp,_,_) = selection_box {
                    let positions = create_box_positions(start, end).to_vec();
                    mmp.mesh_mut().update_positions(&positions);                    
                }
                else {
                    panic!("selection box is not a DongoEntity::Object");
                }
            }
            None => {
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
                
                let id = entities.add_entity_from_gm(selectionbox_gm, DongoMetadata::new(Some("SelectionBox"), vec![TAG_SELECTION_BOX,TAG_NO_LIGHT]));

                self.selection_box = Some(id);
            }
        }
    }
}
