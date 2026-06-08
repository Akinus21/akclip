use std::env;
use std::process::exit;

#[cfg(target_os = "macos")]
mod clipboard {
    use std::process::Command;

    pub fn read() -> Option<String> {
        Command::new("pbpaste")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
    }

    pub fn write(s: &str) -> Option<()> {
        Command::new("pbcopy")
            .arg("-p")
            .write(s.as_bytes())
            .ok()
            .map(|_| ())
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    use std::process::Command;

    pub fn read() -> Option<String> {
        Command::new("xclip")
            .args(&["-selection", "clipboard", "-o"])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
    }

    pub fn write(s: &str) -> Option<()> {
        let mut child = Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .ok()?;

        use std::io::Write;
        child.stdin.as_mut()?.write_all(s.as_bytes()).ok()?;
        child.wait().ok()?;
        Some(())
    }
}

#[cfg(target_os = "windows")]
mod clipboard {
    use std::process::Command;

    pub fn read() -> Option<String> {
        Command::new("powershell")
            .args(&["-Command", "Get-Clipboard"])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
    }

    pub fn write(s: &str) -> Option<()> {
        Command::new("powershell")
            .args(&["-Command", "Set-Clipboard", "-Value", s])
            .output()
            .ok()?;
        Some(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let text = args[1..].join(" ");
        if clipboard::write(&text).is_some() {
            println!("Copied to clipboard: {}", text);
        } else {
            eprintln!("Failed to copy to clipboard");
            exit(1);
        }
    } else {
        match clipboard::read() {
            Some(text) => {
                print!("{}", text.trim());
            }
            None => {
                eprintln!("Failed to read from clipboard");
                exit(1);
            }
        }
    }
}