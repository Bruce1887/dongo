use crate::*;
use three_d::*;

pub struct DongoObject {
    pub(crate) id: u16,
    pub(crate) mm_provider: Box<dyn MeshMaterialProvider>, // this is what it is all about
    pub(crate) o_type: DongoEntityType,
}

impl DongoObject {
    pub(crate) fn new(id: u16, mmp: Box<dyn MeshMaterialProvider>, o_type: DongoEntityType) -> DongoObject {
        DongoObject {
            id,
            mm_provider: mmp,
            o_type,
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
    fn get_id(&self) -> u16 {
        self.id
    }

    fn get_type(&self) -> &DongoEntityType {
        &self.o_type
    }

    fn get_pos(&self) -> Vec3 {
        let transform = self.mm_provider.mesh().transformation();
        let (x,y,z) = (transform.w.x, transform.w.y, transform.w.z);
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
}
