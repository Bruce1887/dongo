use three_d::*;

pub enum DongoObjectType {
    Map,
    MapEntity,
    PlayerEntity,
    Selection,
}

pub struct DongoObject {
    id: i16,
    object: Box<dyn Object>, // this is what it is all about
    _o_type: DongoObjectType,
}

impl DongoObject {
    pub fn new(id: i16, object: Box<dyn Object>,o_type: DongoObjectType) -> DongoObject {
        DongoObject {
            id,
            object,
            _o_type: o_type,
        }
    }

    pub fn get_id(&self) -> i16 {
        self.id
    }

    pub fn get_object(&self) -> &Box<dyn Object> {
        &self.object
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

    pub fn add_object(&mut self, id: i16, object: Box<dyn Object>, o_type: DongoObjectType) {
        self.objects.push(DongoObject::new(id, object, o_type));
    }

    pub fn get_object(&self, id: i16) -> Option<&DongoObject> {
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

    pub fn get_vec(&self) -> Vec<&dyn Object> {
        self.objects.iter().map(|obj| obj.get_object().as_ref()).collect()
    }
    
}