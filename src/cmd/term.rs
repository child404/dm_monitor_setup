use crate::custom_errors::TermOutputError;
use std::process::{Command, Stdio};
use std::str;

pub struct TermCmd;

impl TermCmd {
    pub fn exec(cmd: &str) {
        let mut child = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .spawn()
            .expect("term command failed to start!");
        child.wait().expect("failed to wait on child");
    }

    pub fn exec_with_output(cmd: &str) -> Result<String, TermOutputError> {
        let child = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::piped())
            .spawn()
            .expect("term command failed to start!");
        match str::from_utf8(
            &child
                .wait_with_output()
                .expect("failed to wait on child")
                .stdout,
        )
        .expect("invalid utf8 sequence")
        .trim()
        {
            "" => Err(TermOutputError::EmptyString),
            valid_output => Ok(valid_output.to_string()),
        }
    }
}
