use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let action = &args[1];
        match action.as_str() {
            "copy" => {
                if args.len() > 2 {
                    let text = &args[2..].join(" ");
                    copy_to_clipboard(&text);
                    println!("Copied to clipboard: {}", text);
                } else {
                    println!("Usage: akclip copy <text>");
                }
            }
            "paste" => {
                if let Some(text) = paste_from_clipboard() {
                    println!("{}", text);
                } else {
                    println!("Clipboard is empty");
                }
            }
            "clear" => {
                clear_clipboard();
                println!("Clipboard cleared");
            }
            _ => {
                println!("Unknown action: {}", action);
                println!("Usage: akclip <copy|paste|clear>");
            }
        }
    } else {
        println!("akclip - A clipboard utility");
        println!("Usage: akclip <copy|paste|clear>");
    }
}

#[cfg(target_os = "windows")]
fn copy_to_clipboard(text: &str) {
    use std::process::Command;
    let _ = Command::new("cmd")
        .args(["/C", &format!("echo {} | clip", text)])
        .output();
}

#[cfg(target_os = "macos")]
fn copy_to_clipboard(text: &str) {
    use std::process::Command;
    let _ = Command::new("pbcopy")
        .arg(text)
        .output();
}

#[cfg(target_os = "linux")]
fn copy_to_clipboard(text: &str) {
    use std::process::Command;
    let _ = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .arg("-i")
        .arg("-f")
        .stdin(osutils_stdin(text))
        .output();
}

#[cfg(target_os = "windows")]
fn paste_from_clipboard() -> Option<String> {
    use std::process::Command;
    Command::new("cmd")
        .args(["/C", "powershell -command Get-Clipboard"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
}

#[cfg(target_os = "macos")]
fn paste_from_clipboard() -> Option<String> {
    use std::process::Command;
    Command::new("pbpaste")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
}

#[cfg(target_os = "linux")]
fn paste_from_clipboard() -> Option<String> {
    use std::process::Command;
    Command::new("xclip")
        .args(["-selection", "clipboard", "-o"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
}

#[cfg(target_os = "windows")]
fn clear_clipboard() {
    use std::process::Command;
    let _ = Command::new("cmd")
        .args(["/C", "echo off | clip"])
        .output();
}

#[cfg(target_os = "macos")]
fn clear_clipboard() {
    use std::process::Command;
    let _ = Command::new("pbcopy")
        .arg("/dev/null")
        .output();
}

#[cfg(target_os = "linux")]
fn clear_clipboard() {
    use std::process::Command;
    let _ = Command::new("xclip")
        .args(["-selection", "clipboard", "-i", "/dev/null"])
        .output();
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn copy_to_clipboard(_text: &str) {
    eprintln!("Unsupported platform for copy operation");
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn paste_from_clipboard() -> Option<String> {
    eprintln!("Unsupported platform for paste operation");
    None
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn clear_clipboard() {
    eprintln!("Unsupported platform for clear operation");
}

#[cfg(target_os = "linux")]
fn osutils_stdin(text: &str) -> std::process::Stdio {
    use std::io::Write;
    let mut child = std::process::Command::new("xclip")
        .args(["-selection", "clipboard", "-i"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn xclip");
    if let Some(ref mut stdin) = child.stdin {
        let _ = stdin.write_all(text.as_bytes());
    }
    std::process::Stdio::piped()
}