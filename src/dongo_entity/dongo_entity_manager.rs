use crate::*;
use three_d::*;

pub const MAP_ID: u16 = u16::MAX;
pub const SELECTION_ID: u16 = u16::MAX - 1;

pub struct DongoEntityManager {
    objects: Vec<DongoObject>,
    next_vacant_id: ENTITYID,
    pub models: Vec<DongoModel>,
}

impl std::fmt::Display for DongoEntityManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        str.push_str("___Entities___\n");
        str.push_str("==============\n");
        str.push_str("   Objects    \n");
        for obj in &self.objects {
            str.push_str(&format!("{}\n", obj));
        }
        str.push_str("\n");
        str.push_str("    Models   \n");
        for model in &self.models {
            str.push_str(&format!("{}\n", model));
        }
        str.push_str("==============\n");
        write!(f, "{}", str)
    }
}

impl DongoEntityManager {
    pub fn new() -> DongoEntityManager {
        DongoEntityManager {
            objects: Vec::new(),
            next_vacant_id: 0,
            models: Vec::new(),         
        }
    }

    // used in mouse_selection among others. Should only be used for certain stuff.
    pub fn add_object_with_id(
        &mut self,
        id: ENTITYID,
        mm_provider: Box<dyn MeshMaterialProvider>,
        e_type: DongoEntityType,
    ) {
        self.objects.push(DongoObject::new_with_id(id, mm_provider, e_type));
    }

    pub fn add_object_from_mmp(
        &mut self,
        mm_provider: Box<dyn MeshMaterialProvider>,
        e_type: DongoEntityType,
    ) {
        self.objects
            .push(DongoObject::new_with_id(self.next_vacant_id, mm_provider, e_type));
        self.next_vacant_id += 1;
    }

    pub fn add_dongo_object(&mut self, mut dongo_object: DongoObject) {
        dongo_object.id = Some(self.next_vacant_id);
        self.objects.push(dongo_object);
        self.next_vacant_id += 1;
    }

    pub fn add_dongo_model(&mut self, mut dmodel: DongoModel) {
        dmodel.id = Some(self.next_vacant_id);
        self.models.push(dmodel);
        self.next_vacant_id;
    }

    pub fn add_dongomodel_from_model(
        &mut self,
        model: Model<PhysicalMaterial>,
        e_type: DongoEntityType,
    ) {
        self.models.push(DongoModel {
            id: Some(self.next_vacant_id),
            desc: None,
            model,
            e_type,
        });
        self.next_vacant_id += 1;
    }

    // pub fn remove_many_objects(&mut self, closure: impl Fn(&DongoObject) -> bool) {
    //     self.objects.retain(|obj| !closure(obj));
    // }

    pub fn take_object(&mut self, id: u16) -> Option<DongoObject> {
        if let Some(index) = self.objects.iter().position(|obj| obj.id() == Some(id)) {
            Some(self.objects.remove(index))
        } else {
            None
        }
    }

    pub fn take_model(&mut self, id: u16) -> Option<DongoModel> {
        if let Some(index) = self.models.iter().position(|model| model.id == Some(id)) {
            Some(self.models.remove(index))
        } else {
            None
        }
    }

    pub fn get_object_by_id(&mut self, id: u16) -> Option<&mut DongoObject> {
        for obj in &mut self.objects {
            if obj.id() == Some(id) {
                return Some(obj);
            }
        }
        None
    }

    pub fn get_model_by_id(&mut self, id: u16) -> Option<&mut DongoModel> {
        for model in &mut self.models {
            if model.id == Some(id) {
                return Some(model);
            }
        }
        None
    }

    pub fn get_entity_by_id(&mut self, id: u16) -> Option<&mut dyn DongoEntity> {
        let found = self
            .objects
            .iter_mut()
            .map(|obj| obj as &mut dyn DongoEntity)
            .chain(
                self.models
                    .iter_mut()
                    .map(|model| model as &mut dyn DongoEntity),
            )
            .find(|e| e.id() == Some(id));

        found
    }

    pub fn get_objects(&mut self) -> &mut Vec<DongoObject> {
        &mut self.objects
    }
    pub fn get_models(&self) -> &Vec<DongoModel> {
        &self.models
    }

    pub fn all_as_entities(&mut self) -> Vec<&mut dyn DongoEntity> {
        let mut entities: Vec<&mut dyn DongoEntity> = Vec::new();
        entities.extend(
            self.objects
                .iter_mut()
                .map(|obj| obj as &mut dyn DongoEntity),
        );
        entities.extend(
            self.models
                .iter_mut()
                .map(|model| model as &mut dyn DongoEntity),
        );
        entities
    }

    // gets all objects and models that satisfy the predicate
    pub fn all_as_object(&self, predicate: impl Fn(&dyn DongoEntity) -> bool) -> Vec<&dyn Object> {
        self.objects
            .iter()
            .filter(|obj| predicate(*obj as &dyn DongoEntity))
            .map(|obj| obj.get_object())
            .chain(
                self.models
                    .iter()
                    .filter(|m| predicate(*m as &dyn DongoEntity))
                    .flat_map(|m| m.model.iter().map(|part| part as &dyn Object)),
            )
            .collect()
    }

    pub fn get_all_within_bounds(
        &self,
        start: Vec3,
        end: Vec3,
    ) -> Vec<(Option<ENTITYID>, DongoEntityType)> {
        let mut entities: Vec<&dyn DongoEntity> = Vec::new();
        entities.extend(
            self.objects
                .iter()
                .filter(|obj| obj.is_within_bounds(start, end))
                .map(|obj| obj as &dyn DongoEntity),
        );
        entities.extend(
            self.models
                .iter()
                .filter(|model| model.is_within_bounds(start, end))
                .map(|model| model as &dyn DongoEntity),
        );

        let mut r_value: Vec<(Option<ENTITYID>, DongoEntityType)> = Vec::new();
        for e in entities {
            r_value.push((e.id(), *e.de_type()));
        }
        r_value
    }

    pub fn get_selected(&self) -> Vec<ENTITYID> {
        let mut selected: Vec<ENTITYID> = Vec::new();
        self.objects.iter().map(|obj| obj as &dyn DongoEntity).chain(
            self.models.iter().map(|obj| obj as &dyn DongoEntity)
        ).for_each(|e| {
            if let DongoEntityType::NonSelectable { entity: NonSelectableEntity::SelectionMarker(id) } = e.de_type() {
                selected.push(*id);
            }
        });
        selected
    }

    pub fn drop_selected(&mut self){

    }
}

// can be passed to get_vec to get all objects
pub const fn no_predicate(_: &dyn crate::dongo_traits::DongoEntity) -> bool {
    true
}
