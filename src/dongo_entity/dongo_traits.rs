//use crate::*;
use three_d::*;

// used in DongoObject. Models provide handles for mesh and a material through their modelparts. Pure objects dont, thats why this trait is needed.
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
