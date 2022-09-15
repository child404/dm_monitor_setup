#![allow(unused_imports, unused_variables, dead_code)]
use crate::config::Config;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Write;
use std::fs;
use std::io::Write as IoWrite;
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
pub struct MonitorLayouts {
    pub layouts: Vec<MonitorLayout>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MonitorLayout {
    pub name: String,
    pub monitors: Vec<Monitor>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Monitor {
    pub name: String,
    pub height_px: u16,
    pub width_px: u16,
    pub rate: u8,
    pub is_primary: bool,
    pub is_auto: bool,
    pub pos: MonitorPosition,
    pub dupl: MonitorDuplicated,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MonitorPosition {
    pub is_related: bool,
    pub related_pos: String,
    pub related_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MonitorDuplicated {
    pub is_duplicated: bool,
    pub name: String,
}

impl MonitorLayouts {
    pub fn from_config() -> Self {
        let layouts_toml = fs::read_to_string(Config::path_to_config())
            .expect("Should have been able to read the file");
        toml::from_str(&layouts_toml).expect("Correct monitor_setups.toml structure")
    }

    pub fn add_to_config(&self) {
        // TODO: add here check for existing layout namesself) {
        // TODO: restructure it to check/add one by one
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(Config::path_to_config())
            .unwrap();
        file.write_all(self._to_toml().as_bytes())
            .expect("MonitorLayouts written to a file");
    }

    fn _overwrite_config(&self) {
        // TODO: rewrite to not duplicate code in add_to_config():
        //       add to Config
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(Config::path_to_config())
            .unwrap();
        file.write_all(self._to_toml().as_bytes())
            .expect("MonitorLayouts written to a file");
    }

    fn _to_toml(&self) -> String {
        toml::to_string(&self).expect("Convert MonitorSetups to toml")
    }

    pub fn names(&self) -> Vec<String> {
        self.layouts
            .iter()
            .map(|monitor_setup| monitor_setup.name.clone())
            .collect::<Vec<String>>()
    }

    pub fn get_layout_by(&self, name: String) -> Result<MonitorLayout, ()> {
        for layout in self.layouts.iter() {
            if layout.name == name {
                return Ok(layout.clone());
            }
        }
        Err(())
    }

    pub fn remove_layout(&mut self, name: String) {
        if let Some(pos) = self.layouts.iter().position(|layout| *layout.name == name) {
            self.layouts.remove(pos);
        }
        self._overwrite_config();
    }

    pub fn is_empty(&self) -> bool {
        self.layouts.is_empty()
    }
}

impl MonitorLayout {
    pub fn to_xrandr_command(&self) -> String {
        let mut command = String::from("xrandr");
        for monitor in self.monitors.iter() {
            write!(&mut command, " {}", &monitor.to_xrandr_output()).unwrap();
        }
        println!("{:?}", &command);
        command
    }

    pub fn add_to_config(&self) {
        MonitorLayouts {
            layouts: vec![self.clone()],
        }
        .add_to_config();
    }

    pub fn apply(&self) {
        Command::new("bash")
            .arg("-c")
            .arg(self.to_xrandr_command())
            .spawn()
            .expect("xrandr successfully applied layout");
    }
}

impl Monitor {
    pub fn to_xrandr_output(&self) -> String {
        let mut output = format!(
            "--output {} --mode {}x{} --rate {}",
            self.name, self.height_px, self.width_px, self.rate
        );
        if self.is_auto {
            output += " --auto";
        }
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
