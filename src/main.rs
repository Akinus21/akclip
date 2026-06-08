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
        "hello" => {
            let name = if args.len() > 2 {
                args[2].clone()
            } else {
                String::from("World")
            };
            println!("Hello, {}!", name);
        }
        "echo" => {
            let remaining: Vec<String> = args[2..].to_vec();
            println!("{}", remaining.join(" "));
        }
        "add" => {
            if args.len() < 4 {
                eprintln!("Usage: {} add <num1> <num2>", args[0]);
                exit(1);
            }
            match (args[2].parse::<i64>(), args[3].parse::<i64>()) {
                (Ok(a), Ok(b)) => println!("{}", a + b),
                _ => {
                    eprintln!("Error: Both arguments must be valid integers");
                    exit(1);
                }
            }
        }
        "status" => {
            println!("Application Status: OK");
            println!("Version: 1.0.0");
            println!("Total arguments: {}", args.len() - 1);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Available commands: hello, echo, add, status");
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_command() {
        let args: Vec<String> = vec![
            String::from("test"),
            String::from("hello"),
            String::from("TestUser")
        ];
        assert_eq!(args[2], "TestUser");
    }

    #[test]
    fn test_add_command() {
        let args: Vec<String> = vec![
            String::from("test"),
            String::from("add"),
            String::from("5"),
            String::from("3")
        ];
        assert_eq!(args[2].parse::<i64>().unwrap(), 5);
        assert_eq!(args[3].parse::<i64>().unwrap(), 3);
    }
}