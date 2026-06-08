use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--help" => {
                println!("Usage: {} [OPTIONS]", args[0]);
                println!("Options:");
                println!("  --help    Show this help message");
                println!("  --version Show version information");
                exit(0);
            }
            "--version" => {
                println!("akclip v0.0.31");
                exit(0);
            }
            _ => {
                if args[1].starts_with('-') {
                    eprintln!("Error: Unknown option: {}", args[1]);
                    eprintln!("Run with --help for usage information");
                    exit(1);
                }
            }
        }
    }

    if args.len() < 2 {
        eprintln!("Error: Missing required argument");
        eprintln!("Usage: {} <text>", args[0]);
        eprintln!("Run with --help for more information");
        exit(1);
    }

    let input_text = &args[1];
    let result = transform_text(input_text);

    match result {
        Ok(output) => {
            println!("{}", output);
            exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}

fn transform_text(text: &str) -> Result<String, String> {
    if text.is_empty() {
        return Err("Input text cannot be empty".to_string());
    }

    let mut result = String::new();
    for (i, c) in text.chars().enumerate() {
        if i > 0 {
            result.push('-');
        }
        result.push(c);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_text_simple() {
        let result = transform_text("hello");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "h-e-l-l-o");
    }

    #[test]
    fn test_transform_text_empty() {
        let result = transform_text("");
        assert!(result.is_err());
    }

    #[test]
    fn test_transform_text_single_char() {
        let result = transform_text("a");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "a");
    }
}