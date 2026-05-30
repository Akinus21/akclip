use std::env;
use std::fs::File;
use std::io::Write;

// Function to capture output and send to the clipboard
fn capture_and_copy(output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut tmpfile = String::new();
    let mut contents = String::new();

    // Stream output to a temporary file
    writeln!(tmpfile, "{}", output)?;
    contents = contents;

    // Copy the temporary file to the clipboard
    let mut tmp_file = File::create(tmpfile)?;
    let mut contents_write = String::new();
    for c in &contents {
        contents_write.push_str(c);
    }
    writeln!(tmp_file, "{}", contents_write)?;

    // Log the event to the active system journal
    println!("akclip: capturing output to clipboard");
    let mut log_message = "akclip: capturing output to clipboard";
    write!(log_message, &mut log_message)?;

    // Debug mode (enable detailed logging)
    if log_message == "akclip: no debug logging enabled" {
        println!("akclip: No debug logging enabled");
    }

    Ok(())
}

// Function to handle the command completion
fn complete_command(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let command = command.trim();

    if command == "akclip" {
        capture_and_copy(&command)?;
    }

    Ok(())
}

fn main() {
    match env::args() {
        [] => {
            println!("Usage: $0 [-s|--stream] [-c|--clipboard-only] [-h|--help]" );
            println!("   akclip captures stdin to the clipboard.  With --stream (-s), accumulates all input then copies on Ctrl+C.
   \nWith --clipboard-only (-c), enters interactive mode for manual paste+copy.
   \nUse -s to stream output to a tmp file, then copy the tmp file to the clipboard when the command is finished." );
            println!("   --version: Show version");
            println!("   --help: Show help");
            return;
        }
        _ => {
            println!("Usage: $0 [-s|--stream] [-c|--clipboard-only] [-h|--help]" );
            println!("   akclip captures stdin to the clipboard.  With --stream (-s), accumulates all input then copies on Ctrl+C.
   \nWith --clipboard-only (-c), enters interactive mode for manual paste+copy.
   \nUse -s to stream output to a tmp file, then copy the tmp file to the clipboard when the command is finished." );
            println!("   --version: Show version");
            println!("   --help: Show help");
            return;
    }
}
