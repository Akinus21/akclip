use std::env;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    
    // Read all stdin
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error reading stdin: {}", e);
        std::process::exit(1);
    }
    
    // Trim the input
    let text = input.trim();
    
    // Copy to clipboard using arboard
    match arboard::Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(text) {
                eprintln!("Error setting clipboard: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error accessing clipboard: {}", e);
            std::process::exit(1);
        }
    }
}