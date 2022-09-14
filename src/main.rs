#![allow(unused_imports, unused_variables)]
mod dmenu_interface;
mod monitor_setup;
use dmenu_interface::DmenuCmd;
use monitor_setup::{Monitor, MonitorDuplicated, MonitorPosition, MonitorSetup, MonitorSetups};
// use winit::event_loop::EventLoop;
// use xrandr::XHandle;

fn main() {
    // let monitors = XHandle::open().expect("").all_outputs().expect("");
    // println!("{:#?}", monitors);

    // let event_loop = EventLoop::new();
    // let monitors = event_loop.available_monitors();
    // for monitor in monitors {
    //     println!("{:#?}", monitor);
    // }
    // let monitor = Monitor {
    //     name: "eDP-1".to_string(),
    //     height_px: 1920,
    //     width_px: 1200,
    //     rate: 120,
    //     pos: MonitorPosition {
    //         is_related: false,
    //         related_pos: "".to_string(),
    //         related_name: "".to_string(),
    //     },
    //     dupl: MonitorDuplicated {
    //         is_duplicated: false,
    //         name: "".to_string(),
    //     },
    //     is_primary: true,
    // };
    // let monitor_setup = MonitorSetup {
    //     name: "Single monitor".to_string(),
    //     monitors: vec![monitor],
    // };
    // let setups = MonitorSetups {
    //     setup: vec![monitor_setup],
    // };
    // println!("{:#?}", MonitorSetups::from_config());
    // println!("{:#?}", setups);

    // setups.to_toml();

    let user_setups = MonitorSetups::from_config();
    let start_options = [
        user_setups.names(),
        vec![
            "Auto-detect".to_string(),
            "Disconnect all".to_string(),
            "Create new setup".to_string(),
            "Remove setup".to_string(),
        ],
    ]
    .concat();
    match DmenuCmd::new(start_options, "Choose an option: ".to_string())
        .exec()
        .as_str()
    {
        "Auto-detect" => println!("Detecting setup..."),
        "Disconnect all" => println!("Disconnecting all displays..."),
        "Create new setup" => println!("Creating new setup..."),
        "Remove setup" => println!("Removing setup"),
        _ => {}
    }
}
