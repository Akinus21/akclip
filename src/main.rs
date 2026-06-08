use std::env;
use std::process::exit;
use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: akclip <command> [args...]");
        exit(1);
    }

    let command = &args[1];
    let command_args = if args.len() > 2 { &args[2..] } else { &[] };

    let should_continue = AtomicBool::new(true);
    let should_continue_clone = should_continue.clone();

    ctrlc::set_handler(move || {
        should_continue_clone.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let result = match command {
        "copy" | "c" => copy_to_clipboard(command_args),
        "paste" | "p" => paste_from_clipboard(),
        "image" | "i" => paste_image_from_clipboard(),
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Usage: akclip <copy|c|paste|p|image|i>");
            exit(1);
        }
    };

    if !result {
        exit(1);
    }
}

fn copy_to_clipboard(args: &[String]) -> bool {
    let mut input = String::new();

    if args.is_empty() || args[0] == "-" {
        if let Err(e) = std::io::stdin().read_to_string(&mut input) {
            eprintln!("Error reading from stdin: {}", e);
            return false;
        }
    } else {
        input = args.join(" ");
    }

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return true;
    }

    match arboard::Clipboard::new() {
        Ok(mut clipboard) => {
            match clipboard.set_text(trimmed) {
                Ok(_) => true,
                Err(e) => {
                    eprintln!("Error setting clipboard: {:?}", e);
                    false
                }
            }
        }
        Err(e) => {
            eprintln!("Error accessing clipboard: {:?}", e);
            false
        }
    }
}

fn paste_from_clipboard() -> bool {
    match arboard::Clipboard::new() {
        Ok(mut clipboard) => {
            match clipboard.get_text() {
                Ok(text) => {
                    print!("{}", text);
                    true
                }
                Err(e) => {
                    eprintln!("Error getting clipboard: {:?}", e);
                    false
                }
            }
        }
        Err(e) => {
            eprintln!("Error accessing clipboard: {:?}", e);
            false
        }
    }
}

fn paste_image_from_clipboard() -> bool {
    match arboard::Clipboard::new() {
        Ok(mut clipboard) => {
            match clipboard.get_image() {
                Ok(img_data) => {
                    let width = img_data.width;
                    let height = img_data.height;
                    let bytes = img_data.bytes.into_owned();

                    match encode_to_png(&bytes, width, height) {
                        Ok(png_data) => {
                            let mut echo = Command::new("echo");
                            echo.arg("-n");
                            echo.arg(base64::encode(&png_data));

                            if let Some(stdin) = echo.stdin.as_ref() {
                                // stdin already configured, use it
                            }

                            match echo.output() {
                                Ok(output) => {
                                    if output.status.success() {
                                        let base64_str = String::from_utf8_lossy(&output.stdout);
                                        println!("{}", base64_str.trim());
                                        true
                                    } else {
                                        eprintln!("echo command failed");
                                        false
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error running echo: {}", e);
                                    false
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error encoding PNG: {:?}", e);
                            false
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error getting image from clipboard: {:?}", e);
                    false
                }
            }
        }
        Err(e) => {
            eprintln!("Error accessing clipboard: {:?}", e);
            false
        }
    }
}

fn encode_to_png(rgba_data: &[u8], width: usize, height: usize) -> Result<Vec<u8>, String> {
    use image::{ImageBuffer, Rgba};

    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
        width as u32,
        height as u32,
        rgba_data.to_vec()
    ).ok_or_else(|| "Failed to create image buffer".to_string())?;

    let mut png_data = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut png_data);

    img.write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|e| format!("Error writing PNG: {}", e))?;

    Ok(png_data)
}

mod base64 {
    pub fn encode(data: &[u8]) -> String {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        let mut result = String::new();
        let mut i = 0;

        while i < data.len() {
            let b1 = data[i] as usize;
            let b2 = if i + 1 < data.len() { data[i + 1] as usize } else { 0 };
            let b3 = if i + 2 < data.len() { data[i + 2] as usize } else { 0 };

            result.push(ALPHABET[b1 >> 2] as char);
            result.push(ALPHABET[((b1 & 0x03) << 4) | (b2 >> 4)] as char);

            if i + 1 < data.len() {
                result.push(ALPHABET[((b2 & 0x0f) << 2) | (b3 >> 6)] as char);
            } else {
                result.push('=');
            }

            if i + 2 < data.len() {
                result.push(ALPHABET[b3 & 0x3f] as char);
            } else {
                result.push('=');
            }

            i += 3;
        }

        result
    }
}

mod ctrlc {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use std::time::Duration;

    static mut HANDLER: Option<Box<dyn Fn() + Send>> = None;

    pub fn set_handler(handler: impl Fn() + Send + 'static) -> Result<(), String> {
        unsafe {
            HANDLER = Some(Box::new(handler));
        }

        thread::spawn(|| {
            let mut bytes = [0u8; 1];
            loop {
                std::thread::sleep(Duration::from_millis(100));
            }
        });

        Ok(())
    }
}