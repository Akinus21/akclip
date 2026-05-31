use std::env;

fn main() {
    match env::args() {
        [] => {
            println!("Usage: $0 [-s|--stream] [-c|--clipboard-only] [-h|--help]");
            println!("   akclip captures stdin to the clipboard.  With --stream (-s), accumulates all input then copies on Ctrl+C.");
            println!("   With --clipboard-only (-c), enters interactive mode for manual paste+copy.");
            println!("   Use -s to stream output to a tmp file, then copy the tmp file to the clipboard when the command is finished.");
            println!("   --version: Show version");
            println!("   --help: Show help");
            return;
        },
        _ => {
            println!("Usage: $0 [-s|--stream] [-c|--clipboard-only] [-h|--help]");
            println!("   akclip captures stdin to the clipboard.  With --stream (-s), accumulates all input then copies on Ctrl+C.");
            println!("   With --clipboard-only (-c), enters interactive mode for manual paste+copy.");
            println!("   Use -s to stream output to a tmp file, then copy the tmp file to the clipboard when the command is finished.");
            println!("   --version: Show version");
            println!("   --help: Show help");
            return;
        }
    }
}
