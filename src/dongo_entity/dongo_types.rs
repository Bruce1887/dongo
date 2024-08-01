pub type PLAYERID = u8;
pub type ENTITYID = u16;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum DongoEntityType {
    Selectable { entity: SelectableEntity },
    NonSelectable { entity: NonSelectableEntity },
    UI,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SelectableEntity {
    PlayerEntity(PLAYERID),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NonSelectableEntity {
    WorldTerrain,
    WorldEntity,
    SelectionMarker(ENTITYID),
    SelectionBox,
}
