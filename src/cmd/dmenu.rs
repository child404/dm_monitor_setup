use crate::cmd::term;
use crate::custom_errors::{LayoutError, TermOutputError};
use crate::params::Params;
use std::process;
use std::str;

pub struct DmenuCmd {
    pub executable: String,
    pub args: Vec<String>,
    pub prompt_options: Vec<String>,
    pub prompt_message: String,
}

pub struct DmenuDefaults;

impl DmenuCmd {
    pub fn new(prompt_options: &[String], prompt_message: String) -> Self {
        DmenuCmd {
            executable: Params::dmenu_executable(),
            args: Params::dmenu_args(),
            prompt_options: prompt_options.to_vec(),
            prompt_message,
        }
    }

    pub fn exec(&self) -> String {
        if let Ok(output) = term::exec_with_output(&self.to_string()) {
            return output;
        }
        // handle Esc or ^[ to quit the dmenu
        process::exit(0);
    }
}

impl ToString for DmenuCmd {
    fn to_string(&self) -> String {
        format!(
            "printf \"{}\" | {} {} -p \"{}\"",
            self.prompt_options.join("\n"),
            self.executable,
            self.args.join(" "),
            self.prompt_message
        )
    }
}

impl DmenuDefaults {
    pub fn exec_confirm() -> String {
        DmenuDefaults::exec_with_yes_no("Confirm? ")
    }

    pub fn exec_inherit_layout() -> String {
        DmenuDefaults::exec_with_yes_no("Inherit existing layout? ")
    }

    fn exec_with_yes_no(prompt_message: &str) -> String {
        DmenuCmd::new(
            &["No".to_string(), "Yes".to_string()],
            prompt_message.to_string(),
        )
        .exec()
    }

    pub fn exec_continue() -> bool {
        DmenuDefaults::exec_with_yes_no("Continue? ") == "Yes"
    }

    pub fn confirmed() -> bool {
        DmenuDefaults::exec_confirm() == "Yes"
    }

    pub fn exec_is_primary() -> bool {
        DmenuDefaults::exec_with_yes_no("Is primary monitor? ") == "Yes"
    }

    fn exec_with_skip(msg: &str, default_opts: Option<&[String]>) -> String {
        let mut opts = vec!["Skip".to_string()];
        if let Some(def_opts) = default_opts {
            opts = [def_opts, &opts].concat();
        }
        DmenuCmd::new(&opts, msg.to_string()).exec()
    }

    pub fn exec_position(monitor_name: &str) -> String {
        DmenuDefaults::exec_with_skip(
            &format!("Where to place the monitor {}? ", monitor_name),
            Some(&Params::monitor_positions()),
        )
    }

    fn exec_with_goback(msg: &str, default_opts: Option<&[String]>) {
        let mut opts = vec!["← Go back".to_string()];
        if let Some(def_opts) = default_opts {
            opts = [def_opts, &opts].concat();
        }
        DmenuCmd::new(&opts, msg.to_string()).exec();
    }

    pub fn exec_no_layouts_found() {
        DmenuDefaults::exec_with_goback("No layouts found ", None);
    }

    pub fn exec_layout_to_remove(options: &[String]) -> String {
        DmenuCmd::new(options, "Which layout to remove? ".to_string()).exec()
    }

    pub fn exec_err(err: LayoutError, err_msg: &str) {
        DmenuDefaults::exec_with_goback(&format!("{}: {}", err, err_msg), None);
    }

    pub fn exec_start(layout_names: &[String]) -> String {
        DmenuCmd::new(
            &[layout_names, &Params::start_options()].concat(),
            "Choose an option: ".to_string(),
        )
        .exec()
    }
}
