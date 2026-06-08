use std::env;
use std::process::exit;
use std::error::Error;

#[cfg(target_os = "macos")]
mod clipboard {
    use arboard::Clipboard;
    use std::sync::Mutex;

    pub struct ClipboardManager {
        clipboard: Mutex<Clipboard>,
    }

    impl ClipboardManager {
        pub fn new() -> Result<Self, Box<dyn Error>> {
            let clipboard = Clipboard::new()?;
            Ok(Self {
                clipboard: Mutex::new(clipboard),
            })
        }

        pub fn get_content(&self) -> Result<String, Box<dyn Error>> {
            let mut clipboard = self.clipboard.lock().unwrap();
            match clipboard.get_text() {
                Ok(text) => Ok(text),
                Err(_) => Ok(String::new()),
            }
        }

        pub fn set_content(&self, content: &str) -> Result<(), Box<dyn Error>> {
            let mut clipboard = self.clipboard.lock().unwrap();
            clipboard.set_text(content)?;
            Ok(())
        }
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    use arboard::Clipboard;
    use std::sync::Mutex;

    pub struct ClipboardManager {
        clipboard: Mutex<Clipboard>,
    }

    impl ClipboardManager {
        pub fn new() -> Result<Self, Box<dyn Error>> {
            let clipboard = Clipboard::new()?;
            Ok(Self {
                clipboard: Mutex::new(clipboard),
            })
        }

        pub fn get_content(&self) -> Result<String, Box<dyn Error>> {
            let mut clipboard = self.clipboard.lock().unwrap();
            match clipboard.get_text() {
                Ok(text) => Ok(text),
                Err(_) => Ok(String::new()),
            }
        }

        pub fn set_content(&self, content: &str) -> Result<(), Box<dyn Error>> {
            let mut clipboard = self.clipboard.lock().unwrap();
            clipboard.set_text(content)?;
            Ok(())
        }
    }
}

#[cfg(target_os = "windows")]
mod clipboard {
    use arboard::Clipboard;
    use std::sync::Mutex;

    pub struct ClipboardManager {
        clipboard: Mutex<Clipboard>,
    }

    impl ClipboardManager {
        pub fn new() -> Result<Self, Box<dyn Error>> {
            let clipboard = Clipboard::new()?;
            Ok(Self {
                clipboard: Mutex::new(clipboard),
            })
        }

        pub fn get_content(&self) -> Result<String, Box<dyn Error>> {
            let mut clipboard = self.clipboard.lock().unwrap();
            match clipboard.get_text() {
                Ok(text) => Ok(text),
                Err(_) => Ok(String::new()),
            }
        }

        pub fn set_content(&self, content: &str) -> Result<(), Box<dyn Error>> {
            let mut clipboard = self.clipboard.lock().unwrap();
            clipboard.set_text(content)?;
            Ok(())
        }
    }
}

fn get_clipboard_content() -> Result<String, Box<dyn Error>> {
    let manager = clipboard::ClipboardManager::new()?;
    manager.get_content()
}

fn set_clipboard_content(content: &str) -> Result<(), Box<dyn Error>> {
    let manager = clipboard::ClipboardManager::new()?;
    manager.set_content(content)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: akclip <text> or akclip --get");
        exit(1);
    }

    let command = &args[1];

    if command == "--get" {
        match get_clipboard_content() {
            Ok(content) => {
                if !content.is_empty() {
                    println!("{}", content);
                }
            }
            Err(e) => {
                eprintln!("Error getting clipboard: {}", e);
                exit(1);
            }
        }
    } else {
        let text = args[1..].join(" ");
        match set_clipboard_content(&text) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error setting clipboard: {}", e);
                exit(1);
            }
        }
    }
}