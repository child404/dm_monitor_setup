use crate::cmd::{dmenu, term, xrandr};
use crate::layouts_config::LayoutsConfig;
use crate::monitor_layout::{MonitorLayout, MonitorLayouts};

pub fn auto_detect_layout() {
    unimplemented!();
}

pub fn create_new_layout(user_layouts: &MonitorLayouts) {
    if dmenu::Defaults::exec_inherit_layout() == "Yes" {
        unimplemented!();
    }
    let monitors = term::exec_with_output(xrandr::get_list_of_monitors());
    let monitors: Vec<&str> = monitors.split('\n').collect();
    println!("{:#?}", monitors);
    unimplemented!();
}

pub fn disconnect_all_monitors() {
    unimplemented!();
}

pub fn remove_layout(user_layouts: &mut MonitorLayouts) {
    if user_layouts.is_empty() {
        return dmenu::Defaults::exec_no_layouts_found();
    }
    let layout_name = dmenu::Defaults::exec_layout_to_remove(&user_layouts.names());
    match user_layouts.find_layout(&layout_name) {
        Ok(pos) if dmenu::Defaults::confirmed() => user_layouts.remove_layout(pos),
        Err(err) => dmenu::Defaults::exec_err(err, &layout_name),
        _ => {}
    };
}

pub fn apply_layout(user_layouts: &MonitorLayouts, layout_name: &str) {
    match user_layouts.get_layout_by(layout_name) {
        Ok(layout) if dmenu::Defaults::confirmed() => layout.apply(),
        Err(err) => dmenu::Defaults::exec_err(err, layout_name),
        _ => {}
    };
}
