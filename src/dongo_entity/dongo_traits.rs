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

    fn get_aabb_center(&self) -> Vec3;
}

impl Debug for dyn DongoObjectTraits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, o_type: {:?} ", self.get_id(), self.get_type())
    }
}
