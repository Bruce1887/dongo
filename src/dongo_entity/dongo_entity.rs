use ::core::panic;

use crate::*;
use three_d::*;

pub enum DongoEntity{
    Object(Box<dyn MeshMaterialProvider>,DongoMetadata),
    Model(Model<PhysicalMaterial>,DongoMetadata), 
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

        DongoEntity::Model(model_mat, meta)
    } 

    pub fn from_gm<M: Material + 'static>(gm: Gm<Mesh, M>, meta: DongoMetadata) -> DongoEntity {
        DongoEntity::Object(Box::new(gm), meta)
    }

    pub fn metadata(&self) -> &DongoMetadata {
        match self {
            DongoEntity::Object(_, meta) => meta,
            DongoEntity::Model(_, meta) => meta,
        }
    }
    
    pub fn metadata_mut(&mut self) -> &mut DongoMetadata {
        match self {
            DongoEntity::Object(_, meta) => meta,
            DongoEntity::Model(_, meta) => meta,
        }
    }

    pub fn has_tag(&self, tag: DONGOTAG) -> bool {
        self.metadata().tags.contains(&tag)
    }

    pub fn pos(&self) -> Vec3 {
        panic!()
    }

    pub fn set_pos(&mut self, pos: Vec3) {
        match self {
            DongoEntity::Object(mmp, _) => {
                let mut transform = mmp.mesh().transformation();
                transform.w = vec4(pos.x, pos.y, pos.z, 1.0);
                mmp.mesh_mut().set_transformation(transform);
            },
            DongoEntity::Model(model, _) => {
                model.iter_mut().for_each(|part| {
                    let mut transform = part.transformation();
                    transform.w = vec4(pos.x, pos.y, pos.z, 1.0);
                    part.set_transformation(transform);
                });
            },
            
        }
    }

    pub fn set_transform(&mut self, transform: Mat4) {
        match self {
            DongoEntity::Object(mmp, _) => {
                mmp.mesh_mut().set_transformation(transform);
            },
            DongoEntity::Model(model, _) => {
                model.iter_mut().for_each(|part| part.set_transformation(transform));
            },
        }
    }

    pub fn transform(&self) -> Mat4 {
        panic!()
    }

    pub fn animate(&mut self, delta_time: f32) {
        panic!()
    }

    /*
    fn get_objects(&self) -> Vec<&dyn Object> {
        match self {
            DongoEntity::Object(o, _) => vec![o.as_ref()],
            DongoEntity::Model(m, _) => m.iter().map(|part| part as &dyn Object).collect(),
            DongoEntity::ColorModel(m) => m.iter().map(|part| part as &dyn Object).collect(),
        }
    }
    */
    
}