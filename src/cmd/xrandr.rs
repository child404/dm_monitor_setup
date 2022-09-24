use regex::Regex;
use std::collections::HashMap;

use crate::{cmd::term::TermCmd, monitor_layout::MonitorLayout, screen_opts::ScreenOptions};

pub struct XrandrCmd;

impl XrandrCmd {
    pub fn from_monitor_layout(layout: &MonitorLayout) -> String {
        format!("xrandr {}", layout.to_string())
    }

    pub fn get_list_of_monitors() -> Option<Vec<String>> {
        match TermCmd::exec_with_output("xrandr | grep \" connected\"") {
            Ok(output) => Some(
                output
                    .split('\n')
                    .flat_map(|line| {
                        line.split_whitespace()
                            .take(1)
                            .next()
                            .map(|screen| screen.to_string())
                    })
                    .collect(),
            ),
            Err(err) => None,
        }
    }

    fn _get_display_options() -> Option<String> {
        TermCmd::exec_with_output(
            "xrandr | grep -Ev \"disconnected|Screen\" | awk '{print $1, $2}' | awk -F'[/+* ]' '{print $1\" \"$2}'").ok()
    }

    pub fn get_display_options() -> Option<HashMap<String, ScreenOptions>> {
        let screens_regexp =
            Regex::new(r"(.+) connected\n(?:[\da-zA-Z]+x[\da-zA-Z]+ [\da-zA-Z]+\.[\da-zA-Z]+\n)+")
                .unwrap();
        Self::_get_display_options().map(|opts| {
            HashMap::from_iter(
                screens_regexp
                    .captures_iter(&opts)
                    .map(|screen| (screen[1].to_string(), screen[0].parse().ok().unwrap())),
            )
        })
    }
}
