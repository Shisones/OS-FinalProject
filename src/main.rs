// Rust package modules
mod stdio; // For print() and scan() macros
mod models; // Model classes
mod controller; // Controller functions

// Usage of modules
use stdio::*;
use models::filesystem::FileSystem;
use controller::{
    file_control::*,
    dir_control::*,
    fs_control::*
};

// Driver Code
fn main() {
    // Preparatory variables and instantiation of Filesystem
    let mut fs = FileSystem::new();
    let run = true;

    println!(
        r#"
           ╭──────────────────────────────────────╮
           │        HatchmakerOS v.0.1.0          │
           │ Made for Operating System Final Task │
           │ Copyright :                          │
           │    - Jason Rafif P.S. (2204524)      │
           │    - Defrizal Yahdiyan R. (2206131)  │
           ╰──────────────────────────────────────╯
           "#
    );
    // Main loop
    while run {
        // Print $PS1 variable and scan user input
        print("RasimOS $ ");
        let input = scan();

        // Parse user input to command and arguments
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];

        // Application command handler
        match command {
            "pwd" => { pwd(&fs)} , // Print working directory
            "ls" => { ls(&fs)} , // List the contents of current directory
            "cd" => { // Change directory
                if args.len() == 1 { cd(&mut fs, args[0]); } 
                else { println!("[!] Usage: cd <directory>"); }
            }
            "mkfile" => { // Make a file inside current directory
                if args.len() == 1 { mkfile(&mut fs, args[0]); }
                else { println!("[!] Usage: mkfile <filename>"); }
            }
            "read" => { // Read a file inside current directory
                if args.len() == 1 { read(&fs, args[0]); }
                else { println!("[!] Usage: read <filename>"); }
            }
            "append" => { // Append to a file inside current directory
                if args.len() >= 2 { append(&mut fs, args[0], &args[1..].join(" ")); }
                else { println!("[!] Usage: append <filename> <content>"); }
            }
            "write" => { // Write to a file inside current directory
                if args.len() >= 2 { write(&mut fs, args[0], &args[1..].join(" ")); }
                else { println!("[!] Usage: write <filename> <content>"); }
            }
            "rmfile" => { // Remove a file
                if args.len() == 1 { rmfile(&mut fs, args[0]); } 
                else { println!("[!] Usage: rmfile <filename>"); }
            }
            "mkdir" => { // Make a subdirectory
                if args.len() == 1 { mkdir(&mut fs, args[0]); } 
                else { println!("[!] Usage: mkdir <dirname>"); }
            }
            "rmdir" => { // Remove a subdirectory
                if args.len() == 1 { rmdir(&mut fs, args[0]); } 
                else { println!("[!] Usage: rmdir <dirname>"); }
            }
            "exit" => exit(),
            _ => println!("[!] Command not found"),
        }
    }
}
