use crate::{
    core::{handlers::dmenu::Dmenu, utils::layout_error::LayoutError},
    defaults,
};
use std::process;

pub struct DmenuUI;

impl DmenuUI {
    fn exec_confirm() -> String {
        DmenuUI::exec_with_yes_no("Confirm? ")
    }

    pub fn exec_is_inherit_layout() -> bool {
        DmenuUI::exec_with_yes_no("Inherit existing layout? ") == "Yes"
    }

    fn exec_with_yes_no(prompt_message: &str) -> String {
        Dmenu::new(&["No", "Yes"], prompt_message)
            .run_and_fetch_output()
            .unwrap_or("No".to_string())
    }

    pub fn exec_continue() -> bool {
        DmenuUI::exec_with_yes_no("Continue? ") == "Yes"
    }

    pub fn confirmed() -> bool {
        DmenuUI::exec_confirm() == "Yes"
    }

    pub fn exec_is_primary() -> bool {
        DmenuUI::exec_with_yes_no("Is primary? ") == "Yes"
    }

    fn exec_with_skip(msg: &str, prompt_options: Option<&[&str]>) -> String {
        Dmenu::new(&[prompt_options.unwrap_or(&[]), &["Skip"]].concat(), msg)
            .run_and_fetch_output()
            .unwrap_or("Skip".to_string())
    }

    pub fn exec_position(monitor_name: &str) -> String {
        DmenuUI::exec_with_skip(
            &format!("Where to place the monitor {}? ", monitor_name),
            Some(&defaults::MONITOR_POSITIONS),
        )
    }

    fn exec_with_goback(msg: &str, default_opts: Option<&[&str]>) {
        Dmenu::new(&[default_opts.unwrap_or(&[]), &["← Go back"]].concat(), msg)
            .run_and_fetch_output();
    }

    pub fn exec_overwrite_layout(layout_name: &str) -> bool {
        DmenuUI::exec_with_yes_no(&format!(
            "Layout {} already exists! Overwrite? ",
            layout_name
        )) == "Yes"
    }

    pub fn exec_no_layouts_found() {
        DmenuUI::exec_with_goback("No layouts found ", None);
    }

    pub fn exec_layout_to_remove(layouts: &[&str]) -> String {
        Dmenu::new(layouts, "Which layout to remove? ")
            .run_and_fetch_output()
            .unwrap_or(String::new())
    }

    pub fn exec_err(err: LayoutError, err_msg: &str) {
        DmenuUI::exec_with_goback(&format!("{}: {}", err, err_msg), None);
    }

    pub fn exec_start(layout_names: &[&str]) -> String {
        Dmenu::new(
            &[layout_names, &defaults::MS_LAUNCHER_START_OPTS].concat(),
            "Choose an option: ",
        )
        .run_and_fetch_output()
        .unwrap_or_else(|error| {
            DmenuUI::exec_err(error, "");
            process::exit(1)
        })
    }
}
