use std::process::{Command, Stdio};

pub fn cli_execute(cmd: &str) -> Result<(), String> {
    let child = Command::new(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| e.to_string())?;

    let status = child.wait_with_output().map_err(|e| e.to_string())?;

    if !status.status.success() {
        Err(String::from_utf8_lossy(&status.stderr).into_owned())
    } else {
        Ok(())
    }
}
