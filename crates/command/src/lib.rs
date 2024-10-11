use std::process::{Command, Output, Stdio};
use std::io;

/// Runs the `cmd` given and uses `stdout` and `stderr` to forward it's output.
/// The `stdout` and `stderr` inherit the file descriptor of the parent process.
pub fn run_command(cmd: &str) -> Result<Output, io::Error> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let executable = parts[0];
    let args = &parts[1..];

    let child = Command::new(executable)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    child.wait_with_output()
}
