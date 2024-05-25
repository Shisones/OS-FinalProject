// src/main.rs
mod stdio;
mod models;
mod func;

use stdio::*;
use models::directory::Directory;
use func::{ 
    info::*, 
    crud::* 
};

fn main() {
    let mut run = true;
    let mut workdir = Directory::new("root");
    while run == true {
        print("RasimOS $ ");
        let input = scan();

        match input.as_str() {
            "pwd" => { pwd(workdir.clone())},            
            "ls" => { ls(workdir.clone()) },
            "cd" => {},
            // copy
            // move
            
            "mkfile" => { mkfile(&mut workdir) },
            "read" => { readfile(&mut workdir) } 
            "append" => { appendfile(&mut workdir) } 
            "write" => { writefile(&mut workdir) }
            "rmfile" => { rmfile(&mut workdir) },

            "mkdir" => { mkdir(&mut workdir) },
            "rmdir" => { rmdir(&mut workdir) },
            "exit" => { run = false },
            _ => { print("[!] Command not found\n")}
        }
    }
    
}
