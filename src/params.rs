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

    pub fn dmenu_executable() -> String {
        String::from("dmenu")
    }

    pub fn dmenu_args() -> Vec<String> {
        vec!["-c".to_string(), "-l 5".to_string(), "-bw 1".to_string()]
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
}
