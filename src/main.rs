use std::env;
use std::process::exit;

#[cfg(target_os = "macos")]
mod clipboard {
    use std::process::Command;
    
    pub fn get() -> Option<String> {
        let output = Command::new("pbpaste").output().ok()?;
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    pub fn set(text: &str) {
        let mut child = Command::new("pbcopy").stdin(std::process::Stdio::piped()).spawn().ok()?;
        if let Some(ref mut stdin) = child.stdin {
            use std::io::Write;
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
    }
}

#[cfg(target_os = "windows")]
mod clipboard {
    use std::process::Command;
    
    pub fn get() -> Option<String> {
        let output = Command::new("powershell")
            .args(["-command", "Get-Clipboard"])
            .output()
            .ok()?;
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    pub fn set(_text: &str) {
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    use std::process::{Command, Stdio};
    use std::io::Write;
    
    pub fn get() -> Option<String> {
        // Try xclip first, fall back to xsel
        let output = Command::new("xclip")
            .args(["-selection", "clipboard", "-o"])
            .output()
            .ok()?;
        
        if output.status.success() {
            return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
        
        // Fallback to xsel
        let output = Command::new("xsel")
            .args(["--clipboard", "--output"])
            .output()
            .ok()?;
        
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    pub fn set(text: &str) -> bool {
        // Try xclip first
        let mut child = Command::new("xclip")
            .args(["-selection", "clipboard"])
            .stdin(Stdio::piped())
            .spawn()
            .ok();
        
        if let Some(ref mut child) = child {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(text.as_bytes());
            }
            if child.wait().map(|s| s.success()).unwrap_or(false) {
                return true;
            }
        }
        
        // Fallback to xsel
        let mut child = Command::new("xsel")
            .args(["--clipboard", "--input"])
            .stdin(Stdio::piped())
            .spawn()
            .ok();
        
        if let Some(ref mut child) = child {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(text.as_bytes());
            }
            let _ = child.wait();
        }
        
        true
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    #[cfg(target_os = "macos")]
    {
        if args.len() > 1 && args[1] == "--help" {
            println!("Usage: akclip [OPTIONS]");
            println!("  -c, --copy    Copy stdin to clipboard");
            println!("  -p, --print   Print clipboard contents (default)");
            exit(0);
        }
        
        let action = if args.get(1).map(|s| s.as_str()) == Some("-c") || args.get(1).map(|s| s.as_str()) == Some("--copy") {
            "copy"
        } else {
            "print"
        };
        
        match action {
            "copy" => {
                let mut input = String::new();
                if std::io::stdin().read_line(&mut input).is_ok() {
                    clipboard::set(input.trim());
                }
            }
            "print" => {
                if let Some(content) = clipboard::get() {
                    println!("{}", content);
                }
            }
            _ => {}
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        if args.len() > 1 && args[1] == "--help" {
            println!("Usage: akclip [OPTIONS]");
            println!("  -c, --copy    Copy stdin to clipboard");
            println!("  -p, --print   Print clipboard contents (default)");
            exit(0);
        }
        
        let action = if args.get(1).map(|s| s.as_str()) == Some("-c") || args.get(1).map(|s| s.as_str()) == Some("--copy") {
            "copy"
        } else {
            "print"
        };
        
        match action {
            "copy" => {
                let mut input = String::new();
                if std::io::stdin().read_line(&mut input).is_ok() {
                    clipboard::set(input.trim());
                }
            }
            "print" => {
                if let Some(content) = clipboard::get() {
                    println!("{}", content);
                }
            }
            _ => {}
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        if args.len() > 1 && args[1] == "--help" {
            println!("Usage: akclip [OPTIONS]");
            println!("  -c, --copy    Copy stdin to clipboard");
            println!("  -p, --print   Print clipboard contents (default)");
            exit(0);
        }
        
        let action = if args.get(1).map(|s| s.as_str()) == Some("-c") || args.get(1).map(|s| s.as_str()) == Some("--copy") {
            "copy"
        } else {
            "print"
        };
        
        match action {
            "copy" => {
                let mut input = String::new();
                if std::io::stdin().read_line(&mut input).is_ok() {
                    clipboard::set(input.trim());
                }
            }
            "print" => {
                if let Some(content) = clipboard::get() {
                    println!("{}", content);
                }
            }
            _ => {}
        }
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        eprintln!("akclip is currently only supported on macOS, Windows, and Linux");
        exit(1);
    }
}