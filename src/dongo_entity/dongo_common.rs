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

pub const fn no_predicate(_: &crate::DongoEntity) -> bool {
    true
}