use std::fmt::Debug;

use three_d::*;
pub const MAP_ID: u16 = u16::MAX;
pub const SELECTION_ID: u16 = u16::MAX - 1;

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

    fn get_aabb_center(&self) -> Vec3 {
        panic!("Default implementation is not good enough!");
    }
}

impl Debug for dyn DongoObjectTraits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, o_type: {:?} ", self.get_id(), self.get_type())
    }
}

pub struct DongoObject {
    id: u16,
    object: Box<dyn Object>, // this is what it is all about
    o_type: DongoObjectType,
}
impl DongoObject {
    fn new(id: u16, object: Box<dyn Object>, o_type: DongoObjectType) -> DongoObject {
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
        self.object.aabb().center()
    }
}

pub struct DongoModel {
    id: u16,
    model: Model<PhysicalMaterial>,
    o_type: DongoObjectType,
}
impl DongoModel {
        
}
impl DongoObjectTraits for DongoModel{
    fn get_id(&self) -> u16 {
        self.id
    }

    fn get_type(&self) -> &DongoObjectType {
        &self.o_type
    }

    fn get_aabb_center(&self) -> Vec3 {
        let num_mps = self.model.len();
        let mut vec_sum = Vector3::zero();
        for idx in 0..num_mps {
            vec_sum += self.model[idx].aabb().center();
        }
        vec_sum / num_mps as f32
    }
}


pub struct DongoObjectManager {
    objects: Vec<DongoObject>,
    object_id: u16,
    models: Vec<DongoModel>,
}

impl DongoObjectManager {
    pub fn new() -> DongoObjectManager {
        DongoObjectManager {
            objects: Vec::new(),
            object_id: 0,
            models: Vec::new(),
        }
    }

    // used in mouse_selection among others
    pub fn add_object_with_idx(&mut self, idx: u16, object: Box<dyn Object>, o_type: DongoObjectType) {
        self.objects.push(DongoObject::new(idx, object, o_type));
    }

    pub fn add_object(&mut self, object: Box<dyn Object>, o_type: DongoObjectType) {
        self.objects.push(DongoObject::new(self.object_id, object, o_type));
        self.object_id += 1;
    }

    pub fn add_model(&mut self, model: Model<PhysicalMaterial>, o_type: DongoObjectType){
        self.models.push(DongoModel{
            id: self.object_id,
            model,
            o_type,
        });
        self.object_id += 1;
    }

    // pub fn remove_many_objects(&mut self, closure: impl Fn(&DongoObject) -> bool) {
    //     self.objects.retain(|obj| !closure(obj));
    // }

    pub fn take_obj(&mut self, id: u16) -> Option<DongoObject> {
        if let Some(index) = self.objects.iter().position(|obj| obj.get_id() == id) {
            Some(self.objects.remove(index))
        } else {
            None
        }
    }

    pub fn get_object_by_id(&self, id: u16) -> Option<&DongoObject> {
        for obj in &self.objects {
            if obj.get_id() == id {
                return Some(obj);
            }
        }
        None
    }

    pub fn get_objects(&self) -> &Vec<DongoObject> {
        &self.objects
    }
    pub fn get_models(&self) -> &Vec<DongoModel> {
        &self.models
    }  
    pub fn get_objects_vec(&self, predicate: impl Fn(&DongoObject) -> bool) -> Vec<&dyn Object> {
        self.objects
            .iter()
            .filter(|obj| predicate(obj))
            .map(|obj| obj.get_object().as_ref())
            .chain(self.get_models_vec().collect::<Vec<&dyn Object>>())
            .collect()
    }

    fn get_models_vec(&self) -> impl Iterator<Item = &dyn Object>{
        self.models.iter().flat_map(|m| m.model.iter().map(|part| part as &dyn Object))
    }
}

// can be passed to get_vec to get all objects
pub const fn no_predicate(_: &crate::dongo_object::DongoObject) -> bool {
    true
}
