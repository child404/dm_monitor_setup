use super::command_line as cmd;
use crate::defaults;
use std::{error::Error as StdError, fmt, process, str};

#[derive(Debug)]
pub enum Error {
    InvalidUserChoice(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidUserChoice(user_choice) => {
                f.write_str(&format!("InvalidUserChoice: {}", user_choice))
            }
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidUserChoice(user_choice) => {
                &format!("Invalid User Choice: {}", user_choice)
            }
        }
    }
}

pub struct Dmenu {
    pub executable: String,
    pub args: Vec<String>,
    pub prompt_options: Vec<String>,
    pub prompt_message: String,
}

impl Dmenu {
    pub fn new(prompt_options: &[&str], prompt_message: &str) -> Self {
        Self {
            executable: defaults::DMENU_BINARY.to_string(),
            args: defaults::DMENU_ARGS,
            prompt_options: prompt_options.to_vec(),
            prompt_message: prompt_message.to_string(),
        }
    }

    pub fn run_and_fetch_output(&self) -> Result<String, Error> {
        let output =
            cmd::run_and_fetch_output(&self.to_string()).unwrap_or_else(|error| match error {
                cmd::Error::EmptyOutput => process::exit(0), // handle Esc or ^[ to quit the dmenu
                cmd::Error::Io(error) => unimplemented!(),
            });
        if self._is_output_valid(&output) {
            Ok(output)
        } else {
            Err(Error::InvalidUserChoice(output))
        }
    }

    fn _is_output_valid(&self, output: &str) -> bool {
        self.prompt_options.is_empty() || self.prompt_options.contains(&output.to_string())
    }
}

impl ToString for Dmenu {
    fn to_string(&self) -> String {
        format!(
            "printf \"{}\" | {} {} \"{}\"",
            self.prompt_options.join("\n"),
            self.executable,
            self.args.join(" "),
            self.prompt_message
        )
    }
}
