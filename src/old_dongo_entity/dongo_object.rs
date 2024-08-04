use crate::*;
use three_d::*;

pub struct DongoObject {
    pub(crate) id: Option<ENTITYID>,
    pub(crate) desc: Option<String>,
    pub(crate) mm_provider: Box<dyn MeshMaterialProvider>, // this is what it is all about
    pub(crate) e_type: DongoEntityType,
}

impl std::fmt::Display for DongoObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.id {
            Some(id) => write!(f, "{:?}, {:?}, {}", id, self.e_type, self.desc()),
            None => write!(f, "None, {:?}", self.e_type),
        }
    }
}

impl DongoObject {
    pub(crate) fn new_with_id(
        id: ENTITYID,
        mmp: Box<dyn MeshMaterialProvider>,
        e_type: DongoEntityType,
    ) -> DongoObject {

        // set cull to back
        mmp.material().render_states().cull = Cull::Front; // this has no effect

        DongoObject {
            id: Some(id),
            desc: None,
            mm_provider: mmp,
            e_type,
        }
    }

    pub fn from_gm<M: Material + 'static>(gm: Gm<Mesh, M>, e_type: DongoEntityType) -> DongoObject {
        
        // set cull to back
        gm.material.render_states().cull = Cull::Back; // this has no effect

        DongoObject {
            id: None,
            desc: None,
            mm_provider: Box::new(gm),
            e_type,
        }
    }

    pub fn get_object(&self) -> &dyn Object {
        self.mm_provider.object()
    }
    pub fn get_material(&self) -> &dyn Material {
        self.mm_provider.material()
    }
    pub fn get_geometry(&self) -> &dyn Geometry {
        self.mm_provider.geometry()
    }
}
impl DongoEntity for DongoObject {
    fn id(&self) -> Option<ENTITYID> {
        self.id
    }

    fn de_type(&self) -> &DongoEntityType {
        &self.e_type
    }

    fn pos(&self) -> Vec3 {
        let transform = self.mm_provider.mesh().transformation();
        let (x, y, z) = (transform.w.x, transform.w.y, transform.w.z);
        vec3(x, y, z)
    }

    fn set_pos(&mut self, pos: Vec3) {
        let mut transform = self.mm_provider.mesh_mut().transformation().clone();
        transform.w = vec4(pos.x, pos.y, pos.z, 1.0);

        self.mm_provider.mesh_mut().set_transformation(transform);
    }

    fn add_to_pos(&mut self, pos: Vec3) {
        let mut transform = self.mm_provider.mesh_mut().transformation();
        transform.w += vec4(pos.x, pos.y, pos.z, 0.0);
        self.mm_provider.mesh_mut().set_transformation(transform);
    }

    fn transform(&self) -> Mat4 {
        self.mm_provider.mesh().transformation()
    }

    fn set_transform(&mut self, transform: Mat4) {
        self.mm_provider.mesh_mut().set_transformation(transform);
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
        self.mm_provider.mesh_mut().animate(time);
    }
}
