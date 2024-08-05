pub type PLAYERID = u8;
pub type ENTITYID = u16;
pub type DONGOTAG = u8;

// Entity id's
pub const MAP_ID: ENTITYID = ENTITYID::MAX;
// pub const SELECTION_BOX_ID: ENTITYID = ENTITYID::MAX - 1;

// DongoMetadata Tags
pub const TAG_SELECTABLE: DONGOTAG = 0;
pub const TAG_NON_SELECTABLE: DONGOTAG = 1;
pub const TAG_UI: DONGOTAG = 2;
pub const TAG_MAP: DONGOTAG = 3;
pub const TAG_SELECTION_BOX: DONGOTAG = 4;
pub const TAG_SELECTION_MARKER: DONGOTAG = 5;
pub const TAG_HAS_ANIMATION: DONGOTAG = 6;

pub const fn no_predicate(_: &crate::DongoEntity) -> bool {
    true
}

impl std::fmt::Display for crate::DongoEntityManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        str.push_str("___Entities___\n");
        str.push_str("=================\n");
        str.push_str("Variant, id, desc\n");
        str.push_str("=================\n");
        for e in &self.e_vec {
            str.push_str(&format!("{}\n", e));
        }
        str.push_str("=================\n");
        write!(f, "{}", str)
    }   
}

impl std::fmt::Display for crate::DongoEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            crate::DongoEntity::Object(_,meta,id) =>{
                write!(f, "Object: {} {}",id.unwrap(), meta.desc())
            }
            crate::DongoEntity::Model(_,meta,id) => {
                write!(f, "model: {} {}",id.unwrap(), meta.desc())
            }
            //crate::DongoEntity::ColorModel(m) => write!(f, "{}", m),
        }
    }
}
