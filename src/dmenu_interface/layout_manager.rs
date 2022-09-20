// TODO: rewrite this code into smaller parts, rewrite partial code parts in more convenient way
use super::layout_creator;
use crate::cmd::{
    dmenu::{DmenuCmd, DmenuDefaults},
    term, xrandr,
};
use crate::layouts_config::LayoutsConfig;
use crate::monitor_layout::{
    Monitor, MonitorDuplicated, MonitorLayout, MonitorLayouts, MonitorPosition, MonitorSettings,
};
use std::collections::HashMap;

impl MonitorSettings {
    fn is_empty(&self) -> bool {
        self.resolutions.is_empty()
    }
}

pub fn auto_detect_layout() {
    unimplemented!();
}

fn parse_available_monitors() -> Vec<String> {
    xrandr::get_list_of_monitors()
        .split('\n')
        .map(|line| String::from(line.split_whitespace().collect::<Vec<&str>>()[0]))
        .collect()
}

fn parse_res_and_rate(res_and_rate: &str) -> Vec<&str> {
    res_and_rate
        .split_whitespace()
        .take(2)
        .collect::<Vec<&str>>()
}

fn parse_res(res: &str) -> Vec<u16> {
    res.split('x')
        .take(2)
        .flat_map(|res_px| res_px.parse::<u16>())
        .collect::<Vec<u16>>()
}

fn parse_display_option(monitor_line: &str) -> Option<(&str, &str)> {
    if let [res, rate] = parse_res_and_rate(monitor_line)[..] {
        if parse_res(res).len() != 2 {
            return None;
        }
        if let Err(err) = rate.parse::<f32>() {
            return None;
        }
        return Some((res, rate));
    }
    None
}

fn parse_available_monitor_options() -> HashMap<String, MonitorSettings> {
    let mut monitor_settings: HashMap<String, MonitorSettings> = HashMap::new();
    let display_options = xrandr::get_displays();
    let display_options: Vec<&str> = display_options.split('\n').collect();

    let mut current_monitor_name = "";
    let mut current_settings = MonitorSettings::default();
    for display_option in display_options {
        if display_option.ends_with("connected") {
            if !current_settings.is_empty() {
                monitor_settings.insert(current_monitor_name.to_string(), current_settings);
                current_settings = MonitorSettings::default();
            }
            current_monitor_name = display_option.split_whitespace().collect::<Vec<&str>>()[0];
            continue;
        }
        if let Some((res, rate)) = parse_display_option(display_option) {
            current_settings.resolutions.push(res.to_string());
            current_settings.rates.push(rate.to_string());
        }
    }
    monitor_settings.insert(current_monitor_name.to_string(), current_settings);
    monitor_settings
}

pub fn create_new_layout(user_layouts: &MonitorLayouts) {
    if DmenuDefaults::exec_inherit_layout() == "Yes" {
        unimplemented!();
    }

    let mut name_to_monitor = parse_available_monitor_options();
    let mut monitor_layout = MonitorLayout::default();
    let mut is_continue = true;
    let mut is_primary_selected = false;
    while is_continue && !name_to_monitor.is_empty() {
        let conn_opt = DmenuCmd::new(
            &["Connect".to_string(), "Duplicate".to_string()],
            "Connect or duplicate monitor? ".to_string(),
        )
        .exec();

        let selected_monitor_name = DmenuCmd::new(
            &name_to_monitor.keys().cloned().collect::<Vec<String>>(),
            format!("Which monitor to add ({})? ", &conn_opt.to_lowercase()),
        )
        .exec();

        let selected_monitor = &name_to_monitor[&selected_monitor_name];
        let selected_res = DmenuCmd::new(
            &selected_monitor.resolutions,
            format!("Which resolution for {}? ", selected_monitor_name),
        )
        .exec();
        let selected_rate = DmenuCmd::new(
            &selected_monitor.rates,
            format!("Which rate for {}? ", selected_monitor_name),
        )
        .exec();

        let mut is_primary = false;
        if !is_primary_selected {
            is_primary = DmenuDefaults::exec_is_primary();
            is_primary_selected = is_primary;
        }

        let mut monitor_position = MonitorPosition::default();
        if !monitor_layout.monitors.is_empty() {
            match DmenuDefaults::exec_position(&selected_monitor_name).as_str() {
                "Skip" => {}
                pos => {
                    let related_monitor = DmenuCmd::new(
                        &monitor_layout.monitor_names(),
                        format!("Place {} {} which monitor? ", &selected_monitor_name, &pos),
                    )
                    .exec();
                    monitor_position = MonitorPosition::related(pos, &related_monitor);
                }
            }
        }

        if DmenuDefaults::confirmed() {
            name_to_monitor.remove(&selected_monitor_name);
            if let [height_px, width_px] = &selected_res
                .split('x')
                .map(|x| x.parse::<u16>().expect("Parsed monitor resolution"))
                .collect::<Vec<u16>>()[..]
            {
                monitor_layout.monitors.push(Monitor {
                    name: selected_monitor_name,
                    rate: selected_rate
                        .parse::<f32>()
                        .unwrap_or_else(|_| panic!("Parsed monitor rate {}", &selected_rate)),
                    height_px: *height_px,
                    width_px: *width_px,
                    pos: monitor_position,
                    is_auto: true,
                    is_primary,
                    dupl: MonitorDuplicated::default(),
                })
            }
        }
        is_continue = DmenuDefaults::exec_continue();
        if !is_continue {
            monitor_layout.name =
                DmenuCmd::new(&[], String::from("Choose the name for your layout: ")).exec();
        }
    }

    LayoutsConfig::add(MonitorLayouts {
        layouts: vec![monitor_layout],
    });
}

pub fn disconnect_all_monitors() {
    unimplemented!();
}

pub fn remove_layout(user_layouts: &mut MonitorLayouts) {
    if user_layouts.is_empty() {
        return DmenuDefaults::exec_no_layouts_found();
    }
    let layout_name = DmenuDefaults::exec_layout_to_remove(&user_layouts.names());
    match user_layouts.find_layout(&layout_name) {
        Ok(pos) if DmenuDefaults::confirmed() => user_layouts.remove_layout(pos),
        Err(err) => DmenuDefaults::exec_err(err, &layout_name),
        _ => {}
    };
}

pub fn apply_layout(user_layouts: &MonitorLayouts, layout_name: &str) {
    match user_layouts.get_layout_by(layout_name) {
        Ok(layout) if DmenuDefaults::confirmed() => layout.apply(),
        Err(err) => DmenuDefaults::exec_err(err, layout_name),
        _ => {}
    };
}
