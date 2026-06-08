use std::env;
use std::process::exit;

#[cfg(target_os = "macos")]
mod clipboard {
    use std::process::Command;

    pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
        let result = Command::new("pbcopy")
            .arg("-pboard")
            .arg("general")
            .stdin(std::process::Stdio::piped())
            .spawn();

        match result {
            Ok(mut child) => {
                if let Some(ref mut stdin) = child.stdin {
                    use std::io::Write;
                    stdin.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
                }
                child.wait().map_err(|e| e.to_string())?;
                Ok(())
            }
            Err(e) => Err(format!("Failed to spawn pbcopy: {}", e)),
        }
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    use std::process::Command;

    pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
        let result = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdin(std::process::Stdio::piped())
            .spawn();

        match result {
            Ok(mut child) => {
                if let Some(ref mut stdin) = child.stdin {
                    use std::io::Write;
                    stdin.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
                }
                child.wait().map_err(|e| e.to_string())?;
                Ok(())
            }
            Err(e) => Err(format!("Failed to spawn xclip: {}", e)),
        }
    }
}

#[cfg(target_os = "windows")]
mod clipboard {
    use std::ptr::null_mut;

    #[link(name = "user32")]
    extern "system" {
        fn OpenClipboard(hwnd: *mut std::ffi::c_void) -> i32;
        fn CloseClipboard() -> i32;
        fn EmptyClipboard() -> i32;
        fn SetClipboardData(format: u32, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    }

    pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
        unsafe {
            if OpenClipboard(null_mut()) == 0 {
                return Err("Failed to open clipboard".to_string());
            }
            EmptyClipboard();

            let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
            let size = wide.len() * std::mem::size_of::<u16>();

            let global = std::alloc::System.alloc(std::alloc::Layout::from_size_align(size, 1).unwrap());
            if global.is_null() {
                CloseClipboard();
                return Err("Failed to allocate memory".to_string());
            }

            std::ptr::copy_nonoverlapping(wide.as_ptr(), global as *mut u16, wide.len());
            SetClipboardData(13, global);
            CloseClipboard();

            Ok(())
        }
    }
}

fn get_clipboard_content() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let output = Command::new("pbpaste")
            .output()
            .map_err(|e| format!("Failed to execute pbpaste: {}", e))?;
        String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8: {}", e))
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let output = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .arg("-o")
            .output()
            .map_err(|e| format!("Failed to execute xclip: {}", e))?;
        String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8: {}", e))
    }

    #[cfg(target_os = "windows")]
    {
        Err("Get clipboard not implemented on Windows".to_string())
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Err("Unsupported platform".to_string())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: akclip [copy|clear|get]");
        eprintln!("  copy <text> - Copy text to clipboard");
        eprintln!("  clear       - Clear the clipboard");
        eprintln!("  get         - Get current clipboard content");
        exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "copy" => {
            if args.len() < 3 {
                eprintln!("Error: 'copy' requires text argument");
                exit(1);
            }
            let text = &args[2..].join(" ");
            match clipboard::copy_to_clipboard(&text) {
                Ok(()) => println!("Copied to clipboard"),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        "clear" => {
            match clipboard::copy_to_clipboard("") {
                Ok(()) => println!("Clipboard cleared"),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        "get" => {
            match get_clipboard_content() {
                Ok(content) => println!("{}", content),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Usage: akclip [copy|clear|get]");
            exit(1);
        }
    }
}