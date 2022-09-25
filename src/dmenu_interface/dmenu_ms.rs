use std::process::exit;
use std::{thread, time};

use super::layout_manager::LayoutManager;
use crate::cmd::dmenu::DmenuDefaults;
use crate::params::Params;

pub fn run_daemon() {
    loop {
        let sleep_time = time::Duration::from_millis(Params::daemon_sleep_time_millis());
        thread::sleep(sleep_time);
        unimplemented!();
    }
}

pub fn launch_ms() {
    // TODO: add current layout and add layout_name (Current) or ✓
    // FIXME: LayoutManager wraps LayoutsConfig, LayoutsConfig wraps MonitorLayouts from .toml file
    //        implement remove_layout, etc. in LayoutsConfig
    let mut layout_manager = LayoutManager::default();
    match DmenuDefaults::exec_start(&layout_manager.user_layouts_names()).as_str() {
        "Auto-detect" => layout_manager.auto_detect_layout(),
        "Disconnect all" => layout_manager.disconnect_all_monitors(),
        "Create new layout" => layout_manager.create_new_layout(),
        "Remove layout" => layout_manager.remove_layout(),
        "Exit" => exit(0),
        layout_name => layout_manager.apply_layout(layout_name),
    }
    launch_ms()
}

pub fn spawn_help() {
    println!("usage: dmenu_ms [options]");
    println!("  where options are:",);
    println!("  --help to get help\n\tor -h");
    println!("  --daemon to run daemon\n\tor -d");
}
