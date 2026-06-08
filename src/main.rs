use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        exit(1);
    }
    
    let subcommand = &args[1];
    
    match subcommand.as_str() {
        "copy" | "-c" | "--copy" => {
            if let Err(e) = handle_copy(&args) {
                eprintln!("Error copying to clipboard: {}", e);
                exit(1);
            }
        }
        "paste" | "-p" | "--paste" => {
            if let Err(e) = handle_paste() {
                eprintln!("Error reading from clipboard: {}", e);
                exit(1);
            }
        }
        "help" | "-h" | "--help" => {
            print_usage();
        }
        "version" | "-v" | "--version" => {
            println!("akclip v{}", env!("CARGO_PKG_VERSION"));
        }
        _ => {
            eprintln!("Unknown command: {}", subcommand);
            print_usage();
            exit(1);
        }
    }
}

fn print_usage() {
    println!("akclip - A clipboard utility");
    println!();
    println!("Usage:");
    println!("  akclip copy <text>   Copy text to clipboard");
    println!("  akclip paste         Paste content from clipboard");
    println!("  akclip help          Show this help message");
    println!("  akclip version       Show version information");
    println!();
    println!("Options:");
    println!("  -c, --copy           Copy mode (same as 'copy')");
    println!("  -p, --paste          Paste mode (same as 'paste')");
    println!("  -h, --help           Show help");
    println!("  -v, --version        Show version");
}

fn handle_copy(args: &[String]) -> Result<(), String> {
    if args.len() < 3 {
        return Err("No text provided to copy. Usage: akclip copy <text>".to_string());
    }
    
    let text = &args[2];
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let echo = Command::new("echo")
            .arg("-n")
            .arg(text)
            .output()
            .map_err(|e| format!("Failed to execute echo: {}", e))?;
        
        let pbcopy = Command::new("pbcopy")
            .stdin(echo.stdout)
            .output()
            .map_err(|e| format!("Failed to execute pbcopy: {}", e))?;
        
        if !pbcopy.status.success() {
            return Err("pbcopy command failed".to_string());
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        
        let xclip = Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn xclip: {}", e))?;
        
        if let Some(mut stdin) = xclip.stdin.take() {
            use std::io::Write;
            stdin.write_all(text.as_bytes())
                .map_err(|e| format!("Failed to write to xclip: {}", e))?;
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("cmd")
            .args(&["/C", "echo", text, "|", "clip"])
            .output()
            .map_err(|e| format!("Failed to execute clip: {}", e))?;
    }
    
    println!("Copied to clipboard: {}", text);
    Ok(())
}

fn handle_paste() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let output = Command::new("pbpaste")
            .output()
            .map_err(|e| format!("Failed to execute pbpaste: {}", e))?;
        
        if !output.status.success() {
            return Err("pbpaste command failed".to_string());
        }
        
        let text = String::from_utf8_lossy(&output.stdout);
        print!("{}", text);
        io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let output = Command::new("xclip")
            .args(&["-selection", "clipboard", "-o"])
            .output()
            .map_err(|e| format!("Failed to execute xclip: {}", e))?;
        
        if !output.status.success() {
            return Err("xclip command failed. Make sure xclip is installed.".to_string());
        }
        
        let text = String::from_utf8_lossy(&output.stdout);
        print!("{}", text);
        io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;
    }
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let output = Command::new("cmd")
            .args(&["/C", "powershell", "-Command", "Get-Clipboard"])
            .output()
            .map_err(|e| format!("Failed to execute Get-Clipboard: {}", e))?;
        
        let text = String::from_utf8_lossy(&output.stdout);
        print!("{}", text);
        io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;
    }
    
    Ok(())
}