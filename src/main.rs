use std::env;
use std::process::exit;

#[cfg(target_os = "macos")]
mod clipboard {
    use std::process::Command;

    pub fn get_clipboard() -> Option<String> {
        let output = Command::new("pbpaste").output().ok()?;
        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            None
        }
    }

    pub fn set_clipboard(text: &str) -> Result<(), String> {
        let mut child = Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        if let Some(ref mut stdin) = child.stdin {
            use std::io::Write;
            stdin.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
        }

        child.wait().map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    use std::process::Command;

    pub fn get_clipboard() -> Option<String> {
        let output = Command::new("xclip")
            .args(["-selection", "clipboard", "-o"])
            .output()
            .ok()?;
        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            None
        }
    }

    pub fn set_clipboard(text: &str) -> Result<(), String> {
        let mut child = Command::new("xclip")
            .args(["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        if let Some(ref mut stdin) = child.stdin {
            use std::io::Write;
            stdin.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
        }

        child.wait().map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod clipboard {
    use std::process::Command;

    pub fn get_clipboard() -> Option<String> {
        let output = Command::new("powershell")
            .args(["-Command", "Get-Clipboard"])
            .output()
            .ok()?;
        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            None
        }
    }

    pub fn set_clipboard(text: &str) -> Result<(), String> {
        let output = Command::new("powershell")
            .args(["-Command", &format!("Set-Clipboard -Value '{}'", text.replace("'", "''"))])
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: akclip <command> [text]");
        eprintln!("Commands:");
        eprintln!("  get          Get clipboard content");
        eprintln!("  set <text>   Set clipboard content");
        exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "get" => {
            match clipboard::get_clipboard() {
                Some(content) => println!("{}", content),
                None => {
                    eprintln!("Failed to read clipboard content");
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
            match clipboard::set_clipboard(text) {
                Ok(_) => println!("Clipboard set successfully"),
                Err(e) => {
                    eprintln!("Failed to set clipboard: {}", e);
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Usage: akclip <command> [text]");
            eprintln!("Commands:");
            eprintln!("  get          Get clipboard content");
            eprintln!("  set <text>   Set clipboard content");
            exit(1);
        }
    }
}