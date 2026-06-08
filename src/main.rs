use std::env;
use std::process::exit;
use std::io::Read;

#[cfg(target_os = "macos")]
mod clipboard {
    use std::process::{Command, Stdio};

    pub fn get() -> Option<String> {
        let output = Command::new("pbpaste")
            .output()
            .ok()?;
        Some(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    pub fn set(text: &str) -> bool {
        let mut echo = Command::new("echo")
            .arg(text.trim())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn echo command");

        if let Some(ref mut stdout) = echo.stdout {
            let mut input = Vec::new();
            stdout.read_to_end(&mut input).ok();

            let result = Command::new("pbcopy")
                .stdin(Stdio::piped())
                .spawn();

            if let Ok(mut pbcopy) = result {
                if let Some(ref mut stdin) = pbcopy.stdin {
                    use std::io::Write;
                    stdin.write_all(&input).is_ok()
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    use std::process::{Command, Stdio};
    use std::io::Read;

    pub fn get() -> Option<String> {
        let mut cmd = Command::new("xclip")
            .args(["-selection", "clipboard", "-o"])
            .stdout(Stdio::piped())
            .spawn()
            .ok()?;

        let mut output = String::new();
        cmd.stdout.take().map(|mut s| s.read_to_string(&mut output).ok());
        Some(output)
    }

    pub fn set(text: &str) -> bool {
        let mut child = Command::new("xclip")
            .args(["-selection", "clipboard", "-i"])
            .stdin(Stdio::piped())
            .spawn()
            .ok();

        if let Some(ref mut p) = child {
            if let Some(ref mut stdin) = p.stdin {
                use std::io::Write;
                stdin.write_all(text.as_bytes()).is_ok()
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(target_os = "windows")]
mod clipboard {
    use std::process::{Command, Stdio};

    pub fn get() -> Option<String> {
        let output = Command::new("powershell")
            .args(["-command", "Get-Clipboard"])
            .stdout(Stdio::piped())
            .output()
            .ok()?;

        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    pub fn set(text: &str) -> bool {
        let mut child = Command::new("powershell")
            .args(["-command", "Set-Clipboard", "-Value"])
            .arg(text)
            .stdin(Stdio::piped())
            .spawn()
            .ok();

        child.map(|mut c| c.wait().is_ok()).unwrap_or(false)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        match clipboard::get() {
            Some(content) => println!("{}", content),
            None => {
                eprintln!("Failed to read clipboard content");
                exit(1);
            }
        }
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_str() {
        "-g" | "--get" => {
            match clipboard::get() {
                Some(content) => println!("{}", content),
                None => {
                    eprintln!("Failed to read clipboard content");
                    exit(1);
                }
            }
        }
        "-s" | "--set" => {
            if args.len() < 3 {
                eprintln!("Usage: akclip -s <text>");
                exit(1);
            }
            let text = &args[2..].join(" ");
            if clipboard::set(&text) {
                println!("Text copied to clipboard");
            } else {
                eprintln!("Failed to write to clipboard");
                exit(1);
            }
        }
        "-h" | "--help" => {
            println!("akclip - A clipboard utility");
            println!("Usage:");
            println!("  akclip              Read from clipboard and print to stdout");
            println!("  akclip -g           Get clipboard content (same as above)");
            println!("  akclip -s <text>    Set clipboard content");
            println!("  akclip -h           Show this help message");
        }
        _ => {
            eprintln!("Unknown option: {}", subcommand);
            eprintln!("Usage: akclip [-g|--get] [-s|--set <text>] [-h|--help]");
            exit(1);
        }
    }
}