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

    // Initiate the folders of the root filesystem
    mkdir(&mut fs, "bin");
    mkdir(&mut fs, "boot");
    mkdir(&mut fs, "dev");
    mkdir(&mut fs, "etc");
    mkdir(&mut fs, "home");
    mkdir(&mut fs, "lib");
    mkdir(&mut fs, "media");
    mkdir(&mut fs, "mnt");
    mkdir(&mut fs, "opt");
    mkdir(&mut fs, "root");
    mkdir(&mut fs, "sbin");
    mkdir(&mut fs, "srv");
    mkdir(&mut fs, "temp");
    mkdir(&mut fs, "usr");
    mkdir(&mut fs, "var");

    // Initiate a partition
    let mut secret_partition = FileSystem::new();
    mkdir(&mut secret_partition, "DataPenting");
    cd(&mut secret_partition, "DataPenting");
    mkfile(&mut secret_partition, "RahasiaNegara.txt");
    write(&mut secret_partition, "RahasiaNegara.txt", "Ahmad Taufiq Hidayat Jadi Presiden?!?!?!?!");

    println!(r#"
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
            "cpfile" => {
                if args.len() == 2 {
                    cpfile(&mut fs, args[0], args[1]);
                } else {
                    println!("[!] Usage: cp <file> <newname>");
                }
            }
            "renmfile" => {
                if args.len() == 2 {
                    renmfile(&mut fs, args[0], args[1]);
                } else {
                    println!("[!] Usage: renm <file> <newname>");
                }
            }
            "cpdir" => {
                if args.len() == 2 {
                    cpdir(&mut fs, args[0], args[1]);
                } else {
                    println!("[!] Usage: cpdir <directory> <newname>");
                }
            }
            "renmdir" => {
                if args.len() == 2 {
                    renmdir(&mut fs, args[0], args[1]);
                } else {
                    println!("[!] Usage: renmdir <directory> <newname>");
                }
            }
            "mount" => {
                if args.len() == 1 {
                    mount(&mut fs, args[0], secret_partition.root.clone());
                } else {
                    println!("[!] Usage: mount <mountpoint>");
                }
            }
            "exit" => exit(),
            _ => println!("[!] Command not found"),
        }
    }
}
