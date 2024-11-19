#[inline]
pub const fn no_predicate(_: &crate::DongoEntity) -> bool {
    true
}

impl std::fmt::Display for crate::DongoEntityManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        str.push_str("_____Entities_____\n==================\nVariant, id, desc \n==================\n");
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
                write!(f, "Object: {} {} {:?}",id.unwrap(), meta.desc(),self.pos())
            }
            crate::DongoEntity::Model(_,meta,id) => {
                write!(f, "Model: {} {} {:?}",id.unwrap(), meta.desc(),self.pos())
            }
            crate::DongoEntity::Terrain(_,meta,id,_) => {
                write!(f, "Terrain: {} {}",id.unwrap(), meta.desc())
            }
            //crate::DongoEntity::ColorModel(m) => write!(f, "{}", m),
        }
    }
}
