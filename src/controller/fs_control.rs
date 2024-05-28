use colored::Colorize;
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
        println!("{}", name.blue());
    }
    for (name, _) in &dir.files {
        println!("{}", name);
    }
    for (mount_point, _) in &fs.mounts {
        println!("{}", mount_point.cyan());
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
            println!("{}", "[!] Directory not found".red());
        }
    }
}

// Mount another fs
pub fn mount(fs: &mut FileSystem, mount_point: &str, fs_to_mount: Directory) {
    let dir = fs.get_current_directory_mut();
    if dir.directories.contains_key(mount_point) {
        println!("{}", "[!] Mount point already exists".red());
    } else {
        dir.directories.insert(mount_point.to_string(), fs_to_mount);
        println!("{} {}", "[*] Filesystem mounted at".yellow(), mount_point.yellow());
    }
}

// Exit the program
pub fn exit() {
    println!("{}", "[*] Exiting...".yellow());
    std::process::exit(0);
}