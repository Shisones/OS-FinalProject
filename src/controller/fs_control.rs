use crate::models::{
    filesystem::FileSystem,
    directory::Directory
};

// Print working directory
pub fn pwd(fs: &FileSystem) {
    println!("{}", fs.current_dir.join("/"));
}

// List the directory contents
pub fn ls(fs: &FileSystem) {
    let dir = fs.get_current_directory();
    for (name, _) in &dir.directories {
        println!("{} (d)", name);
    }
    for (name, _) in &dir.files {
        println!("{} (f)", name);
    }
    for (mount_point, _) in &fs.mounts {
        println!("{} (m)", mount_point);
    }
}

// Change current directory
pub fn cd(fs: &mut FileSystem, dir_name: &str) {
    if dir_name == ".." {
        if fs.current_dir.len() > 1 {
            fs.current_dir.pop();
        }
    } else {
        let dir = fs.get_current_directory();
        if dir.directories.contains_key(dir_name) {
            fs.current_dir.push(dir_name.to_string());
        } else {
            println!("[!] Directory not found");
        }
    }
}

// Copy a file 
pub fn cpfile(fs: &mut FileSystem, src: &str, dest: &str) {
    if let Err(err) = fs.copy_file(src, dest) {
        println!("{}", err);
    }
}

// Rename a file
pub fn renmfile(fs: &mut FileSystem, src: &str, dest: &str) {
    if let Err(err) = fs.rename_file(src, dest) {
        println!("{}", err);
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

pub fn mount(fs: &mut FileSystem, mount_point: &str, fs_to_mount: Directory) {
    let dir = fs.get_current_directory_mut();
    if dir.directories.contains_key(mount_point) {
        println!("[!] Mount point already exists");
    } else {
        dir.directories.insert(mount_point.to_string(), fs_to_mount);
        println!("[*] Filesystem mounted at {}", mount_point);
    }
}

// Exit the program
pub fn exit() {
    println!("Exiting...");
    std::process::exit(0);
}