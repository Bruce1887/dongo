use ::core::panic;

use crate::*;
use three_d::*;

pub struct DongoEntityManager {
    e_vec: Vec<DongoEntity>,
    next_vacant_id: ENTITYID,
}

impl DongoEntityManager {
    pub fn new() -> DongoEntityManager {
        DongoEntityManager {
            e_vec: Vec::new(),
            next_vacant_id: ENTITYID::MIN,
        }
    }
    
    fn _get_entities(&self) -> &Vec<DongoEntity> {
        &self.e_vec
    }

    pub fn get_objects(&self) -> Vec<&dyn Object> {
        let mut objects: Vec<&dyn Object> = Vec::new();

        self.e_vec.iter().for_each(|e| {
            match e {
                DongoEntity::Object(mmp, _) => objects.push(mmp.object()),
                DongoEntity::Model(m, _) => m.iter().for_each(|part| objects.push(part)),
                // DongoEntity::ColorModel(m) => m.iter().for_each(|part| objects.push(part)),
            }
        });   

        objects
    }

    pub fn filter_to_entities(&self, predicate: impl Fn(&DongoEntity) -> bool) -> Vec<&DongoEntity> {
        self.e_vec.iter().filter(|e| predicate(*e)).collect()
    }

    pub fn filter_to_entities_mut(&mut self, predicate: impl Fn(&DongoEntity) -> bool) -> Vec<&mut DongoEntity> {
        self.e_vec.iter_mut().filter(|e| predicate(*e)).collect()
    }

    pub fn filter_to_objects(&self, predicate: impl Fn(&DongoEntity) -> bool) -> Vec<&dyn Object> {
        let mut objects: Vec<&dyn Object> = Vec::new();

        self.e_vec.iter().filter(|e| predicate(*e) ).for_each(|e| {
            match e {
                DongoEntity::Object(mmp, _) => objects.push(mmp.object()),
                DongoEntity::Model(m, _) => m.iter().for_each(|part| objects.push(part)),
                // DongoEntity::ColorModel(m) => m.iter().for_each(|part| objects.push(part)),
            }
        });   

        objects
    }

    pub fn add_entity(&mut self, mut entity: DongoEntity) -> ENTITYID {
        let id = self.next_vacant_id;
        entity.metadata_mut().set_id_achtung(id);
        self.e_vec.push(entity);
        self.next_vacant_id += 1;
        id
    }

    pub fn add_entity_from_gm<M: Material + 'static>(&mut self, gm: Gm<Mesh, M>, mut meta: DongoMetadata) -> ENTITYID {
        let id = self.next_vacant_id;
        meta.set_id_achtung(id);
        self.add_entity(DongoEntity::Object(Box::new(gm), meta));
        self.next_vacant_id += 1;
        id
    }

    pub fn get_entity_by_id(&self, id: u16) -> Option<&DongoEntity> {
        self.e_vec.iter().find(|e| e.metadata().id() == Some(id))
    }

    pub fn get_entity_by_id_mut(&mut self, id: u16) -> Option<&mut DongoEntity> {
        self.e_vec.iter_mut().find(|e| e.metadata().id() == Some(id))
    }

    pub fn take_entity_by_id(&mut self, id: u16) -> Option<DongoEntity> {
        self.e_vec.iter_mut().position(|e| e.metadata().id() == Some(id)).map(|i| self.e_vec.remove(i))
    }

    pub fn get_all_within_bounds(&self, start: Vec3, end: Vec3) -> Vec<&dyn Object> {
        panic!()
    }
}