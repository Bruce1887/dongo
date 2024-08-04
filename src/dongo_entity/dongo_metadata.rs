use super::{DONGOTAG, ENTITYID};

pub struct DongoMetadata{
    id: Option<ENTITYID>, // TODO: it is really bad that id is public. perhaps an intermediary struct should be used
    pub desc: Option<String>,
    pub tags: Vec<DONGOTAG>,
}

impl DongoMetadata {
    pub fn id(&self) -> Option<ENTITYID> {
        self.id
    }

    // ACHTUNG! Dont set id unless you know what you are doing. It is used in DongoEntityManager::add_entity
    pub fn set_id_achtung(&mut self, id: ENTITYID) {
        self.id = Some(id);
    }
    
    pub fn new_empty() -> DongoMetadata {
        DongoMetadata {
            id: None,
            desc: None,
            tags: Vec::new(),
        }
    }

    pub fn new(desc: Option<&str>, tags: Vec<DONGOTAG>) -> DongoMetadata {
        DongoMetadata {
            id: None,
            desc: match desc {
                Some(desc) => Some(desc.to_string()),
                None => None,
            },
            tags,
        }
    }
}