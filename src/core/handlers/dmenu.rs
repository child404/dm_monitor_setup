use crate::cmd::term::TermCmd;
use crate::custom_errors::LayoutError;
use crate::params::Params;
use std::process;
use std::str;

pub struct DmenuCMD {
    pub executable: String,
    pub args: Vec<String>,
    pub prompt_options: Vec<String>,
    pub prompt_message: String,
}

impl DmenuCMD {
    pub fn new(prompt_options: &[String], prompt_message: String) -> Self {
        DmenuCmd {
            executable: Params::dmenu_executable(),
            args: Params::dmenu_args(),
            prompt_options: prompt_options.to_vec(),
            prompt_message,
        }
    }

    pub fn exec(&self) -> String {
        if let Ok(output) = TermCmd::exec_with_output(&self.to_string()) {
            // if self.prompt_options.is_empty() || self.prompt_options.contains(&output) {
            return output;
            // }
        }
        // handle Esc or ^[ to quit the dmenu
        process::exit(0);
    }
}

impl ToString for DmenuCMD {
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
