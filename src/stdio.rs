use std::io::*;

pub fn scan() -> String {
    // Declare a stdin and return variable
    let stdin = stdin();
    let mut input = String::new();
    
    // Read the stdin and send it to input
    stdin.read_line(&mut input)
        .expect("Failed to read line");
    
    // Trim whitespaces
    return input.trim().to_string(); // Convert &str to String and return
}

pub fn print(buffer : &str) {
    print!("{}", buffer);
    stdout().flush().unwrap();
}