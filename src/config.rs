pub struct Config;

// TODO: find the way to store default configuration in another way
impl Config {
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
}
