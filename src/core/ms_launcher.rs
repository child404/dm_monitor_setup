mod layout_creator;
mod layout_detector;
mod layout_manager;

use std::process::exit;

use crate::cmd::dmenu::DmenuDefaults;
use layout_manager::LayoutManager;

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
