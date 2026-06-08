use std::env;
use std::process::exit;

#[cfg(target_os = "macos")]
mod clipboard {
    use std::error::Error;
    use std::ffi::CStr;
    use std::ptr;
    use std::sync::OnceLock;

    static PASTEBOARD: OnceLock<*mut std::ffi::c_void> = OnceLock::new();

    fn get_pasteboard() -> Result<*mut std::ffi::c_void, Box<dyn Error>> {
        if let Some(pb) = PASTEBOARD.get() {
            return Ok(*pb);
        }
        unsafe {
            let appkit = libc::dlopen(
                b"/System/Library/Frameworks/AppKit.framework/AppKit\0".as_ptr() as *const i8,
                libc::RTLD_LAZY,
            );
            if appkit.is_null() {
                return Err("Failed to load AppKit framework".into());
            }
            let pasteboard_sym = libc::dlsym(
                appkit,
                b"UIPasteboard\0".as_ptr() as *const i8,
            );
            if pasteboard_sym.is_null() {
                return Err("Failed to find UIPasteboard symbol".into());
            }
            let pb = pasteboard_sym as *mut std::ffi::c_void;
            PASTEBOARD.set(pb).map_err(|_| "Failed to set pasteboard")?;
            Ok(pb)
        }
    }

    pub fn read_clipboard() -> Result<String, Box<dyn Error>> {
        let pb = get_pasteboard()?;
        unsafe {
            let items = (*(pb as *const PasteboardRef)).items;
            if items.is_null() || (*items).is_empty() {
                return Ok(String::new());
            }
            let item = *(*items).get_unchecked(0);
            if item.is_null() {
                return Ok(String::new());
            }
            let item_types = (*item).itemTypes;
            if item_types.is_null() {
                return Ok(String::new());
            }
            let utf8_type = CFSTR("public.utf8-plain-text");
            let has_utf8 = (*item_types)
                .iter()
                .any(|t| *t == utf8_type);
            if !has_utf8 {
                return Ok(String::new());
            }
            let data_ptr = get_pasteboard_data(pb, utf8_type)?;
            if data_ptr.is_null() {
                return Ok(String::new());
            }
            let data_len = libc::strlen(data_ptr);
            let bytes = std::slice::from_raw_parts(data_ptr as *const u8, data_len);
            String::from_utf8(bytes.to_vec()).map_err(|e| e.into())
        }
    }

    pub fn write_clipboard(content: &str) -> Result<(), Box<dyn Error>> {
        let pb = get_pasteboard()?;
        unsafe {
            let items = (*(pb as *const PasteboardRef)).items;
            if items.is_null() {
                return Err("No items in pasteboard".into());
            }
            if (*items).is_empty() {
                return Err("Empty pasteboard items".into());
            }
            let item = *(*items).get_unchecked(0);
            if item.is_null() {
                return Err("Null pasteboard item".into());
            }
            let cf_string = CFSTR(content.as_ptr() as *const i8);
            set_pasteboard_data(pb, cf_string)?;
            Ok(())
        }
    }
}

#[cfg(target_os = "linux")]
mod clipboard {
    use std::error::Error;
    use arboard::Clipboard;

    pub fn read_clipboard() -> Result<String, Box<dyn Error>> {
        let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
        clipboard.get_text().map_err(|e| e.to_string())
    }

    pub fn write_clipboard(content: &str) -> Result<(), Box<dyn Error>> {
        let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
        clipboard.set_text(content).map_err(|e| e.to_string())
    }
}

fn get_clipboard_content() -> Result<String, Box<dyn Error>> {
    #[cfg(target_os = "macos")]
    {
        clipboard::read_clipboard()
    }
    #[cfg(target_os = "linux")]
    {
        clipboard::read_clipboard()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        Err("Unsupported platform".into())
    }
}

fn set_clipboard_content(content: &str) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "macos")]
    {
        clipboard::write_clipboard(content)
    }
    #[cfg(target_os = "linux")]
    {
        clipboard::write_clipboard(content)
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        Err("Unsupported platform".into())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <set|get> [content]", args[0]);
        eprintln!("  set: Set clipboard content");
        eprintln!("  get: Get clipboard content");
        exit(1);
    }

    match args[1].as_str() {
        "get" => {
            match get_clipboard_content() {
                Ok(content) => {
                    println!("{}", content);
                }
                Err(e) => {
                    eprintln!("Error getting clipboard: {}", e);
                    exit(1);
                }
            }
        }
        "set" => {
            let content = if args.len() > 2 {
                args[2].clone()
            } else {
                eprintln!("Error: 'set' command requires content argument");
                exit(1);
            };
            match set_clipboard_content(&content) {
                Ok(_) => {
                    println!("Clipboard set successfully");
                }
                Err(e) => {
                    eprintln!("Error setting clipboard: {}", e);
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("Invalid command: {}", args[1]);
            eprintln!("Use 'get' or 'set'");
            exit(1);
        }
    }
}