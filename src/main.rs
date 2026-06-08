use std::env;
use std::process::exit;

#[cfg(target_os = "macos")]
mod clipboard {
    use std::process::Command;

    pub fn get_clipboard() -> Result<String, String> {
        let output = Command::new("pbpaste")
            .output()
            .map_err(|e| format!("Failed to execute pbpaste: {}", e))?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err("Failed to get clipboard content".to_string())
        }
    }

    pub fn set_clipboard(text: &str) -> Result<(), String> {
        let mut process = Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to execute pbcopy: {}", e))?;

        use std::io::Write;
        if let Some(ref mut stdin) = process.stdin {
            stdin.write_all(text.as_bytes())
                .map_err(|e| format!("Failed to write to pbcopy: {}", e))?;
        }

        let status = process.wait()
            .map_err(|e| format!("Failed to wait for pbcopy: {}", e))?;

        if status.success() {
            Ok(())
        } else {
            Err("Failed to set clipboard content".to_string())
        }
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    use std::process::Command;

    pub fn get_clipboard() -> Result<String, String> {
        let output = Command::new("xclip")
            .args(&["-selection", "clipboard", "-o"])
            .output()
            .map_err(|e| format!("Failed to execute xclip: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let output = Command::new("xsel")
                .args(&["--clipboard", "--output"])
                .output()
                .map_err(|e| format!("Failed to execute xsel: {}", e))?;

            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err("Failed to get clipboard content".to_string())
            }
        }
    }

    pub fn set_clipboard(text: &str) -> Result<(), String> {
        let mut process = Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to execute xclip: {}", e))?;

        use std::io::Write;
        if let Some(ref mut stdin) = process.stdin {
            stdin.write_all(text.as_bytes())
                .map_err(|e| format!("Failed to write to xclip: {}", e))?;
        }

        let status = process.wait()
            .map_err(|e| format!("Failed to wait for xclip: {}", e))?;

        if status.success() {
            Ok(())
        } else {
            let mut process = Command::new("xsel")
                .args(&["--clipboard", "--input"])
                .stdin(std::process::Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to execute xsel: {}", e))?;

            if let Some(ref mut stdin) = process.stdin {
                stdin.write_all(text.as_bytes())
                    .map_err(|e| format!("Failed to write to xsel: {}", e))?;
            }

            let status = process.wait()
                .map_err(|e| format!("Failed to wait for xsel: {}", e))?;

            if status.success() {
                Ok(())
            } else {
                Err("Failed to set clipboard content".to_string())
            }
        }
    }
}

#[cfg(target_os = "windows")]
mod clipboard {
    use std::process::Command;

    pub fn get_clipboard() -> Result<String, String> {
        let output = Command::new("powershell")
            .args(&["-Command", "Get-Clipboard"])
            .output()
            .map_err(|e| format!("Failed to execute Get-Clipboard: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err("Failed to get clipboard content".to_string())
        }
    }

    pub fn set_clipboard(text: &str) -> Result<(), String> {
        let output = Command::new("powershell")
            .args(&["-Command", &format!("Set-Clipboard -Value '{}'", text.replace("'", "''"))])
            .output()
            .map_err(|e| format!("Failed to execute Set-Clipboard: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err("Failed to set clipboard content".to_string())
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
mod clipboard {
    pub fn get_clipboard() -> Result<String, String> {
        Err("Unsupported platform".to_string())
    }

    pub fn set_clipboard(_text: &str) -> Result<(), String> {
        Err("Unsupported platform".to_string())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: akclip [get|set] [text]");
        println!("  get - Get clipboard content");
        println!("  set - Set clipboard content");
        exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "get" => {
            match clipboard::get_clipboard() {
                Ok(content) => {
                    print!("{}", content);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        "set" => {
            if args.len() < 3 {
                eprintln!("Error: 'set' command requires text argument");
                exit(1);
            }
            let text = &args[2];
            match clipboard::set_clipboard(text) {
                Ok(()) => {
                    println!("Clipboard set successfully");
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("Error: Unknown command '{}'. Use 'get' or 'set'", command);
            exit(1);
        }
    }
}