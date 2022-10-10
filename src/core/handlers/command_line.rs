use std::{
    error::Error as StdError,
    fmt, io,
    process::{Command, Stdio},
    str,
};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    EmptyOutput,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::EmptyOutput => f.write_str("EmptyString"),
            Self::Io(err) => f.write_str(&err.to_string()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Self::EmptyOutput => "Empty String",
            Self::Io(err) => &err.to_string(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

pub fn run(command: &str) -> Result<(), io::Error> {
    let mut child = Command::new("bash").arg("-c").arg(command).spawn()?;
    child.wait()?;
    Ok(())
}

pub fn run_and_fetch_output(command: &str) -> Result<String, Error> {
    let child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()?;
    let output = str::from_utf8(&child.wait_with_output()?.stdout)
        .expect("Command: invalid utf8 sequence output");
    if !output.is_empty() {
        Ok(output.trim().to_string())
    } else {
        Err(Error::EmptyOutput)
    }
}
