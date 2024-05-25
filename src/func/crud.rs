use crate::models::directory::Directory;
use crate::models::file::File;

use crate::stdio::*;

pub fn mkfile(workdir: &mut Directory){
    print("[i] Make file : ");
    let filename = scan();
    workdir.create_file(filename.as_str());
    
}
pub fn rmfile(workdir: &mut Directory){
    print("[i] Remove file : ");
    let filename = scan();
    if workdir.file_exists(filename.as_str()) {
        workdir.remove_file(filename.as_str());
    }
}
pub fn mkdir(workdir: &mut Directory){
    print("[i] Make directory : ");
    let dirname = scan();
    workdir.create_directory(dirname.as_str());
}

pub fn rmdir(workdir: &mut Directory){
    print("[i] Remove directory : ");
    let dirname = scan();

    if workdir.dir_exists(dirname.as_str()) {
        workdir.remove_directory(dirname.as_str());
    }
}

pub fn readfile(workdir: &mut Directory){
    print("[i] Read file : ");
    let filename = scan();

    if workdir.file_exists(filename.as_str()) {
        let file = workdir.get_file(filename.as_str()).unwrap();

        println!("{}", file.read())
    }
}

pub fn writefile(workdir: &mut Directory){
    print("[i] Write to file : ");
    let filename = scan();

    if workdir.file_exists(filename.as_str()) {
        let file = workdir.get_file_mut(filename.as_str()).unwrap();
        
        print("[i] Enter something to write\n");
        let content = scan();

        file.write(content.as_str());
    }
}

pub fn appendfile(workdir: &mut Directory){
    print("[i] Append to file : ");
    let filename = scan();

    if workdir.file_exists(filename.as_str()) {
        let file = workdir.get_file_mut(filename.as_str()).unwrap();

        print("Enter something to write\n");
        let content = scan();

        file.write(content.as_str());
    }
}