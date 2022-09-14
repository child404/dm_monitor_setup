#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;
use std::fs;

pub fn read_config() {
    let path_to_config = "/home/child404/.config/dm_monitor_setup/monitor_setups.toml";
    let toml_string =
        fs::read_to_string(path_to_config).expect("Should have been able to read the file");
    let config: MonitorSetups =
        toml::from_str(&toml_string).expect("Correct monitor_setups.toml structure");
    println!("{:#?}", config);
}

pub fn write_config(setups: MonitorSetups) {
    let toml_string = toml::to_string(&setups).expect("Convert MonitorSetups to toml");
    println!("{:#?}", toml_string)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MonitorSetups {
    pub setup: Vec<MonitorSetup>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MonitorSetup {
    pub name: String,
    pub monitors: Vec<Monitor>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Monitor {
    pub name: String,
    pub height_px: u16,
    pub width_px: u16,
    pub rate: u8,
    pub pos: MonitorPosition,
    pub dupl: MonitorDuplicated,
    pub is_primary: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MonitorPosition {
    pub is_related: bool,
    pub related_pos: String,
    pub related_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MonitorDuplicated {
    pub is_duplicated: bool,
    pub name: String,
}

impl MonitorSetups {
    pub fn to_toml(&self) {
        let toml_string = toml::to_string(&self).expect("Convert MonitorSetups to toml");
        println!("{:#?}", toml_string)
    }
}

impl MonitorSetup {
    pub fn from_config(setup_name: &str) -> Self {
        unimplemented!()
    }

    pub fn add_to_config(&self) {
        let path_to_config = "$HOME/.config/dm_monitor_setup/monitor_setups.toml";
        unimplemented!()
    }

    pub fn to_xrandr_command(&self) -> String {
        let mut command = String::from("xrandr");
        for monitor in self.monitors.iter() {
            command += &monitor.to_xrandr_output();
        }
        command
    }
}

impl Monitor {
    pub fn from_string(xrandr_output: String) -> Self {
        unimplemented!()
    }

    pub fn to_xrandr_output(&self) -> String {
        let mut output = format!(
            "--output {} --mode {}x{} --rate {}",
            self.name, self.height_px, self.width_px, self.rate
        );
        if self.pos.is_related {
            write!(
                &mut output,
                " --{} {}",
                self.pos.related_pos, self.pos.related_name
            )
            .unwrap();
        }
        if self.dupl.is_duplicated {
            write!(&mut output, " --same-as {}", self.dupl.name).unwrap();
        }
        if self.is_primary {
            output += " --primary";
        }
        output
    }
}
