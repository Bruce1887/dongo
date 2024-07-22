/*
use three_d::*;
pub struct DongoGlobals{
    pub context: Context,
    pub camera: Camera,
    pub objects: Vec<Box<dyn Object>>,
    pub map: Gm<Mesh,PhysicalMaterial>,
    pick_mesh : Option<Mesh>,
}

impl DongoGlobals {
    pub fn new(context: Context, camera: Camera, objects: Vec<Box<dyn Object>>, map: Gm<Mesh,PhysicalMaterial>,) -> DongoGlobals {
        DongoGlobals {
            context,
            camera,
            objects,
            map,
            pick_mesh: None,
        }
    }

    pub fn map_object(&self) -> &Gm<Mesh,PhysicalMaterial> {
        &self.map
    }

    pub fn all_objects(&self) -> Vec<&dyn Object> {
        self.objects.iter().map(|obj| &**obj).collect::<Vec<&dyn Object>>()
    }

    pub fn map_and_all_objects(&self) -> Vec<&dyn Object> {
        let mut all_objects = self.all_objects();
        all_objects.push(self.map_object());
        all_objects
    }
}
*/
