use std::process::Command;

fn main() {
    let output = Command::new("echo")
        .args(["[ERROR] No issue content provided. Please specify the issue to diagnose."])
        .output()
        .expect("Failed to execute command");
    
    println!("{}", String::from_utf8_lossy(&output.stdout));
}