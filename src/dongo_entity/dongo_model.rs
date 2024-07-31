use crate::*;
use three_d::*;

pub struct DongoModel {
    pub(crate) id: Option<u16>,
    pub(crate) desc : Option<String>,
    pub(crate) model: Model<PhysicalMaterial>,
    pub(crate) e_type: DongoEntityType,
}

impl std::fmt::Display for DongoModel{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.id {
            Some(id) => write!(f, "{:?}, {:?}, {}", id, self.e_type, self.desc()), 
            None => write!(f, "None, {:?}", self.e_type),
        }        
    }
}

impl DongoModel {
    pub fn foo(&mut self) {
        self.model.iter_mut().for_each(|part| {
            // dbg!(m.geometry.transformation());
            // dbg!(m.transformation());
            let transform = part.transformation().clone();
            part.set_transformation(transform * Mat4::from_translation(vec3(0.0, 0.0, 10.0)));
        });
    }
}
impl DongoEntity for DongoModel {
    fn id(&self) -> Option<u16> {
        self.id
    }

    fn de_type(&self) -> &DongoEntityType {
        &self.e_type
    }

    fn pos(&self) -> Vec3 {
        // i suppose it is fine to check the first modelpart only, as long as all modelparts are moved together
        let transform = self.model.first().unwrap().transformation(); // this will panic if a model contains 0 modelparts
        let (x,y,z) = (transform.w.x, transform.w.y, transform.w.z);
        vec3(x, y, z)
    }

    fn add_to_pos(&mut self, pos: Vec3) {
        self.model.iter_mut().for_each(|part| {
            let mut transform = part.transformation().clone();
            transform.w += vec4(pos.x, pos.y, pos.z, 0.0);
            part.set_transformation(transform);
        });
    }

    fn set_pos(&mut self, pos: Vec3) {
        self.model.iter_mut().for_each(|part| {
            let mut transform = part.transformation().clone();
            transform.w = vec4(pos.x, pos.y, pos.z, transform.w.w);
            part.set_transformation(transform);
        });
    }

    fn desc(&self) -> &str {
        match &self.desc {
            Some(desc) => desc,
            None => "No description provided",
        }
    }
    fn set_desc(&mut self, desc: String){
        self.desc = Some(desc);
    }

    fn animate(&mut self, time: f32) {
        self.model.iter_mut().for_each(|part| {
            part.animate(time);
        });
    }
}