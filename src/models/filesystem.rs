use crate::models::directory::Directory;

/* Filesystem Class */
#[derive(Debug)]
pub struct FileSystem {
    pub root: Directory, // The root directory of the filesystem
    pub current_dir: Vec<String>, // Current directory of the filesystem
}

/* Filesystem method implementations */ 
impl FileSystem {
    // Constructor
    pub fn new() -> Self {
        FileSystem {
            root: Directory::new("root"),
            current_dir: vec!["root".to_string()],
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
}
