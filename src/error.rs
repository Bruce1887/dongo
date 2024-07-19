pub trait ErrorMessage {
    fn error_message(&self) -> String {
        String::from("Error. No message defined.")
    }
}

#[derive(Debug)]
pub enum DongoError {
    MapGeneratorError(u8),
    UnknowError(u8),
}

impl ErrorMessage for DongoError {
    fn error_message(&self) -> String {
        match self {
            DongoError::MapGeneratorError(code) => match code {
                0 => String::from("Could not open mapfile"),
                1 => String::from("Could not parse contents of mapfile_1"),
                2 => String::from("MapGenerator field missing after parsing mapfile_1"),
                _ => String::from("Error: MapGeneratorError. Unknown code."),
            },
            DongoError::UnknowError(code) => String::from(format!("UnknowError. Code: {}", code)),
        }
    }
}
