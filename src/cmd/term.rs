use std::process::{Command, Stdio};
use std::str;

pub fn exec(cmd: &str) {
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .spawn()
        .expect("term command failed to start!");
    child.wait().expect("failed to wait on child");
}

pub fn exec_with_output(cmd: &str) -> String {
    let child = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .spawn()
        .expect("term command failed to start!");
    let output = child.wait_with_output().expect("failed to wait on child");
    String::from(
        str::from_utf8(&output.stdout)
            .expect("invalid utf8 sequence")
            .trim(),
    )
}
