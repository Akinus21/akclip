use std::env;
use std::io::{self, Read};
use arboard::Clipboard;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("Usage: akclip [OPTIONS]");
        println!("   -s, --stream     Accumulate all input then copy on Ctrl+C");
        println!("   -c, --clipboard-only   Enter interactive mode for manual paste+copy");
        println!("   -h, --help        Show this help message");
        println!("   --version         Show version");
        return;
    }
    
    if args.contains(&"--version".to_string()) {
        println!("akclip version 0.1.1");
        return;
    }
    
    let stream_mode = args.contains(&"-s".to_string()) || args.contains(&"--stream".to_string());
    let clipboard_only = args.contains(&"-c".to_string()) || args.contains(&"--clipboard-only".to_string());
    
    if clipboard_only {
        println!("akclip interactive mode: paste content and press Ctrl+C to copy to clipboard");
        println!("Press Ctrl+D to exit...");
        let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");
        clipboard.set_text(&buffer).expect("Failed to set clipboard");
        println!("Content copied to clipboard!");
        return;
    }
    
    if stream_mode {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");
        let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
        clipboard.set_text(&buffer).expect("Failed to set clipboard");
        println!("Content copied to clipboard!");
    } else {
        println!("Usage: akclip [-s|--stream] [-c|--clipboard-only] [-h|--help]");
        println!("   akclip captures stdin to the clipboard.  With --stream (-s), accumulates all input then copies on Ctrl+C.");
        println!("   With --clipboard-only (-c), enters interactive mode for manual paste+copy.");
        println!("   Use -s to stream output to a tmp file, then copy the tmp file to the clipboard when the command is finished.");
        println!("   --version: Show version");
        println!("   --help: Show help");
    }
}