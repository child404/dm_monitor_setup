#![allow(dead_code)]
use std::process::exit;

use crate::dmenu_cmd;
use crate::monitor_setup::{
    Monitor, MonitorDuplicated, MonitorLayout, MonitorLayouts, MonitorPosition,
};

pub fn run_daemon() {
    loop {
        unimplemented!();
    }
}

pub fn launch_ms() {
    // TODO: move everything in dmenu_interface module with Defaults from dmenu_cmd
    let mut user_layouts = MonitorLayouts::from_config();
    let layout_names = user_layouts.names();
    let start_options = [
        layout_names.clone(),
        vec![
            "Auto-detect".to_string(),
            "Disconnect all".to_string(),
            "Create new setup".to_string(),
            "Remove setup".to_string(),
            "Exit".to_string(),
        ],
    ]
    .concat();
    let dmenu_cmd_ = dmenu_cmd::DmenuCmd::new(start_options, "Choose an option: ".to_string());
    match dmenu_cmd_.exec().as_str() {
        "" => exit(0), // handle Esc or ^[ to quit the program
        "Auto-detect" => println!("Detecting setup..."),
        "Disconnect all" => println!("Disconnecting all displays..."),
        "Create new setup" => println!("Creating new setup..."),
        "Remove setup" => {
            if user_layouts.is_empty() {
                dmenu_cmd::Defaults::exec_no_layouts_found();
                launch_ms();
                return;
            }
            let layout_to_remove = dmenu_cmd::DmenuCmd::new(
                layout_names.clone(),
                "Which layout to remove? ".to_string(),
            )
            .exec();
            if !layout_names.contains(&layout_to_remove)
                || dmenu_cmd::Defaults::exec_confirmation() == "No"
            {
                launch_ms();
                return;
            }
            user_layouts.remove_layout(layout_to_remove);
            launch_ms();
        }
        "Exit" => exit(0),
        layout_name => {
            if !layout_names.contains(&layout_name.to_string())
                || dmenu_cmd::Defaults::exec_confirmation() == "No"
            {
                launch_ms();
                return;
            }
            match user_layouts.get_layout_by(layout_name.to_string()) {
                Ok(layout) => layout.apply(),
                Err(_) => {
                    launch_ms();
                }
            }
        }
    }
}

pub fn spawn_help() {
    println!("usage: dmenu_ms [options]");
    println!("  where options are:",);
    println!("  --help to get help\n\tor -h");
    println!("  --daemon to run daemon\n\tor -d");
}

fn test_monitor_setup() {
    let monitor = Monitor {
        name: "eDP-1".to_string(),
        height_px: 1920,
        width_px: 1200,
        rate: 120,
        is_primary: true,
        is_auto: true,
        pos: MonitorPosition {
            is_related: false,
            related_pos: "".to_string(),
            related_name: "".to_string(),
        },
        dupl: MonitorDuplicated {
            is_duplicated: false,
            name: "".to_string(),
        },
    };
    let monitor_setup = MonitorLayout {
        name: "Single monitor".to_string(),
        monitors: vec![monitor],
    };
    let setups = MonitorLayouts {
        layouts: vec![monitor_setup],
    };
    println!("{:#?}", MonitorLayouts::from_config());
    println!("{:#?}", setups);

    setups.add_to_config();
}
