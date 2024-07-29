use crate::*;

use three_d::*;

pub const MAP_ID: u16 = u16::MAX;
pub const SELECTION_ID: u16 = u16::MAX - 1;

pub struct DongoObjectManager {
    objects: Vec<DongoObject>,
    object_id: u16,
    pub models: Vec<DongoModel>,
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

    pub fn get_objects(&mut self) -> &mut Vec<DongoObject> {
        &mut self.objects
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