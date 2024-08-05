use super::DONGOTAG;

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