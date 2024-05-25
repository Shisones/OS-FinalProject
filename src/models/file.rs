// File class implementation

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    content: String,
}

// Methods for file
impl File {
    // Create a new file
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            content: String::new(),
        }
    }

    // Write to file (append)
    pub fn append(&mut self, content: &str) {
        self.content.push_str(content);
    }
    
    pub fn write(&mut self, content: &str) {
        self.content.clear();
        self.content.push_str(content);
    }

    // Read file content
    pub fn read(&self) -> &str {
        &self.content
    }
}
