use crate::cmd::term;
use crate::custom_errors::LayoutError;
use crate::params::Params;
use std::process::exit;
use std::str;

pub struct DmenuCmd {
    pub executable: String,
    pub args: Vec<String>,
    pub prompt_options: Vec<String>,
    pub prompt_message: String,
}

pub struct Defaults;

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
        let result = term::exec_with_output(&format!(
            "printf \"{}\" | {} {} -p \"{}\"",
            self.prompt_options.join("\n"),
            self.executable,
            self.args.join(" "),
            self.prompt_message
        ));
        if result.is_empty() {
            // handle Esc or ^[ to quit the program
            exit(0);
        }
        result
    }
}

impl Defaults {
    pub fn exec_confirm() -> String {
        Defaults::exec_with_yes_no("Confirm? ")
    }

    pub fn exec_inherit_layout() -> String {
        Defaults::exec_with_yes_no("Inherit existing layout? ")
    }

    fn exec_with_yes_no(prompt_message: &str) -> String {
        DmenuCmd::new(
            &["No".to_string(), "Yes".to_string()],
            prompt_message.to_string(),
        )
        .exec()
    }

    pub fn confirmed() -> bool {
        Defaults::exec_confirm() == "Yes"
    }

    fn exec_with_goback(msg: &str, default_opts: &[String]) {
        DmenuCmd::new(
            &[default_opts, &["← Go back".to_string()]].concat(),
            msg.to_string(),
        )
        .exec();
    }

    pub fn exec_no_layouts_found() {
        Defaults::exec_with_goback("No layouts found ", &[]);
    }

    pub fn exec_layout_to_remove(options: &[String]) -> String {
        DmenuCmd::new(options, "Which layout to remove? ".to_string()).exec()
    }

    pub fn exec_err(err: LayoutError, err_msg: &str) {
        Defaults::exec_with_goback(&format!("{}: {}", err, err_msg), &[]);
    }

    pub fn exec_start(layout_names: &[String]) -> String {
        DmenuCmd::new(
            &[layout_names, &Params::start_options()].concat(),
            "Choose an option: ".to_string(),
        )
        .exec()
    }
}
