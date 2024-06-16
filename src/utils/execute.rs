use std::process::Command;

pub fn cli_execute(cmd: &str) {
    println!("⚙ Executing {} 🚀", cmd);

    let mut child = Command::new("sh") // or "cmd" on Windows
        .arg("-c")
        .arg(cmd)
        .spawn()
        .expect("Failed to start command");

    child.wait().expect("Failed to wait on command");
}
