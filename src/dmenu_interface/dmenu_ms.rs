#![allow(dead_code)]
use std::process::exit;
use std::{thread, time};

use super::layout_manager::LayoutManager;
use crate::cmd::dmenu::DmenuDefaults;
use crate::monitor_layout::MonitorLayouts;
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
    let mut user_layouts = MonitorLayouts::from_config();
    match DmenuDefaults::exec_start(&user_layouts.names()).as_str() {
        "Auto-detect" => LayoutManager::auto_detect_layout(),
        "Disconnect all" => LayoutManager::disconnect_all_monitors(),
        "Create new layout" => LayoutManager::create_new_layout(&user_layouts),
        "Remove layout" => LayoutManager::remove_layout(&mut user_layouts),
        "Exit" => exit(0),
        layout_name => LayoutManager::apply_layout(&user_layouts, layout_name),
    }
    launch_ms()
}

pub fn spawn_help() {
    println!("usage: dmenu_ms [options]");
    println!("  where options are:",);
    println!("  --help to get help\n\tor -h");
    println!("  --daemon to run daemon\n\tor -d");
}
