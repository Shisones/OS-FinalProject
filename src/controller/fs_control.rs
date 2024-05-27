use crate::models::filesystem::FileSystem;

// Print working directory
pub fn pwd(fs: &FileSystem) {
    println!("{}", fs.current_dir.join("/"));
}

// List the directory contents
pub fn ls(fs: &FileSystem) {
    let dir = fs.get_current_directory();
    for (name, _) in &dir.directories {
        println!("{} (d)\n", name);
    }
    for (name, _) in &dir.files {
        println!("{} (f)\n", name);
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

// Exit the program
pub fn exit() {
    println!("Exiting...");
    std::process::exit(0);
}