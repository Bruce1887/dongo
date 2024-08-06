use crate::*;
use three_d::*;

pub enum DongoEntity{
    Object(Box<dyn MeshMaterialProvider>,DongoMetadata,Option<ENTITYID>),
    Model(Model<PhysicalMaterial>,DongoMetadata,Option<ENTITYID>), 
    Terrain(Box<dyn MeshMaterialProvider>,DongoMetadata,Option<ENTITYID>, DongoTerrainMetadata),
    //ColorModel(Model<ColorMaterial>),
}


impl DongoEntity {
    pub fn from_obj_file(
        context: &Context,
        obj_filename: &str,
        meta: DongoMetadata
    ) -> DongoEntity {
        let path = format!("assets/{}/massaged_{}.obj", obj_filename,obj_filename);
        let mut loaded = three_d_asset::io::load(&[path]).unwrap();
        let model = loaded.deserialize(format!("{}.obj",obj_filename)).unwrap();

        let mut model_mat = three_d::Model::<PhysicalMaterial>::new(&context, &model).unwrap();

        // set cull to back
        model_mat.iter_mut().for_each(|part| {
            part.material.render_states.cull = Cull::Back;  
        });

        DongoEntity::Model(model_mat, meta,None)
    } 

    pub fn from_gm<M: Material + 'static>(gm: Gm<Mesh, M>, meta: DongoMetadata) -> DongoEntity {
        DongoEntity::Object(Box::new(gm), meta,None)
    }

    pub fn id(&self) -> Option<ENTITYID>{
        match self {
            DongoEntity::Object(_,_,id) => *id,
            DongoEntity::Model(_,_,id) => *id,
            DongoEntity::Terrain(_,_,id,_) => *id,
        }
    
    }

    /// ACHTUNG! This function should be avoided and used carefully when necessary.
    /// It is up to the caller to ensure that the id is unique.
    /// It should generally only be used when current id is None.
    pub fn set_id_achtung(&mut self, new_id: ENTITYID) {
        match self {
            DongoEntity::Object(_,_,current_id) => *current_id = Some(new_id),
            DongoEntity::Model(_,_,current_id) => *current_id = Some(new_id),
            DongoEntity::Terrain(_,_,current_id,_) => *current_id = Some(new_id),
        }
    }

    pub fn metadata(&self) -> &DongoMetadata {
        match self {
            DongoEntity::Object(_, meta,_) => meta,
            DongoEntity::Model(_, meta,_) => meta,
            DongoEntity::Terrain(_, meta,_, _ ) => meta,
        }
    }
    
    pub fn metadata_mut(&mut self) -> &mut DongoMetadata {
        match self {
            DongoEntity::Object(_, meta,_) => meta,
            DongoEntity::Model(_, meta,_) => meta,
            DongoEntity::Terrain(_, meta,_, _ ) => meta,
        }
    }

    pub fn has_tag(&self, tag: DONGOTAG) -> bool {
        self.metadata().tags.contains(&tag)
    }

    pub fn pos(&self) -> Vec3 {
        match self {
            DongoEntity::Object(mmp,_,_) => {
                let transform = mmp.mesh().transformation();
                let (x, y, z) = (transform.w.x, transform.w.y, transform.w.z);
                vec3(x, y, z)
            },
            DongoEntity::Model(model,_,_) => {
                let transform = model.first().unwrap().transformation(); // WARNING: this can fail if the model is empty (it should never be empty i think?)
                let (x, y, z) = (transform.w.x, transform.w.y, transform.w.z);
                vec3(x, y, z)
            },
            _ => panic!("pos() not implemented for this entity type"),
        }
    }

    pub fn set_pos(&mut self, pos: Vec3) {
        match self {
            DongoEntity::Object(mmp,_,_) => {
                
                let mut transform = mmp.mesh().transformation();

                transform.w = vec4(pos.x, pos.y, pos.z, 1.0);
                mmp.mesh_mut().set_transformation(transform);
            },
            DongoEntity::Model(model, _,_) => {
                model.iter_mut().for_each(|part| {
                    let mut transform = part.transformation();
                    transform.w = vec4(pos.x, pos.y, pos.z, 1.0);
                    part.set_transformation(transform);
                });
            },
            _ => panic!("set_pos() not implemented for this entity type"),
        }
    }

    pub fn set_transform(&mut self, transform: Mat4) {
        match self {
            DongoEntity::Object(mmp, _,_) => {
                mmp.mesh_mut().set_transformation(transform);
            },
            DongoEntity::Model(model, _,_) => {
                model.iter_mut().for_each(|part| part.set_transformation(transform));
            },
            _ => panic!("set_transform() not implemented for this entity type"),
        }
    }

    pub fn transform(&self) -> Mat4 {
        match self {
            DongoEntity::Object(mmp,_,_) => mmp.mesh().transformation(),
            DongoEntity::Model(model,_,_) => model.first().unwrap().transformation(),
            _ => panic!("transform() not implemented for this entity type"),
        }
    }

    pub fn animate(&mut self, delta_time: f32) {
        match self {
            DongoEntity::Object(mmp,_,_) => mmp.mesh_mut().animate(delta_time),
            DongoEntity::Model(model,_,_) => model.iter_mut().for_each(|part| part.animate(delta_time)),
            _ => panic!("animate() not implemented for this entity type"),
            
        }
    }

    pub fn is_within_bounds(&self, start: Vec3, end: Vec3) -> bool {
        let pos = self.pos();
        start.x.min(end.x) <= pos.x
            && pos.x <= start.x.max(end.x)
            && start.y.min(end.y) <= pos.y
            && pos.y <= start.y.max(end.y)
    }

    pub fn get_height_at(&self, x: f32, y: f32) -> f32 {
        match self {
            DongoEntity::Terrain(_,_,_,dt_meta) => {

                dt_meta.get_height_at(x as f64, y as f64) as f32
            },
            _ => panic!("get_height_at() not implemented for this entity type"),
        }
    }

}