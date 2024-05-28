use std::collections::HashMap;
use crate::models::directory::Directory;

/* Filesystem Class */
#[derive(Debug, Clone)]
pub struct FileSystem {
    pub root: Directory, // The root directory of the filesystem
    pub current_dir: Vec<String>, // Current directory of the filesystem
    pub mounts: HashMap<String, Directory>, // Add mounts to keep track of mounted filesystems
}

/* Filesystem method implementations */ 
impl FileSystem {
    // Constructor
    pub fn new() -> Self {
        FileSystem {
            root: Directory::new("#"),
            current_dir: vec!["#".to_string()],
            mounts: HashMap::new(), // Initialize mounts
        }
    }

    // Get current directory (unmutable)
    pub fn get_current_directory(&self) -> &Directory {
        let mut dir = &self.root;
        for part in &self.current_dir[1..] {
            dir = dir.directories.get(part).unwrap();
        }
        return dir;
    }

    // Get current directory (mutable)
    pub fn get_current_directory_mut(&mut self) -> &mut Directory {
        let mut dir = &mut self.root;
        for part in &self.current_dir[1..] {
            dir = dir.directories.get_mut(part).unwrap();
        }
        return dir;
    }

    // Copy a file to other directory
    pub fn copy_file(&mut self, src: &str, dest: &str) -> Result<(), String> {
        let dir = self.get_current_directory_mut();
        if let Some(file) = dir.files.get(src) {
            dir.files.insert(dest.to_string(), file.clone());
            Ok(())
        } else {
            Err(format!("[!] File '{}' not found", src))
        }
    }

    // Move a file to other directory
    pub fn rename_file(&mut self, src: &str, dest: &str) -> Result<(), String> {
        let dir = self.get_current_directory_mut();
        if let Some(file) = dir.files.remove(src) {
            dir.files.insert(dest.to_string(), file);
            Ok(())
        } else {
            Err(format!("[!] File '{}' not found", src))
        }
    }

    // Copy a directory to other directory
    pub fn copy_dir(&mut self, src: &str, dest: &str) -> Result<(), String> {
        let dir = self.get_current_directory_mut();
        if let Some(directory) = dir.directories.get(src) {
            dir.directories.insert(dest.to_string(), directory.clone());
            Ok(())
        } else {
            Err(format!("[!] Directory '{}' not found", src))
        }
    }

    // Move a directory to other directory
    pub fn rename_dir(&mut self, src: &str, dest: &str) -> Result<(), String> {
        let dir = self.get_current_directory_mut();
        if let Some(directory) = dir.directories.remove(src) {
            dir.directories.insert(dest.to_string(), directory);
            Ok(())
        } else {
            Err(format!("[!] Directory '{}' not found", src))
        }
    }
}
