pub type DONGOTAG = u8;

// DongoMetadata Tags
pub const TAG_SELECTABLE: DONGOTAG = 0;
pub const TAG_UI: DONGOTAG = 1;
pub const TAG_MAP: DONGOTAG = 2;
pub const TAG_SELECTION_BOX: DONGOTAG = 3;
pub const TAG_SELECTION_MARKER: DONGOTAG = 4;
pub const TAG_HAS_ANIMATION: DONGOTAG = 5;
pub const TAG_NO_LIGHT: DONGOTAG = 6;

pub struct DongoMetadata{
    pub desc: Option<String>,
    pub tags: Vec<DONGOTAG>,
}

impl DongoMetadata {    
    pub fn new_empty() -> DongoMetadata {
        DongoMetadata {
            desc: None,
            tags: Vec::new(),
        }
    }

    pub fn new(desc: Option<&str>, tags: Vec<DONGOTAG>) -> DongoMetadata {
        DongoMetadata {
            desc: match desc {
                Some(desc) => Some(desc.to_string()),
                None => None,
            },
            tags,
        }
    }

    pub fn desc(&self) -> &str {
        match &self.desc {
            Some(desc) => desc,
            None => "No description provided",
        }
    }
}

