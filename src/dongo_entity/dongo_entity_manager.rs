use crate::*;
use three_d::*;

pub struct DongoEntityManager {
    pub(crate) e_vec: Vec<DongoEntity>,
    next_vacant_id: ENTITYID,
    has_changes: bool,
}

impl DongoEntityManager {
    pub fn new() -> DongoEntityManager {
        DongoEntityManager {
            e_vec: Vec::new(),
            next_vacant_id: ENTITYID::MIN,
            has_changes: false,
        }
    }
    
    fn _get_entities(&self) -> &Vec<DongoEntity> {
        &self.e_vec
    }

    pub fn has_changes(&self) -> bool {
        self.has_changes
    }
    
    pub fn get_objects(&self) -> Vec<&dyn Object> {
        let mut objects: Vec<&dyn Object> = Vec::new();

        self.e_vec.iter().for_each(|e| {
            match e {
                DongoEntity::Object(mmp, _,_) => objects.push(mmp.object()),
                DongoEntity::Model(m, _,_) => m.iter().for_each(|part| objects.push(part)),
                DongoEntity::Terrain(mmp,_ ,_ ,_ ) => objects.push(mmp.object()),
                // _ => (),
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
    
        self.e_vec.iter().filter(|e| predicate(*e)).for_each(|e| {
            match e {
                DongoEntity::Object(mmp, _, _) => objects.push(mmp.object()),
                DongoEntity::Model(m, _, _) => m.iter().for_each(|part| objects.push(part.object())),
                DongoEntity::Terrain(mmp, _, _, _) => objects.push(mmp.object()),
            }
        });
    
        objects
    }

    pub fn add_entity(&mut self, mut entity: DongoEntity) -> ENTITYID {
        let id = self.next_vacant_id;
        entity.set_id_achtung(id);
        self.e_vec.push(entity);
        self.next_vacant_id += 1;
        self.has_changes = true;
        id
    }

    pub fn add_entity_from_gm<M: Material + 'static>(&mut self, gm: Gm<Mesh, M>, meta: DongoMetadata) -> ENTITYID {
        let id = self.next_vacant_id;
        self.add_entity(DongoEntity::Object(Box::new(gm), meta,Some(id)));
        self.next_vacant_id += 1;
        self.has_changes = true;
        id
    }

    pub fn get_entity_by_id(&self, id: u16) -> Option<&DongoEntity> {
        self.e_vec.iter().find(|e| e.id() == Some(id))
    }

    pub fn get_entity_by_id_mut(&mut self, id: u16) -> Option<&mut DongoEntity> {
        self.e_vec.iter_mut().find(|e| e.id() == Some(id))
    }

    pub fn take_entity_by_id(&mut self, id: u16) -> Option<DongoEntity> {
        self.e_vec.iter_mut().position(|e| e.id() == Some(id)).map(|i| self.e_vec.remove(i))
    }

    pub fn get_all_within_bounds(&self, start: Vec3, end: Vec3) -> Vec<&DongoEntity> {
        let inside = self.e_vec.iter().filter(|e| e.has_tag(TAG_SELECTABLE)).filter(|e| {
            e.is_within_bounds(start, end)
        });
        inside.collect()

        // let mut objects: Vec<&dyn Object> = Vec::new();
        // inside.for_each(|e| {
        //     match e {
        //         DongoEntity::Object(mmp,_, _) => objects.push(mmp.object()),
        //         DongoEntity::Model(m, _,_) => m.iter().for_each(|part| objects.push(part)),
        //         // DongoEntity::ColorModel(m) => m.iter().for_each(|part| objects.push(part)),
        //     }
        // });
        // objects
    }

    pub fn get_map(&self) -> Option<&DongoEntity> {
        self.e_vec.iter().find(|e| e.has_tag(TAG_MAP))
    }
}