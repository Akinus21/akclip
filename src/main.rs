use std::env;
use std::io::{self, Write};
use std::process::exit;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: akclip <command> [options]");
        eprintln!("Commands:");
        eprintln!("  copy <text>   - Copy text to clipboard");
        eprintln!("  paste         - Paste from clipboard");
        eprintln!("  clear         - Clear clipboard");
        exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "copy" => {
            if args.len() < 3 {
                eprintln!("Usage: akclip copy <text>");
                exit(1);
            }
            let text = &args[2];
            copy_to_clipboard(text);
        },
        "paste" => {
            paste_from_clipboard();
        },
        "clear" => {
            clear_clipboard();
        },
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Available commands: copy, paste, clear");
            exit(1);
        }
    }
}

fn copy_to_clipboard(text: &str) {
    let mut xclip = Command::new("xclip")
        .args(&["-selection", "clipboard"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn xclip");

    if let Some(mut stdin) = xclip.stdin.take() {
        use std::io::Write;
        stdin.write_all(text.as_bytes()).expect("Failed to write to xclip stdin");
    }

    xclip.wait().expect("Failed to wait for xclip");
}

fn paste_from_clipboard() {
    let output = Command::new("xclip")
        .args(&["-selection", "clipboard", "-o"])
        .output()
        .expect("Failed to execute xclip");

    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout);
        print!("{}", text);
        std::io::stdout().flush().ok();
    } else {
        eprintln!("Failed to paste from clipboard");
        exit(1);
    }
}

fn clear_clipboard() {
    let mut xclip = Command::new("xclip")
        .args(&["-selection", "clipboard", "-i"])
        .stdin(std::process::Stdio::null())
        .spawn()
        .expect("Failed to spawn xclip");

    xclip.wait().expect("Failed to wait for xclip");
    
    let mut xsel = Command::new("xsel")
        .args(&["--clipboard", "--clear"])
        .spawn()
        .expect("Failed to spawn xsel");

    xsel.wait().expect("Failed to wait for xsel");
}

fn get_clipboard_content() -> Option<String> {
    let output = Command::new("xclip")
        .args(&["-selection", "clipboard", "-o"])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
}

fn set_clipboard_content(content: &str) -> bool {
    let mut xclip = Command::new("xclip")
        .args(&["-selection", "clipboard"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .is_ok();

    xclip
}

fn wait_for_clipboard_change(original: &str, timeout_secs: u64) -> Option<String> {
    let start = std::time::Instant::now();
    
    while start.elapsed().as_secs() < timeout_secs {
        if let Some(current) = get_clipboard_content() {
            if current != original {
                return Some(current);
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    
    None
}

fn has_xclip() -> bool {
    Command::new("xclip")
        .arg("-v")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn has_xsel() -> bool {
    Command::new("xsel")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn detect_clipboard_tool() -> &'static str {
    if has_xclip() {
        "xclip"
    } else if has_xsel() {
        "xsel"
    } else {
        eprintln!("Error: Neither xclip nor xsel is installed");
        eprintln!("Please install xclip or xsel to use akclip");
        exit(1);
    }
}

fn copy_with_retry(text: &str, max_retries: u32) -> bool {
    for attempt in 0..max_retries {
        if set_clipboard_content(text) {
            return true;
        }
        if attempt < max_retries - 1 {
            thread::sleep(Duration::from_millis(50));
        }
    }
    false
}

fn monitor_clipboard<F>(callback: F, interval_ms: u64)
where
    F: Fn(String) + Send + 'static,
{
    let mut last_content = get_clipboard_content().unwrap_or_default();
    
    loop {
        thread::sleep(Duration::from_millis(interval_ms));
        
        if let Some(current) = get_clipboard_content() {
            if current != last_content {
                last_content = current.clone();
                callback(current);
            }
        }
    }
}