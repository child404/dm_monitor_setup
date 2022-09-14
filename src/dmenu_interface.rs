use std::process::{Command, Stdio};
use std::str;

pub struct DmenuCmd {
    pub bin: String,
    pub params: Vec<String>,
    pub options: Vec<String>,
    pub prompt_message: String,
}

impl DmenuCmd {
    pub fn new(options: Vec<String>, prompt_message: String) -> Self {
        DmenuCmd {
            bin: "dmenu".to_string(),
            params: vec!["-c".to_string(), "-l 5".to_string(), "-bw 1".to_string()],
            options,
            prompt_message,
        }
    }

    pub fn exec(&self) -> String {
        let command = format!(
            "printf \"{}\" | {} {} -p \"{}\"",
            self.options.join("\n"),
            self.bin,
            self.params.join(" "),
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
