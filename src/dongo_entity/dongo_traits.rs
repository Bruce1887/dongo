use three_d::*;
use std::fmt::Debug;

#[derive(PartialEq,Debug)]
pub enum DongoObjectType {
    Map,
    MapEntity,
    PlayerEntity,
    Selection,
    UI,
}

pub trait DongoObjectTraits{
    fn get_id(&self) -> u16;

    fn get_type(&self) -> &DongoObjectType;

    fn get_pos(&self) -> Vec3{
        panic!("There should not be a default implementation for get_pos. it is only here now to make the compiler happy");
    }

    fn set_pos(&mut self, pos: Vec3) {
        panic!("There should not be a default implementation for set_pos. it is only here now to make the compiler happy"); 
    }

    fn add_to_pos(&mut self, pos: Vec3){
        panic!("There should not be a default implementation for add_to_pos. it is only here now to make the compiler happy");
    }

    fn get_aabb_center(&self) -> Vec3;
}

impl Debug for dyn DongoObjectTraits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, o_type: {:?} ", self.get_id(), self.get_type())
    }
}
