pub struct Params;

// TODO: find the way to store default configuration in another way
impl Params {
    pub fn path_to_config() -> String {
        match dirs::home_dir() {
            Some(home_dir) => {
                home_dir
                    .into_os_string()
                    .into_string()
                    .expect("Valid home dir path")
                    + "/.config/dmenu_ms/monitor_setups.toml"
            }
            None => panic!("Cannot find home dir"),
        }
    }

    pub fn monitor_positions() -> Vec<String> {
        vec![
            "left-of".to_string(),
            "right-of".to_string(),
            "above".to_string(),
            "below".to_string(),
        ]
    }

    pub fn dmenu_executable() -> String {
        String::from("dmenu")
    }

    pub fn dmenu_args() -> Vec<String> {
        vec!["-l 5".to_string(), "-g 1".to_string(), "-p".to_string()]
    }

    pub fn daemon_sleep_time_millis() -> u64 {
        3 * 1000 // 3 seconds
    }

    pub fn start_options() -> Vec<String> {
        vec![
            "Auto-detect".to_string(),
            "Disconnect all".to_string(),
            "Create new layout".to_string(),
            "Remove layout".to_string(),
            "Exit".to_string(),
        ]
    }

    pub fn path_to_current_layout() -> String {
        String::from("/tmp/current_layout.toml")
    }
}
