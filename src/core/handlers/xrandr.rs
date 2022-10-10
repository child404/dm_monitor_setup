use regex::Regex;
use std::collections::HashMap;

use crate::core::{
    handlers::command_line as cmd,
    utils::{monitor_layout::Layout, monitor_options::MonitorOptions},
};

type DisplayToOptions = HashMap<String, MonitorOptions>;

pub struct Xrandr;

// TODO: change XrandrCMD to be similar to DmenuCMD
impl Xrandr {
    pub fn from_monitor_layout(layout: &Layout) -> String {
        format!("xrandr {}", layout.to_string())
    }

    fn parse_output_line(line: &str) -> Option<String> {
        line.split_whitespace().take(1).next().map(str::to_string)
    }

    pub fn get_list_of_monitors() -> Option<Vec<String>> {
        Some(
            cmd::run_and_fetch_output("xrandr | grep \" connected\"")
                .unwrap_or_else(|error| match error {
                    cmd::Error::EmptyOutput => unimplemented!(),
                    cmd::Error::Io(error) => unimplemented!(),
                })
                .split('\n')
                .flat_map(Self::parse_output_line)
                .collect(),
        )
    }

    pub fn get_display_modes() -> DisplayToOptions {
        let screens_regexp =
            Regex::new(r"(.+) connected\n(?:[\da-zA-Z]+x[\da-zA-Z]+ [\da-zA-Z]+\.[\da-zA-Z]+\n)+")
                .expect("hardcoded regexp");
        let opts = cmd::run_and_fetch_output(
            "xrandr | grep -Ev \"disconnected|Screen\" | awk '{print $1, $2}' | awk -F'[/+* ]' '{print $1\" \"$2}'"
        ).unwrap_or_else(|error| {
           match error {
               cmd::Error::EmptyOutput => unimplemented!(),
               cmd::Error::Io(error) => unimplemented!(),
           }
        });
        HashMap::from_iter(screens_regexp.captures_iter(&opts).map(|dp| {
            (
                dp[1].to_string(),
                dp[0]
                    .parse()
                    .expect("correct display options as it already matched regexp"),
            )
        }))
    }
}
