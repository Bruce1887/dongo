use crate::*;
use three_d::*;

pub struct DongoObject {
    pub(crate) id: u16,
    pub(crate) object: Box<dyn Object>, // this is what it is all about
    pub(crate) o_type: DongoObjectType,
}
impl DongoObject {
    pub(crate) fn new(id: u16, object: Box<dyn Object>, o_type: DongoObjectType) -> DongoObject {
        DongoObject { id, object, o_type }
    }

    pub fn get_object(&self) -> &Box<dyn Object> {
        &self.object
    }
}
impl DongoObjectTraits for DongoObject{
    fn get_id(&self) -> u16 {
        self.id
    }

    fn get_type(&self) -> &DongoObjectType {
        &self.o_type
    }

    fn get_aabb_center(&self) -> Vec3 {
        dbg!(self as &dyn DongoObjectTraits);
        dbg!(self.object.aabb().min(), self.object.aabb().max());
        dbg!(self.object.aabb().center());
        self.object.aabb().center()
    }
}




