use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        let content = args[1..].join(" ");
        match set_clipboard(&content) {
            Ok(_) => {
                println!("Copied to clipboard: {}", content);
                exit(0);
            }
            Err(e) => {
                eprintln!("Failed to set clipboard: {}", e);
                exit(1);
            }
        }
    } else {
        match get_clipboard() {
            Ok(content) => {
                println!("{}", content);
                exit(0);
            }
            Err(e) => {
                eprintln!("Failed to get clipboard: {}", e);
                exit(1);
            }
        }
    }
}

fn get_clipboard() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let output = Command::new("pbpaste")
            .output()
            .map_err(|e| format!("Failed to execute pbpaste: {}", e))?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let output = Command::new("xclip")
            .args(&["-selection", "clipboard", "-o"])
            .output()
            .map_err(|e| format!("Failed to execute xclip: {}", e))?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let output = Command::new("powershell")
            .args(&["-Command", "Get-Clipboard"])
            .output()
            .map_err(|e| format!("Failed to execute Get-Clipboard: {}", e))?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Err("Unsupported platform".to_string())
    }
}

fn set_clipboard(content: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let mut child = Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn pbcopy: {}", e))?;
        
        use std::io::Write;
        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(content.as_bytes())
                .map_err(|e| format!("Failed to write to pbcopy: {}", e))?;
        }
        
        child.wait()
            .map_err(|e| format!("Failed to wait for pbcopy: {}", e))?;
        Ok(())
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let mut child = Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn xclip: {}", e))?;
        
        use std::io::Write;
        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(content.as_bytes())
                .map_err(|e| format!("Failed to write to xclip: {}", e))?;
        }
        
        child.wait()
            .map_err(|e| format!("Failed to wait for xclip: {}", e))?;
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let ps_command = format!("Set-Clipboard -Value '{}'", content.replace("'", "''"));
        Command::new("powershell")
            .args(&["-Command", &ps_command])
            .output()
            .map_err(|e| format!("Failed to execute Set-Clipboard: {}", e))?;
        Ok(())
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Err("Unsupported platform".to_string())
    }
}