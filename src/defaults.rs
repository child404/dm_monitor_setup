// TODO: move this func to layout_config.rs
fn find_path_to_config() -> String {
    match dirs::home_dir() {
        // TODO: add check for monitor_setups.toml existance here
        Some(home_dir) => {
            home_dir
                .into_os_string()
                .into_string()
                .expect("Valid home dir path")
                + PATH_TO_CONFIG
        }
        // TODO: change panic! behavior to just notify that config can't be found
        //       or create that filepath
        None => panic!("Cannot find home dir"),
    }
}

fn find_dmenu_binary() -> String {
    unimplemented!();
}

const PATH_TO_CONFIG: &str = "/.config/dmenu_ms/monitor_layouts.toml";

const PATH_TO_CURRENT_LAYOUT: &str = "/tmp/current_layout.toml";
const PATH_TO_DETECTED_MONITORS: &str = "/tmp/detected_monitor_opts.toml";

const DMENU_ARGS: [&str; 3] = ["-l 5", "-g 1", "-p"];
const DMENU_BINARY: &str = "dmenu";
const XRANDR_BINARY: &str = "xrandr";
// TODO: add here binary recognition instead of hard-coded potential paths
const BINARY_PATHS: [&str; 2] = ["usr/bin", "/usr/local/bin/dmenu"];

const MONITOR_POSITIONS: [&str; 4] = ["left-of", "right-of", "above", "below"];
const DAEMON_SLEEP_TIME_MILLIS: usize = 2 * 1000;
const MS_LAUNCHER_START_OPTS: [&str; 5] = [
    "Auto-detect",
    "Disconnect all",
    "Create new layout",
    "Remove layout",
    "Exit",
];
