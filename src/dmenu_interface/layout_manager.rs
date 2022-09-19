use crate::cmd::{
    dmenu::{Defaults, DmenuCmd},
    term, xrandr,
};
use crate::layouts_config::LayoutsConfig;
use crate::monitor_layout::{
    Monitor, MonitorDuplicated, MonitorLayout, MonitorLayouts, MonitorPosition,
};
use std::collections::HashMap;

pub fn auto_detect_layout() {
    unimplemented!();
}

fn parse_available_monitors() -> Vec<String> {
    xrandr::get_list_of_monitors()
        .split('\n')
        .map(|line| String::from(line.split_whitespace().collect::<Vec<&str>>()[0]))
        .collect()
}

fn parse_resolution(res_px: &str) -> u16 {
    unimplemented!();
}

fn parse_display_option(monitor_line: &str) -> Option<(u16, u16, f32)> {
    if let [res, rate] = &monitor_line
        .split_whitespace()
        .take(2)
        .collect::<Vec<&str>>()[..]
    {
        if let [height_px, width_px] = res
            .split('x')
            .take(2)
            .flat_map(|res_px| res_px.parse::<u16>())
            .collect::<Vec<u16>>()[..]
        {
            if let Ok(rate) = rate.parse::<f32>() {
                return Some((height_px, width_px, rate));
            }
        }
    }
    None
}

fn parse_available_monitor_options() -> HashMap<String, Vec<Monitor>> {
    let mut name_to_monitor: HashMap<String, Vec<Monitor>> = HashMap::new();
    let display_options = xrandr::get_displays();
    let display_options: Vec<&str> = display_options.split('\n').collect();

    let mut current_monitor_name = "";
    let mut current_monitors: Vec<Monitor> = Vec::new();
    for display_option in display_options {
        if display_option.ends_with("connected") {
            if !current_monitors.is_empty() {
                name_to_monitor.insert(
                    current_monitor_name.to_string(),
                    current_monitors[..10].to_vec(),
                );
                current_monitors = Vec::<Monitor>::new();
            }
            current_monitor_name = display_option.split_whitespace().collect::<Vec<&str>>()[0];
            continue;
        }
        if let Some((height_px, width_px, rate)) = parse_display_option(display_option) {
            current_monitors.push(Monitor {
                name: current_monitor_name.to_string(),
                height_px,
                width_px,
                rate,
                is_auto: false,
                is_primary: false,
                dupl: MonitorDuplicated::not_duplicated(),
                pos: MonitorPosition::not_related(),
            })
        }
    }
    name_to_monitor.insert(
        current_monitor_name.to_string(),
        current_monitors[..3].to_vec(),
    );
    name_to_monitor
}

pub fn create_new_layout(user_layouts: &MonitorLayouts) {
    if Defaults::exec_inherit_layout() == "Yes" {
        unimplemented!();
    }

    DmenuCmd::new(
        &["Connect".to_string(), "Duplicate".to_string()],
        "Connect or duplicate monitor? ".to_string(),
    )
    .exec();

    let name_to_monitor = parse_available_monitor_options();

    DmenuCmd::new(
        &Vec::from_iter(name_to_monitor.keys()),
        "Which monitor to add? ".to_string(),
    )
    .exec();

    unimplemented!();
}

pub fn disconnect_all_monitors() {
    unimplemented!();
}

pub fn remove_layout(user_layouts: &mut MonitorLayouts) {
    if user_layouts.is_empty() {
        return Defaults::exec_no_layouts_found();
    }
    let layout_name = Defaults::exec_layout_to_remove(&user_layouts.names());
    match user_layouts.find_layout(&layout_name) {
        Ok(pos) if Defaults::confirmed() => user_layouts.remove_layout(pos),
        Err(err) => Defaults::exec_err(err, &layout_name),
        _ => {}
    };
}

pub fn apply_layout(user_layouts: &MonitorLayouts, layout_name: &str) {
    match user_layouts.get_layout_by(layout_name) {
        Ok(layout) if Defaults::confirmed() => layout.apply(),
        Err(err) => Defaults::exec_err(err, layout_name),
        _ => {}
    };
}
