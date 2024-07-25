use three_d::*;

#[derive(PartialEq)]
pub enum DongoObjectType {
    Map,
    MapEntity,
    PlayerEntity,
    Selection,
    UI,
}

// pub(crate) const MAP_IDX: u1pub const MAP_IDX: u16 = u16::MAX;6 = u16::MAX;
pub(crate) const SELECTION_IDX: u16 = u16::MAX - 1;

pub struct DongoObject {
    id: u16,
    object: Box<dyn Object>, // this is what it is all about
    o_type: DongoObjectType,
}

impl DongoObject {
    pub fn new(id: u16, object: Box<dyn Object>, o_type: DongoObjectType) -> DongoObject {
        DongoObject { id, object, o_type }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_object(&self) -> &Box<dyn Object> {
        &self.object
    }
    pub fn get_type(&self) -> &DongoObjectType {
        &self.o_type
    }
}

pub struct DongoObjectManager {
    objects: Vec<DongoObject>,
}

impl DongoObjectManager {
    pub fn new() -> DongoObjectManager {
        DongoObjectManager {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, id: u16, object: Box<dyn Object>, o_type: DongoObjectType) {
        self.objects.push(DongoObject::new(id, object, o_type));
    }

    pub fn remove_many_objects(&mut self, closure: impl Fn(&DongoObject) -> bool) {
        self.objects.retain(|obj| !closure(obj));
    }

    pub fn take_obj(&mut self, id: u16) -> Option<DongoObject> {
        if let Some(index) = self.objects.iter().position(|obj| obj.get_id() == id) {
            Some(self.objects.remove(index))
        } else {
            None
        }
    }

    pub fn get_object(&self, id: u16) -> Option<&DongoObject> {
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

    pub fn get_vec(&self, predicate: impl Fn(&DongoObject) -> bool) -> Vec<&dyn Object> {
        self.objects
            .iter()
            .filter(|obj| predicate(obj))
            .map(|obj| obj.get_object().as_ref())
            .collect()
    }
}

// can be passed to get_vec to get all objects
pub const fn no_predicate(_: &crate::dongo_object::DongoObject) -> bool {
    true
}
