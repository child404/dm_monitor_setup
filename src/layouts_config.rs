use crate::monitor_layout::{MonitorLayout, MonitorLayouts};
use crate::params::Params;
use std::fs;
use std::io::Write;

pub struct LayoutsConfig;

impl LayoutsConfig {
    pub fn add(layouts: MonitorLayouts) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(Params::path_to_config())
            .unwrap();
        file.write_all(layouts.as_toml().as_bytes())
            .expect("MonitorLayouts written to a file");
    }

    pub fn read() -> String {
        fs::read_to_string(Params::path_to_config())
            .expect("Should have been able to read the file")
    }

    pub fn overwrite_with(layouts: &MonitorLayouts) {
        // TODO: rewrite to not duplicate code in add()
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(Params::path_to_config())
            .unwrap();
        file.write_all(layouts.as_toml().as_bytes())
            .expect("MonitorLayouts written to a file");
    }
}
