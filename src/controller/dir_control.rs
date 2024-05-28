use crate::models::filesystem::FileSystem;
use crate::models::directory::Directory;
use colored::Colorize;

// Make a directory
pub fn mkdir(fs: &mut FileSystem, dir_name: &str) {
    let dir = fs.get_current_directory_mut();
    dir.directories.insert(dir_name.to_string(), Directory::new(dir_name));
}

// Remove a directory
pub fn rmdir(fs: &mut FileSystem, dir_name: &str) {
    let dir = fs.get_current_directory_mut();
    if dir.directories.remove(dir_name).is_none() {
        println!("{}", "[!] Directory not found".red());
    }
}

// Copy a directory
pub fn cpdir(fs: &mut FileSystem, src: &str, dest: &str) {
    if let Err(err) = fs.copy_dir(src, dest) {
        println!("{}", err);
    }
}

// Rename a directory
pub fn renmdir(fs: &mut FileSystem, src: &str, dest: &str) {
    if let Err(err) = fs.rename_dir(src, dest) {
        println!("{}", err);
    }
}
