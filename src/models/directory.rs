use std::collections::HashMap;
use crate::models::file::File;

/* Directory Class */
#[derive(Debug)]
pub struct Directory {
    pub name: String, // Directory name
    pub files: HashMap<String, File>, // Files inside the directory
    pub directories: HashMap<String, Directory>, // Directories inside the directpry
}

/* Directory method implementations */
impl Directory {
    // Constructor
    pub fn new(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }
}
