use crate::config::Config;
use std::process::{Command, Stdio};
use std::str;

pub struct DmenuCmd {
    pub executable: String,
    pub args: Vec<String>,
    pub prompt_options: Vec<String>,
    pub prompt_message: String,
}

pub struct Defaults;

impl DmenuCmd {
    pub fn new(prompt_options: Vec<String>, prompt_message: String) -> Self {
        DmenuCmd {
            executable: Config::dmenu_executable(),
            args: Config::dmenu_args(),
            prompt_options,
            prompt_message,
        }
    }

    pub fn exec(&self) -> String {
        let command = format!(
            "printf \"{}\" | {} {} -p \"{}\"",
            self.prompt_options.join("\n"),
            self.executable,
            self.args.join(" "),
            self.prompt_message
        );
        let child = Command::new("bash")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .spawn()
            .expect("dmenu command failed to start!");
        let output = child.wait_with_output().expect("failed to wait on child");
        String::from(
            str::from_utf8(&output.stdout)
                .expect("invalid utf8 sequence")
                .trim(),
        )
    }
}

impl Defaults {
    pub fn exec_confirmation() -> String {
        DmenuCmd::new(
            vec!["No".to_string(), "Yes".to_string()],
            "Confirm? ".to_string(),
        )
        .exec()
    }

    pub fn exec_no_layouts_found() {
        DmenuCmd::new(vec![], "No layouts found".to_string()).exec();
    }
}
