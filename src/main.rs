use std::env;
use std::process::exit;

#[cfg(target_os = "macos")]
mod clipboard {
    use arboard::Clipboard;

    pub fn get_clipboard_text() -> Result<String, String> {
        let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
        match clipboard.get_text() {
            Ok(text) => Ok(text),
            Err(e) => Err(format!("Failed to get clipboard text: {}", e)),
        }
    }

    pub fn set_clipboard_text(text: &str) -> Result<(), String> {
        let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
        clipboard.set_text(text).map_err(|e| e.to_string())
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    use arboard::Clipboard;

    pub fn get_clipboard_text() -> Result<String, String> {
        let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
        match clipboard.get_text() {
            Ok(text) => Ok(text),
            Err(e) => Err(format!("Failed to get clipboard text: {}", e)),
        }
    }

    pub fn set_clipboard_text(text: &str) -> Result<(), String> {
        let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
        clipboard.set_text(text).map_err(|e| e.to_string())
    }
}

#[cfg(target_os = "windows")]
mod clipboard {
    use arboard::Clipboard;

    pub fn get_clipboard_text() -> Result<String, String> {
        let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
        match clipboard.get_text() {
            Ok(text) => Ok(text),
            Err(e) => Err(format!("Failed to get clipboard text: {}", e)),
        }
    }

    pub fn set_clipboard_text(text: &str) -> Result<(), String> {
        let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
        clipboard.set_text(text).map_err(|e| e.to_string())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: akclip <command> [text]");
        eprintln!("Commands:");
        eprintln!("  get         Get clipboard text");
        eprintln!("  set <text>  Set clipboard text");
        exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "get" => {
            match clipboard::get_clipboard_text() {
                Ok(text) => println!("{}", text),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        "set" => {
            if args.len() < 3 {
                eprintln!("Usage: akclip set <text>");
                exit(1);
            }
            let text = &args[2];
            match clipboard::set_clipboard_text(text) {
                Ok(()) => println!("Clipboard set successfully"),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Usage: akclip <command> [text]");
            eprintln!("Commands:");
            eprintln!("  get         Get clipboard text");
            eprintln!("  set <text>  Set clipboard text");
            exit(1);
        }
    }
}