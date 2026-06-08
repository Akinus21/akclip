use std::env;
use std::process::exit;
use std::error::Error;

#[cfg(target_os = "macos")]
extern crate clipboard;

#[cfg(not(target_os = "macos"))]
use arboard::Clipboard;

#[cfg(target_os = "macos")]
use clipboard::Clipboard;

#[derive(Clone)]
pub struct ClipboardManager {
    clipboard: Clipboard,
}

impl ClipboardManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let clipboard = Clipboard::new()?;
        Ok(ClipboardManager { clipboard })
    }

    pub fn get_content(&mut self) -> Result<String, Box<dyn Error>> {
        let content = self.clipboard.get_text()?;
        Ok(content)
    }

    pub fn set_content(&mut self, content: &str) -> Result<(), Box<dyn Error>> {
        self.clipboard.set_text(content)?;
        Ok(())
    }
}

fn get_clipboard_content() -> Result<String, Box<dyn Error>> {
    let mut manager = ClipboardManager::new()?;
    manager.get_content()
}

fn set_clipboard_content(content: &str) -> Result<(), Box<dyn Error>> {
    let mut manager = ClipboardManager::new()?;
    manager.set_content(content)
}

fn read_env_var(name: &str) -> Option<String> {
    env::var(name).ok()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: akclip <get|set> [content]");
        exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "get" => {
            match get_clipboard_content() {
                Ok(content) => println!("{}", content),
                Err(e) => {
                    eprintln!("Error reading clipboard: {}", e);
                    exit(1);
                }
            }
        }
        "set" => {
            if args.len() < 3 {
                eprintln!("Usage: akclip set <content>");
                exit(1);
            }
            let content = &args[2];
            match set_clipboard_content(content) {
                Ok(()) => println!("Clipboard set successfully"),
                Err(e) => {
                    eprintln!("Error setting clipboard: {}", e);
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}. Use 'get' or 'set'", command);
            exit(1);
        }
    }
}