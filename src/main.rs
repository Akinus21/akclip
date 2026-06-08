use std::env;
use std::process::exit;
use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::io::{stdin, Write};
use arboard::Clipboard;

fn get_clipboard() -> Result<Clipboard, String> {
    Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))
}

fn copy_to_clipboard(content: &str) -> Result<(), String> {
    let mut clipboard = get_clipboard()?;
    clipboard.set_text(content).map_err(|e| format!("Failed to copy: {}", e))
}

fn paste_from_clipboard() -> Result<String, String> {
    let mut clipboard = get_clipboard()?;
    clipboard.get_text().map_err(|e| format!("Failed to paste: {}", e))
}

fn paste_image_from_clipboard() -> Result<(), String> {
    let mut clipboard = get_clipboard()?;
    let image_data = clipboard.get_image().map_err(|e| format!("Failed to get image from clipboard: {}", e))?;
    
    let width = image_data.width;
    let height = image_data.height;
    let bytes = image_data.bytes.into_owned();
    
    let mut cursor = Vec::new();
    
    use image::{ImageBuffer, Rgba};
    
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
        width as u32,
        height as u32,
        bytes
    ).ok_or("Failed to create image buffer")?;
    
    img.write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to encode image: {}", e))?;
    
    let base64_str = base64::encode(&cursor);
    println!("{}", base64_str);
    
    Ok(())
}

fn monitor_keyboard(should_continue: Arc<AtomicBool>) {
    let should_continue_clone = Arc::new(AtomicBool::new(should_continue.load(Ordering::SeqCst)));
    thread::spawn(move || {
        let mut last_key_time = std::time::Instant::now();
        loop {
            thread::sleep(Duration::from_millis(100));
            if !should_continue_clone.load(Ordering::SeqCst) {
                break;
            }
        }
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: akclip <command> [args]");
        eprintln!("Commands:");
        eprintln!("  copy, c <text>    - Copy text to clipboard");
        eprintln!("  paste, p          - Paste from clipboard");
        eprintln!("  image, i          - Paste image from clipboard as base64");
        exit(1);
    }
    
    let command = &args[1];
    let command_args: Vec<String> = args[2..].to_vec();
    
    let should_continue = Arc::new(AtomicBool::new(true));
    let should_continue_clone = should_continue.clone();
    
    let _monitor_thread = thread::spawn(move || {
        monitor_keyboard(should_continue_clone);
    });
    
    let result = match command.as_str() {
        "copy" | "c" => {
            let content = command_args.join(" ");
            copy_to_clipboard(&content)
        },
        "paste" | "p" => paste_from_clipboard().map(|text| {
            println!("{}", text);
            text
        }),
        "image" | "i" => paste_image_from_clipboard(),
        _ => {
            eprintln!("Unknown command: {}", command);
            exit(1);
        }
    };
    
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        exit(1);
    }
    
    let _ = should_continue.load(Ordering::SeqCst);
}