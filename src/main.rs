use std::env;
use std::process::exit;
use std::error::Error;

#[cfg(target_os = "macos")]
use pbpaste;

#[cfg(not(target_os = "macos"))]
use arboard::Clipboard;

#[derive(Debug)]
pub struct ClipboardManager {
    #[cfg(target_os = "macos")]
    content: String,
    #[cfg(not(target_os = "macos"))]
    clipboard: Clipboard,
}

#[cfg(target_os = "macos")]
impl ClipboardManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(ClipboardManager {
            content: String::new(),
        })
    }

    pub fn get_content(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.content.clone())
    }

    pub fn set_content(&self, content: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn update_from_system(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[cfg(not(target_os = "macos"))]
impl ClipboardManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let clipboard = Clipboard::new()?;
        Ok(ClipboardManager { clipboard })
    }

    pub fn get_content(&self) -> Result<String, Box<dyn Error>> {
        let content = self.clipboard.get_text()?;
        Ok(content)
    }

    pub fn set_content(&self, content: &str) -> Result<(), Box<dyn Error>> {
        self.clipboard.set_text(content)?;
        Ok(())
    }

    pub fn update_from_system(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[cfg(target_os = "macos")]
fn get_clipboard_content() -> Result<String, Box<dyn Error>> {
    use std::process::Command;
    let output = Command::new("pbpaste").output()?;
    let content = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(content)
}

#[cfg(target_os = "macos")]
fn set_clipboard_content(content: &str) -> Result<(), Box<dyn Error>> {
    use std::process::Command;
    let mut child = Command::new("pbcopy").stdin(std::process::Stdio::piped()).spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin.write_all(content.as_bytes())?;
    }
    child.wait()?;
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn get_clipboard_content() -> Result<String, Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;
    let content = clipboard.get_text()?;
    Ok(content)
}

#[cfg(not(target_os = "macos"))]
fn set_clipboard_content(content: &str) -> Result<(), Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(content)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: akclip [get|set] [content]");
        exit(1);
    }

    let action = &args[1];

    match action.as_str() {
        "get" => {
            match get_clipboard_content() {
                Ok(content) => {
                    println!("{}", content);
                }
                Err(e) => {
                    eprintln!("Error reading clipboard: {}", e);
                    exit(1);
                }
            }
        }
        "set" => {
            if args.len() < 3 {
                eprintln!("Usage: akclip set [content]");
                exit(1);
            }
            let content = &args[2];
            match set_clipboard_content(content) {
                Ok(()) => {
                    println!("Clipboard set successfully");
                }
                Err(e) => {
                    eprintln!("Error setting clipboard: {}", e);
                    exit(1);
                }
            }
        }
        "watch" => {
            println!("Watching clipboard changes...");
            let mut last_content = String::new();
            loop {
                match get_clipboard_content() {
                    Ok(content) => {
                        if content != last_content {
                            println!("Clipboard changed: {}", content);
                            last_content = content;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading clipboard: {}", e);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
        _ => {
            eprintln!("Invalid action: {}", action);
            eprintln!("Usage: akclip [get|set|watch] [content]");
            exit(1);
        }
    }
}