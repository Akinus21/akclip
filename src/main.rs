use std::env;
use std::process::exit;

#[cfg(target_os = "macos")]
mod clipboard {
    use std::process::Command;

    pub fn copy(text: &str) {
        let mut cmd = Command::new("pbcopy");
        cmd.arg("-pboard").arg("general");
        match cmd.stdin(std::process::Stdio::piped()) {
            Ok(mut stdin) => {
                use std::io::Write;
                let _ = stdin.write_all(text.as_bytes());
                println!("Copied to clipboard");
            }
            Err(e) => {
                eprintln!("Failed to copy: {}", e);
                exit(1);
            }
        }
    }

    pub fn paste() {
        match Command::new("pbpaste").output() {
            Ok(output) => {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
            Err(e) => {
                eprintln!("Failed to paste: {}", e);
                exit(1);
            }
        }
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    use std::process::Command;

    pub fn copy(text: &str) {
        match Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .arg("-i")
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                use std::io::Write;
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(text.as_bytes());
                }
                let _ = child.wait();
                println!("Copied to clipboard");
            }
            Err(e) => {
                eprintln!("Failed to copy: {}", e);
                exit(1);
            }
        }
    }

    pub fn paste() {
        match Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .arg("-o")
            .output()
        {
            Ok(output) => {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
            Err(e) => {
                eprintln!("Failed to paste: {}", e);
                exit(1);
            }
        }
    }
}

#[cfg(target_os = "windows")]
mod clipboard {
    use std::process::Command;

    pub fn copy(text: &str) {
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", "echo", text, "|", "clip"]);
        match cmd.output() {
            Ok(_) => println!("Copied to clipboard"),
            Err(e) => {
                eprintln!("Failed to copy: {}", e);
                exit(1);
            }
        }
    }

    pub fn paste() {
        match Command::new("powershell")
            .args(["-Command", "Get-Clipboard"])
            .output()
        {
            Ok(output) => {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
            Err(e) => {
                eprintln!("Failed to paste: {}", e);
                exit(1);
            }
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
mod clipboard {
    pub fn copy(_text: &str) {
        eprintln!("Clipboard not supported on this platform");
        exit(1);
    }

    pub fn paste() {
        eprintln!("Clipboard not supported on this platform");
        exit(1);
    }
}

fn print_usage() {
    println!("akclip - Clipboard utility");
    println!("Usage: akclip [OPTIONS]");
    println!("Options:");
    println!("  -c, --copy <text>  Copy text to clipboard");
    println!("  -p, --paste        Paste from clipboard");
    println!("  -h, --help         Show this help message");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        exit(0);
    }

    match args[1].as_str() {
        "-c" | "--copy" => {
            if args.len() < 3 {
                eprintln!("Error: Missing text argument for copy");
                print_usage();
                exit(1);
            }
            clipboard::copy(&args[2]);
        }
        "-p" | "--paste" => {
            clipboard::paste();
        }
        "-h" | "--help" => {
            print_usage();
        }
        _ => {
            eprintln!("Error: Unknown option '{}'", args[1]);
            print_usage();
            exit(1);
        }
    }
}