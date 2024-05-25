use crate::models::directory::*;

pub fn ls (workdir: Directory) {
    for (name, _) in workdir.list_directories(){
        println!("├── {} (dir)", name);
    }
    for (name, _) in workdir.list_files(){
        println!("├── {} (file)", name);
    }
}

pub fn pwd (workdir: Directory) {
    println!("{}", workdir.get_name());
}