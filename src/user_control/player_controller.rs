use crate::*;

pub type PlayerId = u32;
pub struct DongoPlayer {
    id: PlayerId,
    name: String,
    player_entity_id: ENTITYID,
    // idk like inventory or and so on
}

impl DongoPlayer {
    pub fn new(id: PlayerId, name: String, player_entity_id: ENTITYID) -> DongoPlayer {
        DongoPlayer {
            id,
            name,
            player_entity_id,
        }
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn player_entity_id(&self) -> ENTITYID {
        self.player_entity_id
    }
}