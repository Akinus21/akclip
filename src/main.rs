use std::env;
use std::process::exit;

#[cfg(target_os = "macos")]
mod clipboard {
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::ptr;

    #[link(name = "Carbon", kind = "framework")]
    extern "C" {
        fn SetPasteboardData(format: u32, data: *const u8, length: u32);
    }

    pub fn set_clipboard(text: &str) -> Result<(), String> {
        let c_text = CString::new(text).map_err(|e| e.to_string())?;
        let bytes = text.as_bytes();

        unsafe {
            SetPasteboardData(0, bytes.as_ptr(), bytes.len() as u32);
        }

        Ok(())
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    pub fn set_clipboard(text: &str) -> Result<(), String> {
        use std::process::Command;

        let echo = Command::new("echo")
            .arg("-n")
            .arg(text)
            .output()
            .map_err(|e| e.to_string())?;

        let xclip = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdin(echo.stdin.take().unwrap_or_else(|| {
                panic!("Failed to capture stdin")
            }))
            .output()
            .map_err(|e| e.to_string())?;

        if !xclip.status.success() {
            return Err(String::from_utf8_lossy(&xclip.stderr).to_string());
        }

        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod clipboard {
    use std::ptr;

    #[link(name = "user32")]
    extern "system" {
        fn OpenClipboard(hwnd: *mut std::ffi::c_void) -> i32;
        fn CloseClipboard() -> i32;
        fn EmptyClipboard() -> i32;
        fn SetClipboardData(format: u32, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
        fn GlobalAlloc(flags: u32, bytes: usize) -> *mut std::ffi::c_void;
        fn GlobalLock(mem: *mut std::ffi::c_void) -> *mut u8;
        fn GlobalUnlock(mem: *mut std::ffi::c_void) -> i32;
    }

    pub fn set_clipboard(text: &str) -> Result<(), String> {
        unsafe {
            if OpenClipboard(ptr::null_mut()) == 0 {
                return Err("Failed to open clipboard".to_string());
            }

            EmptyClipboard();

            let size = text.len() * std::mem::size_of::<u16>();
            let mem = GlobalAlloc(0x0002, size + 2);

            if mem.is_null() {
                CloseClipboard();
                return Err("Failed to allocate memory".to_string());
            }

            let lock = GlobalLock(mem);
            let wide: Vec<u16> = text.encode_utf16().collect();
            std::ptr::copy_nonoverlapping(wide.as_ptr(), lock as *mut u16, wide.len());

            GlobalUnlock(mem);

            SetClipboardData(13, mem);
            CloseClipboard();
        }

        Ok(())
    }
}

fn get_env_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <text>", args[0]);
        exit(1);
    }

    let text = &args[1];

    let result = clipboard::set_clipboard(text);

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        exit(1);
    }

    println!("Clipboard set to: {}", text);
}