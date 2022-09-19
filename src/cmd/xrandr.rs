use crate::cmd::term;
use crate::monitor_layout::{Monitor, MonitorLayout};

pub fn from_monitor_layout(monitor_layout: &MonitorLayout) -> String {
    format!("xrandr {}", monitor_layout.to_string())
}

pub fn get_list_of_monitors() -> String {
    match term::exec_with_output("xrandr | grep \" connected\"") {
        Ok(output) => output,
        Err(err) => "".to_string(),
    }
}

pub fn get_displays() -> String {
    match term::exec_with_output("xrandr | grep -Ev \"disconnected|Screen\" | awk '{print $1, $2}' | awk -F'[/+* ]' '{print $1\" \"$2}'") {
        Ok(output) => output,
        Err(err) => "".to_string()
    }
}
