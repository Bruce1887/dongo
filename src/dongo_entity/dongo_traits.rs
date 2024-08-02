use crate::*;
use three_d::*;

pub trait DongoEntity {
    fn id(&self) -> Option<ENTITYID>;

    fn de_type(&self) -> &DongoEntityType;

    fn pos(&self) -> Vec3;

    fn set_pos(&mut self, pos: Vec3);

    fn add_to_pos(&mut self, pos: Vec3);

    fn transform(&self) -> Mat4;

    fn set_transform(&mut self, transform: Mat4);

    fn is_within_bounds(&self, start: Vec3, end: Vec3) -> bool {
        let pos = self.pos();
        start.x.min(end.x) <= pos.x
            && pos.x <= start.x.max(end.x)
            && start.y.min(end.y) <= pos.y
            && pos.y <= start.y.max(end.y)
    }

    fn set_desc(&mut self, desc: String);

    fn desc(&self) -> &str;

    fn animate(&mut self, time: f32);
}

// used in DongoObject. Models provide handles for mesh and a material. Pure objects dont, thats why this trait is needed.
pub trait MeshMaterialProvider {
    fn geometry(&self) -> &dyn Geometry;
    fn material(&self) -> &dyn Material;
    fn object(&self) -> &dyn Object;
    fn mesh(&self) -> &Mesh;
    fn mesh_mut(&mut self) -> &mut Mesh;
}

// perhaps i should change the dyn stuff to generics, but i dont know how to do that yet :)
impl<M: Material> MeshMaterialProvider for Gm<Mesh, M> {
    fn geometry(&self) -> &dyn Geometry {
        self
    }

    fn material(&self) -> &dyn Material {
        &self.material
    }

    fn object(&self) -> &dyn Object {
        self
    }

    fn mesh(&self) -> &Mesh {
        &self.geometry
    }

    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.geometry
    }
}
