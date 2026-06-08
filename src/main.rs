use std::env;
use std::process::exit;

#[cfg(target_os = "macos")]
mod clipboard {
    use std::process::Command;

    pub fn get() -> Option<String> {
        let output = Command::new("pbpaste")
            .output()
            .ok()?;
        String::from_utf8(output.stdout).ok()
    }

    pub fn set(text: &str) -> bool {
        Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                if let Some(ref mut stdin) = child.stdin {
                    stdin.write_all(text.as_bytes()).ok();
                }
                child.wait().ok();
                Ok(())
            })
            .is_ok()
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    use std::process::Command;

    pub fn get() -> Option<String> {
        let output = Command::new("xclip")
            .args(["-selection", "clipboard", "-o"])
            .output()
            .ok()?;
        String::from_utf8(output.stdout).ok()
    }

    pub fn set(text: &str) -> bool {
        Command::new("xclip")
            .args(["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                if let Some(ref mut stdin) = child.stdin {
                    stdin.write_all(text.as_bytes()).ok();
                }
                child.wait().ok();
                Ok(())
            })
            .is_ok()
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
        String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
    }

    pub fn set(text: &str) -> bool {
        Command::new("powershell")
            .args(["-command", &format!("Set-Clipboard -Value '{}'", text.replace("'", "''"))])
            .spawn()
            .is_ok()
    }
}

fn print_usage() {
    eprintln!("Usage: akclip [copy|get] [text]");
    eprintln!("  copy <text>  - Copy text to clipboard");
    eprintln!("  get          - Get text from clipboard");
    eprintln!("  (no args)    - Copy stdin to clipboard");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let mut text = String::new();
        use std::io::Read;
        if std::io::stdin().read_to_string(&mut text).is_ok() {
            if clipboard::set(text.trim()) {
                exit(0);
            }
        }
        eprintln!("Failed to read from stdin or set clipboard");
        exit(1);
    }

    match args[1].as_str() {
        "copy" => {
            if args.len() < 3 {
                eprintln!("Error: 'copy' requires text argument");
                print_usage();
                exit(1);
            }
            if clipboard::set(&args[2]) {
                exit(0);
            }
            eprintln!("Failed to set clipboard");
            exit(1);
        }
        "get" => {
            match clipboard::get() {
                Some(text) => {
                    println!("{}", text);
                    exit(0);
                }
                None => {
                    eprintln!("Failed to get clipboard content");
                    exit(1);
                }
            }
        }
        "-h" | "--help" | "help" => {
            print_usage();
            exit(0);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            exit(1);
        }
    }
}