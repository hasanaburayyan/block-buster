use std::process::Command;

fn main() {
    let status = Command::new("cargo")
        .args(&["fmt"])
        .status()
        .expect("Failed to run cargo fmt");

    if !status.success() {
        panic!("cargo fmt failed");
    }
}
