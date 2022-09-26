use crate::core::handlers::dmenu::DmenuCMD;

pub struct DmenuUI;

impl DmenuUI {
    pub fn exec_confirm() -> String {
        DmenuUI::exec_with_yes_no("Confirm? ")
    }

    pub fn exec_is_inherit_layout() -> bool {
        DmenuUI::exec_with_yes_no("Inherit existing layout? ") == "Yes"
    }

    fn exec_with_yes_no(prompt_message: &str) -> String {
        DmenuCMD::new(
            &["No".to_string(), "Yes".to_string()],
            prompt_message.to_string(),
        )
        .exec()
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

    fn exec_with_skip(msg: &str, default_opts: Option<&[String]>) -> String {
        let mut opts = vec!["Skip".to_string()];
        if let Some(def_opts) = default_opts {
            opts = [def_opts, &opts].concat();
        }
        DmenuCMD::new(&opts, msg.to_string()).exec()
    }

    pub fn exec_position(monitor_name: &str) -> String {
        DmenuUI::exec_with_skip(
            &format!("Where to place the monitor {}? ", monitor_name),
            Some(&Params::monitor_positions()),
        )
    }

    fn exec_with_goback(msg: &str, default_opts: Option<&[String]>) {
        let mut opts = vec!["← Go back".to_string()];
        if let Some(def_opts) = default_opts {
            opts = [def_opts, &opts].concat();
        }
        DmenuCMD::new(&opts, msg.to_string()).exec();
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

    pub fn exec_layout_to_remove(options: &[String]) -> String {
        DmenuCMD::new(options, "Which layout to remove? ".to_string()).exec()
    }

    pub fn exec_err(err: LayoutError, err_msg: &str) {
        DmenuUI::exec_with_goback(&format!("{}: {}", err, err_msg), None);
    }

    pub fn exec_start(layout_names: &[String]) -> String {
        DmenuCMD::new(
            &[layout_names, &Params::start_options()].concat(),
            "Choose an option: ".to_string(),
        )
        .exec()
    }
}
