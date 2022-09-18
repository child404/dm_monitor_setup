use crate::monitor_layout::{Monitor, MonitorLayout};
use std::fmt::Write;

pub fn from_monitor_layout(monitor_layout: &MonitorLayout) -> String {
    let mut cmd = String::from("xrandr");
    for monitor in monitor_layout.monitors.iter() {
        write!(&mut cmd, " {}", _monitor_output_from(monitor)).unwrap();
    }
    cmd
}

pub fn get_list_of_monitors() -> &'static str {
    "xrandr | grep \" connected\""
}

fn _monitor_output_from(monitor: &Monitor) -> String {
    let mut output = format!(
        "--output {} --mode {}x{} --rate {}",
        monitor.name, monitor.height_px, monitor.width_px, monitor.rate
    );
    if monitor.is_auto {
        output += " --auto";
    }
    if monitor.pos.is_related {
        write!(
            &mut output,
            " --{} {}",
            monitor.pos.related_pos, monitor.pos.related_name
        )
        .unwrap();
    }
    if monitor.dupl.is_duplicated {
        write!(&mut output, " --same-as {}", monitor.dupl.name).unwrap();
    }
    if monitor.is_primary {
        output += " --primary";
    }
    output
}
