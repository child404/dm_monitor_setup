// TODO: rewrite modules use the latest replacement for mod.rs
// NOTE: the structure would be:
//  - '--daemon' - runs daemon
//  - '--help' - spawns help (usage)
//  - no args - launches the dmenu_ms interface with opts:
//      - Auto-detect layout:
//      - Disconnect all:
//          - disconnect all monitors except in-build monitor (if laptop)
//          - disconnect all monitors except primary monitor (if PC setup)
//      - Create new layout: run steps to create new layout
//      - Remove layout: delete selected layout from monitor_layouts.toml
//      - Exit: exits an app with exit(0)
//      - layout_name: applies selected layout

// dmenu_ms:
//      main;
//      core:
//          daemon;
//          layout_tools:
//              monitor;
//              monitor_layout;
//              layout_config;
//              screen_options;
//              layout_errors;
//          layout_manager:
//              layout_creator;
//              layout_detector;
//          tools:
//              term: TermCMD;
//              xrandr: XrandrCMD;
//              term_error: from custom_errors;
//              xrandr_error: from custom errors;
//          dmenu:
//              ui: includes dmenu_cmd::Defaults, but ui::DmenuUI;
//              cmd: DmenuCMD;
//              dmenu_error;
//      help;
//      params;

use crate::core::daemon::MSDaemon;
use crate::dmenu_interface::dmenu_ms::{launch_ms, spawn_help};

pub mod cmd;
pub mod core;
pub mod custom_errors;
pub mod dmenu_interface;
pub mod layouts_config;
pub mod monitor_layout;
pub mod params;
pub mod screen_opts;
use std::env;

fn main() {
    match env::args().nth(1) {
        Some(arg) => match arg.as_str() {
            "--daemon" | "-d" => MSDaemon::start(),
            "--help" | "-h" => spawn_help(),
            _ => println!("Undefined arg: {arg}"),
        },
        None => launch_ms(),
    }
}
