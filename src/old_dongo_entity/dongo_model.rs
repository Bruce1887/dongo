use crate::*;
use three_d::*;

pub struct DongoModel {
    pub(crate) id: Option<ENTITYID>,
    pub(crate) desc: Option<String>,
    pub(crate) model: Model<PhysicalMaterial>,
    pub(crate) e_type: DongoEntityType,
}

impl std::fmt::Display for DongoModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.id {
            Some(id) => write!(f, "{:?}, {:?}, {}", id, self.e_type, self.desc()),
            None => write!(f, "None, {:?}", self.e_type),
        }
    }
}

impl DongoModel {

    // 'obj_filename' does not include suffix ".obj"
    pub fn from_obj_file(
        context: &Context,
        obj_filename: &str,
        e_type: DongoEntityType,
    ) -> DongoModel {
        let path = format!("assets/{}/massaged_{}.obj", obj_filename,obj_filename);
        let mut loaded = three_d_asset::io::load(&[path]).unwrap();
        let model = loaded.deserialize(format!("{}.obj",obj_filename)).unwrap();
        let mut model_mat = three_d::Model::<PhysicalMaterial>::new(&context, &model).unwrap();

        
        // set cull to back
        model_mat.iter_mut().for_each(|part| {
            part.material.render_states.cull = Cull::Back;
        });

        DongoModel {
            id: None,
            desc: None,
            model: model_mat,
            e_type,
        }
    }
}
impl DongoEntity for DongoModel {
    fn id(&self) -> Option<ENTITYID> {
        self.id
    }

    fn de_type(&self) -> &DongoEntityType {
        &self.e_type
    }

    fn pos(&self) -> Vec3 {
        // i suppose it is fine to check the first modelpart only, as long as all modelparts are moved together
        let transform = self.model.first().unwrap().transformation(); // this will panic if a model contains 0 modelparts
        let (x, y, z) = (transform.w.x, transform.w.y, transform.w.z);
        vec3(x, y, z)
    }
    
    fn set_pos(&mut self, pos: Vec3) {
        self.model.iter_mut().for_each(|part| {
            let mut transform = part.transformation().clone();
            transform.w = vec4(pos.x, pos.y, pos.z, transform.w.w);
            part.set_transformation(transform);
        });
    }

    fn add_to_pos(&mut self, pos: Vec3) {
        self.model.iter_mut().for_each(|part| {
            let mut transform = part.transformation().clone();
            transform.w += vec4(pos.x, pos.y, pos.z, 0.0);
            part.set_transformation(transform);
        });
    }

    fn transform(&self) -> Mat4 {
        self.model.first().unwrap().transformation()
    }

    fn set_transform(&mut self, transform: Mat4) {
        self.model.iter_mut().for_each(|part| {
            part.set_transformation(transform);
        })
    }

    fn desc(&self) -> &str {
        match &self.desc {
            Some(desc) => desc,
            None => "No description provided",
        }
    }
    fn set_desc(&mut self, desc: String) {
        self.desc = Some(desc);
    }

    fn animate(&mut self, time: f32) {
        self.model.iter_mut().for_each(|part| {
            part.animate(time);
        });
    }
}
