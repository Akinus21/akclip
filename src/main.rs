use std::env;
use std::process::exit;
use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(target_os = "macos")]
use std::process::Stdio as StdIO;

#[cfg(target_os = "macos")]
fn get_clipboard() -> Result<String, String> {
    let output = Command::new("pbpaste")
        .output()
        .map_err(|e| format!("Failed to execute pbpaste: {}", e))?;
    if !output.status.success() {
        return Err("pbpaste command failed".to_string());
    }
    String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to parse clipboard content: {}", e))
}

#[cfg(target_os = "macos")]
fn set_clipboard(content: &str) -> Result<(), String> {
    let mut echo = Command::new("pbcopy");
    echo.stdin(Stdio::piped())
        .arg(content)
        .output()
        .map_err(|e| format!("Failed to execute pbcopy: {}", e))?;
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn get_clipboard() -> Result<String, String> {
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| format!("Failed to access clipboard: {}", e))?;
    clipboard.get_text()
        .map_err(|e| format!("Failed to get clipboard text: {}", e))
}

#[cfg(not(target_os = "macos"))]
fn set_clipboard(content: &str) -> Result<(), String> {
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| format!("Failed to access clipboard: {}", e))?;
    clipboard.set_text(content)
        .map_err(|e| format!("Failed to set clipboard text: {}", e))?;
    Ok(())
}

fn parse_args() -> (bool, bool, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let mut clear = false;
    let mut image = false;
    let mut text_args: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-c" | "--clear" => {
                clear = true;
            }
            "-i" | "--image" => {
                image = true;
            }
            "-h" | "--help" => {
                println!("Usage: akclip [OPTIONS] [TEXT]");
                println!("");
                println!("Options:");
                println!("  -c, --clear    Clear the clipboard");
                println!("  -i, --image    Handle image data");
                println!("  -h, --help     Show this help message");
                exit(0);
            }
            _ => {
                text_args.push(args[i].clone());
            }
        }
        i += 1;
    }

    (clear, image, text_args)
}

fn main() {
    let (clear, _image, text_args) = parse_args();

    if clear {
        if let Err(e) = set_clipboard("") {
            eprintln!("Error clearing clipboard: {}", e);
            exit(1);
        }
        println!("Clipboard cleared");
        return;
    }

    if !text_args.is_empty() {
        let text = text_args.join(" ");
        if let Err(e) = set_clipboard(&text) {
            eprintln!("Error setting clipboard: {}", e);
            exit(1);
        }
        println!("Set clipboard to: {}", text);
        return;
    }

    match get_clipboard() {
        Ok(content) => {
            println!("{}", content);
        }
        Err(e) => {
            eprintln!("Error reading clipboard: {}", e);
            exit(1);
        }
    }
}