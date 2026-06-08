use std::env;
use std::process::exit;

#[cfg(target_os = "macos")]
mod clipboard {
    use std::process::Command;

    pub fn get() -> Option<String> {
        let output = Command::new("pbpaste").output().ok()?;
        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            None
        }
    }

    pub fn set(text: &str) -> Option<()> {
        let mut cmd = Command::new("pbcopy");
        cmd.arg("-stdin").stdin(os_type::STDIN.clone()).ok()?;
        let output = cmd.output().ok()?;
        if output.status.success() {
            Some(())
        } else {
            None
        }
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    use arboard::Clipboard;
    use std::sync::Mutex;

    static CLIPBOARD: Mutex<Option<Clipboard>> = Mutex::new(None);

    pub fn get() -> Option<String> {
        let mut clipboard = CLIPBOARD.lock().ok()?;
        if clipboard.is_none() {
            *clipboard = Clipboard::new().ok();
        }
        if let Some(ref mut cb) = *clipboard {
            cb.get_text().ok()
        } else {
            None
        }
    }

    pub fn set(text: &str) -> Option<()> {
        let mut clipboard = CLIPBOARD.lock().ok()?;
        if clipboard.is_none() {
            *clipboard = Clipboard::new().ok();
        }
        if let Some(ref mut cb) = *clipboard {
            cb.set_text(text).ok()
        } else {
            None
        }
    }
}

#[cfg(target_os = "windows")]
mod clipboard {
    use std::process::Command;

    pub fn get() -> Option<String> {
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

    pub fn set(text: &str) -> Option<()> {
        let output = Command::new("powershell")
            .args(["-Command", &format!("Set-Clipboard -Value '{}'", text.replace("'", "''"))])
            .output()
            .ok()?;
        if output.status.success() {
            Some(())
        } else {
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: akclip [get|set <text>]");
        exit(1);
    }

    let subcommand = &args[1];

    match subcommand.as_str() {
        "get" => {
            match clipboard::get() {
                Some(text) => println!("{}", text),
                None => {
                    eprintln!("Failed to get clipboard content");
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
            match clipboard::set(text) {
                Some(()) => println!("Clipboard set successfully"),
                None => {
                    eprintln!("Failed to set clipboard content");
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unknown subcommand: {}", subcommand);
            eprintln!("Usage: akclip [get|set <text>]");
            exit(1);
        }
    }
}