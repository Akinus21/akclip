use std::env;
use std::process::exit;

#[cfg(target_os = "macos")]
mod clipboard {
    use std::panic;

    #[link(name = "pthread")]
    extern "C" {}

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct MacPasteboard {
        pub data: *mut std::ffi::c_void,
        pub len: usize,
    }

    impl Drop for MacPasteboard {
        fn drop(&mut self) {
            if !self.data.is_null() {
                unsafe {
                    libc::free(self.data as *mut libc::c_void);
                }
            }
        }
    }

    impl MacPasteboard {
        pub fn new() -> Self {
            MacPasteboard {
                data: std::ptr::null_mut(),
                len: 0,
            }
        }

        pub fn get(&mut self) -> Result<String, String> {
            unsafe {
                let type_util = libc::Long::max(4, 0);
                let mut data_len: libc::size_t = 0;
                let pasteboard: i32 = 1;
                let err = libc::pthread_main_np();

                if err == 0 {
                    return Err("Not on main thread".to_string());
                }

                let result = libc::pasteboard_try_get_item_flavor_data_by_idx(
                    pasteboard,
                    0,
                    type_util,
                    &mut self.data,
                    &mut data_len,
                );

                if result != 0 || self.data.is_null() {
                    return Err("Failed to get clipboard data".to_string());
                }

                self.len = data_len;
                let slice = std::slice::from_raw_parts(self.data as *const u8, data_len);
                String::from_utf8(slice.to_vec())
                    .map_err(|e| format!("UTF-8 conversion error: {}", e))
            }
        }

        pub fn set(&mut self, text: &str) -> Result<(), String> {
            unsafe {
                let err = libc::pthread_main_np();
                if err == 0 {
                    return Err("Not on main thread".to_string());
                }

                let bytes = text.as_bytes();
                let len = bytes.len();

                let data = libc::malloc(len as libc::size_t);
                if data.is_null() {
                    return Err("Failed to allocate memory".to_string());
                }

                std::ptr::copy_nonoverlapping(bytes.as_ptr(), data as *mut u8, len);

                let pasteboard: i32 = 1;
                let result = libc::pasteboard_put_item_flavor_data(
                    pasteboard,
                    0,
                    libc::Long::max(4, 0),
                    data,
                    len as libc::size_t,
                );

                libc::free(data);

                if result != 0 {
                    return Err("Failed to set clipboard data".to_string());
                }

                Ok(())
            }
        }
    }

    pub fn get_clipboard() -> Result<String, String> {
        let mut pasteboard = MacPasteboard::new();
        pasteboard.get()
    }

    pub fn set_clipboard(text: &str) -> Result<(), String> {
        let mut pasteboard = MacPasteboard::new();
        pasteboard.set(text)
    }

    fn setup_panic_handler() {
        panic::set_hook(Box::new(|panic_info| {
            let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };

            let location = if let Some(loc) = panic_info.location() {
                format!("{}:{}:{}", loc.file(), loc.line(), loc.column())
            } else {
                "unknown location".to_string()
            };

            eprintln!("PANIC at {}: {}", location, msg);
        }));
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    use std::sync::Mutex;

    static CLIPBOARD: Mutex<Option<String>> = Mutex::new(None);

    pub fn get_clipboard() -> Result<String, String> {
        let clipboard = CLIPBOARD.lock().map_err(|_| "Lock poisoned")?;
        clipboard.clone().ok_or_else(|| "Clipboard empty".to_string())
    }

    pub fn set_clipboard(text: &str) -> Result<(), String> {
        let mut clipboard = CLIPBOARD.lock().map_err(|_| "Lock poisoned")?;
        *clipboard = Some(text.to_string());
        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod clipboard {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;

    #[link(name = "user32")]
    extern "system" {
        fn OpenClipboard(hwnd: *mut std::ffi::c_void) -> i32;
        fn CloseClipboard() -> i32;
        fn EmptyClipboard() -> i32;
        fn GetClipboardData(format: u32) -> *mut std::ffi::c_void;
        fn SetClipboardData(format: u32, data: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
        fn GlobalAlloc(flags: u32, bytes: usize) -> *mut std::ffi::c_void;
        fn GlobalLock(mem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
        fn GlobalUnlock(mem: *mut std::ffi::c_void) -> i32;
    }

    const CF_UNICODETEXT: u32 = 13;
    const GMEM_MOVEABLE: u32 = 0x0002;

    fn to_wide_string(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(Some(0)).collect()
    }

    fn from_wide_string(ptr: *const u16) -> String {
        if ptr.is_null() {
            return String::new();
        }

        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }

        let slice = std::slice::from_raw_parts(ptr, len);
        String::from_utf16_lossy(slice)
    }

    pub fn get_clipboard() -> Result<String, String> {
        unsafe {
            if OpenClipboard(null_mut()) == 0 {
                return Err("Failed to open clipboard".to_string());
            }

            let data = GetClipboardData(CF_UNICODETEXT);
            let result = if !data.is_null() {
                let locked = GlobalLock(data);
                if !locked.is_null() {
                    let text = from_wide_string(locked as *const u16);
                    GlobalUnlock(data);
                    Ok(text)
                } else {
                    Err("Failed to lock clipboard data".to_string())
                }
            } else {
                Err("No Unicode text in clipboard".to_string())
            };

            CloseClipboard();
            result
        }
    }

    pub fn set_clipboard(text: &str) -> Result<(), String> {
        unsafe {
            if OpenClipboard(null_mut()) == 0 {
                return Err("Failed to open clipboard".to_string());
            }

            EmptyClipboard();

            let wide = to_wide_string(text);
            let size = wide.len() * std::mem::size_of::<u16>();
            let mem = GlobalAlloc(GMEM_MOVEABLE, size);

            if mem.is_null() {
                CloseClipboard();
                return Err("Failed to allocate memory".to_string());
            }

            let locked = GlobalLock(mem);
            if locked.is_null() {
                CloseClipboard();
                return Err("Failed to lock memory".to_string());
            }

            std::ptr::copy_nonoverlapping(wide.as_ptr(), locked as *mut u16, wide.len());
            GlobalUnlock(mem);

            SetClipboardData(CF_UNICODETEXT, mem);
            CloseClipboard();

            Ok(())
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
mod clipboard {
    pub fn get_clipboard() -> Result<String, String> {
        Err("Unsupported platform".to_string())
    }

    pub fn set_clipboard(_text: &str) -> Result<(), String> {
        Err("Unsupported platform".to_string())
    }
}

fn get_clipboard() -> Result<String, String> {
    clipboard::get_clipboard()
}

fn set_clipboard(text: &str) -> Result<(), String> {
    clipboard::set_clipboard(text)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <set|get> [text]", args[0]);
        exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "get" => {
            match get_clipboard() {
                Ok(content) => {
                    println!("{}", content);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        "set" => {
            if args.len() < 3 {
                eprintln!("Usage: {} set <text>", args[0]);
                exit(1);
            }

            let text = &args[2];
            match set_clipboard(text) {
                Ok(()) => {
                    println!("Clipboard set successfully");
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("Invalid command: {}. Use 'get' or 'set'", command);
            exit(1);
        }
    }
}