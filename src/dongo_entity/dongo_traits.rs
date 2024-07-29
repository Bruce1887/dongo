use std::fmt::Debug;
use three_d::*;

#[derive(PartialEq, Debug)]
pub enum DongoEntityType {
    Map,
    MapEntity,
    PlayerEntity,
    Selection,
    UI,
}

pub trait DongoEntity {
    fn get_id(&self) -> u16;

    fn get_type(&self) -> &DongoEntityType;

    fn get_pos(&self) -> Vec3;

    fn set_pos(&mut self, pos: Vec3);

    fn add_to_pos(&mut self, pos: Vec3);
}

impl Debug for dyn DongoEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, o_type: {:?} ", self.get_id(), self.get_type())
    }
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