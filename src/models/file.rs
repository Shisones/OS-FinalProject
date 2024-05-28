/* File Class */
#[derive(Debug, Clone)]
pub struct File {
    pub name: String, // Name of the file
    pub content: String, // Content of the file
}

/* File method implementations */
impl File {
    // Constructor
    pub fn new(name: &str) -> Self {
        File {
            name: name.to_string(),
            content: String::new(),
        }
    }
}
