use std::collections::HashMap;
use crate::models::file::File;

#[derive(Debug, Clone)]
pub struct Directory {
    name: String,
    files: HashMap<String, File>,
    directories: HashMap<String, Directory>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &String { &self.name }
    pub fn set_name(&mut self, name: &str) { self.name = name.to_string(); }

    pub fn get_file(&self, name: &str) -> Option<&File> { self.files.get(name) }
    pub fn get_file_mut(&mut self, name: &str) -> Option<&mut File> { self.files.get_mut(name) }

    pub fn get_directory(&self, name: &str) -> Option<&Directory> { self.directories.get(name) }
    pub fn get_directory_mut(&mut self, name: &str) -> Option<&mut Directory> { self.directories.get_mut(name) }

    pub fn list_files(&self) -> impl Iterator<Item = (&String, &File)> { self.files.iter() }
    pub fn list_directories(&self) -> impl Iterator<Item = (&String, &Directory)> { self.directories.iter() }

    pub fn create_file(&mut self, name: &str) {
        self.files.insert(name.to_string(), File::new(name));
    }

    pub fn create_directory(&mut self, name: &str) {
        let new_dir = Directory::new(name); 
        self.directories.insert(name.to_string(), new_dir);
    }

    pub fn remove_file(&mut self, name: &str) -> Option<File> {
        self.files.remove(name)
    }

    pub fn remove_directory(&mut self, name: &str) -> Option<Directory> {
        self.directories.remove(name)
    }

    pub fn file_exists(&self, name: &str) -> bool {
        self.files.contains_key(name)
    }

    pub fn dir_exists(&self, name: &str) -> bool {
        self.directories.contains_key(name)
    }
}
