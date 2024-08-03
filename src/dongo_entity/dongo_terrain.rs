use crate::*;
use three_d::*;

pub struct DongoTerrain {
    pub(crate) id: Option<ENTITYID>,
    pub(crate) desc: Option<String>,
    pub(crate) dongo_obj : DongoObject,
    pub(crate) e_type: DongoEntityType,
}


