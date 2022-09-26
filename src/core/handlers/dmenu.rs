use super::term::TermCMD;
use crate::defaults;
use std::process;
use std::str;

pub struct DmenuCMD<'a> {
    pub executable: &'a str,
    pub args: &'a [&'a str],
    pub prompt_options: &'a [&'a str],
    pub prompt_message: &'a str,
}

impl<'a> DmenuCMD<'a> {
    pub fn new(prompt_options: &'a [&'a str], prompt_message: &'a str) -> Self {
        Self {
            executable: defaults::DMENU_BINARY,
            args: &defaults::DMENU_ARGS,
            prompt_options,
            prompt_message,
        }
    }

    pub fn exec(&self) -> String {
        if let Ok(output) = TermCMD::exec_with_output(&self.to_string()) {
            // if self.prompt_options.is_empty() || self.prompt_options.contains(&output) {
            return output;
            // }
        }
        // handle Esc or ^[ to quit the dmenu
        process::exit(0);
    }
}

impl ToString for DmenuCMD<'_> {
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
