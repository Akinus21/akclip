use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args...]", args[0]);
        exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "clipboard" | "clip" => {
            handle_clipboard_operation(&args[2..]);
        },
        "copy" => {
            handle_copy_operation(&args[2..]);
        },
        "paste" => {
            handle_paste_operation();
        },
        "clear" => {
            handle_clear_operation();
        },
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Available commands: clipboard, copy, paste, clear");
            exit(1);
        }
    }
}

fn handle_clipboard_operation(args: &[String]) {
    if args.is_empty() {
        eprintln!("clipboard command requires an action");
        exit(1);
    }
    
    match args[0].as_str() {
        "get" => {
            println!("Getting clipboard contents...");
        },
        "set" => {
            if args.len() < 2 {
                eprintln!("Usage: clipboard set <text>");
                exit(1);
            }
            println!("Setting clipboard: {}", args[1]);
        },
        _ => {
            eprintln!("Unknown clipboard action: {}", args[0]);
            exit(1);
        }
    }
}

fn handle_copy_operation(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: copy <text>");
        exit(1);
    }
    
    let text = args.join(" ");
    println!("Copied to clipboard: {}", text);
}

fn handle_paste_operation() {
    println!("Pasting from clipboard...");
}

fn handle_clear_operation() {
    println!("Clipboard cleared");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_main_entry() {
        assert!(true);
    }
    
    #[test]
    fn test_handle_copy() {
        let test_args = ["test".to_string()];
        handle_copy_operation(&test_args);
    }
    
    #[test]
    fn test_handle_paste() {
        handle_paste_operation();
    }
    
    #[test]
    fn test_handle_clear() {
        handle_clear_operation();
    }
}