use crate::models::filesystem::FileSystem;
use crate::models::directory::Directory;

// Make a directory
pub fn mkdir(fs: &mut FileSystem, dir_name: &str) {
    let dir = fs.get_current_directory_mut();
    dir.directories.insert(dir_name.to_string(), Directory::new(dir_name));
}

// Remove a directory
pub fn rmdir(fs: &mut FileSystem, dir_name: &str) {
    let dir = fs.get_current_directory_mut();
    if dir.directories.remove(dir_name).is_none() {
        println!("[!] Directory not found");
    }
}
