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
//          daemon - runs with the flag `--daemon` (`-d`),
//                   constantly parses xrandr command's output to store screen opts/other settings,
//                   and every ~2 seconds checks whether there's a new monitor connected;
//          utils - different utils for monitor/layouts:
//              monitor;
//              monitor_layout;
//              layout_config;
//              monitor_options (previously - screen_options);
//              layout_error (errors for MonitorLayout);
//              monitor_error (errors for monitor/screen);
//          ms_launcher - runs with no flags:
//              layout_manager - manages create, remove, auto-detect layout, etc.;
//              layout_creator;
//              layout_detector;
//          handlers - the handlers for terminal, xrandr and dmenu commands:
//              term (or term_handler): TermCMD;
//              xrandr (or xrandr_handler): XrandrCMD;
//              dmenu (or dmenu_handler): DmenuCMD;
//              term_error: from custom_errors;
//              xrandr_error: from custom errors;
//              dmenu_error;
//      ui - includes dmenu_cmd::Defaults, but ui::DmenuUI (maybe, different files for different modules);
//      help - help function(s) that run with `--help` (`-h`) flag;
//      defaults (or other name) - const params (rewrite it as consts);

pub mod core;
pub mod defaults;
pub mod help;
pub mod ui;

use crate::core::{daemon::MSDaemon, ms_launcher};
use std::env;

fn main() {
    match env::args().nth(1) {
        Some(arg) => match arg.as_str() {
            "--daemon" | "-d" => MSDaemon::start(),
            "--help" | "-h" => help::help(),
            _ => println!("Undefined arg: {arg}"),
        },
        None => ms_launcher::launch(),
    }
}
