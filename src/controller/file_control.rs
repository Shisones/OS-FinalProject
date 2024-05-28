use crate::models::filesystem::FileSystem;
use crate::models::file::File;
use colored::Colorize;

/* Controller for file  */

// Make a file in current directory 
pub fn mkfile(fs: &mut FileSystem, file_name: &str) {
    let dir = fs.get_current_directory_mut();
    dir.files.insert(file_name.to_string(), File::new(file_name));
}

// Read a file in current directory
pub fn read(fs: &FileSystem, file_name: &str) {
    let dir = fs.get_current_directory();
    if let Some(file) = dir.files.get(file_name) {
        println!("{}", file.content);
    } else {
        println!("{}", "[!] File not found".red());
    }
}

// Append to a file 
pub fn append(fs: &mut FileSystem, file_name: &str, content: &str) {
    let dir = fs.get_current_directory_mut();
    if let Some(file) = dir.files.get_mut(file_name) {
        file.content.push_str(content);
    } else {
        println!("{}", "[!] File not found".red());
    }
}

// Write to a file
pub fn write(fs: &mut FileSystem, file_name: &str, content: &str) {
    let dir = fs.get_current_directory_mut();
    if let Some(file) = dir.files.get_mut(file_name) {
        file.content = content.to_string();
    } else {
        println!("{}", "[!] File not found".red());
    }
}

// Remove a file
pub fn rmfile(fs: &mut FileSystem, file_name: &str) {
    let dir = fs.get_current_directory_mut();
    if dir.files.remove(file_name).is_none() {
        println!("{}", "[!] File not found".red());
    }
}

// Copy a file 
pub fn cpfile(fs: &mut FileSystem, src: &str, dest: &str) {
    if let Err(err) = fs.copy_file(src, dest) {
        println!("{}", err.red());
    }
}

// Rename a file
pub fn renmfile(fs: &mut FileSystem, src: &str, dest: &str) {
    if let Err(err) = fs.rename_file(src, dest) {
        println!("{}", err.red());
    }
}


