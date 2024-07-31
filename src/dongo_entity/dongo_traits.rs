use std::fmt::Debug;
use three_d::*;

#[derive(PartialEq, Debug, Clone,Copy)]
pub enum DongoEntityType {
    WorldTerrain, // the terrain. only one of these
    WorldEntity, // an object on the terrain, non interactive. e.g. a tree
    PlayerEntity(u8), // an object belonging to player wit id u8
    SelectionBox, // a box that is used to select objects
    SelectionMarker(u16), // a marker that is used to show that an object is selected. u16 is the id of the selected object
    UI, 
}

pub trait DongoEntity {
    fn id(&self) -> Option<u16>;

    fn de_type(&self) -> &DongoEntityType;

    fn pos(&self) -> Vec3;

    fn set_pos(&mut self, pos: Vec3);

    fn add_to_pos(&mut self, pos: Vec3);

    fn is_within_bounds(&self, start: Vec3, end: Vec3) -> bool {
        let pos = self.pos();
        start.x.min(end.x) <= pos.x && pos.x <= start.x.max(end.x) &&
        start.y.min(end.y) <= pos.y && pos.y <= start.y.max(end.y)
    }

    fn set_desc(&mut self, desc: String){
        panic!("Default implementation is not good enough!");
    }

    fn desc(&self) -> &str {
        panic!("Default implementation is not good enough!")
    }
    
    fn animate(&mut self, time: f32) {
        panic!("Default implementation is not good enough!")
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