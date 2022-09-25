use crate::monitor_layout::{MonitorLayout, MonitorLayouts};
use crate::params::Params;
use std::fs;
use std::io::Write;

pub struct LayoutsConfig {
    pub current_layout: MonitorLayout,
    pub user_layouts: MonitorLayouts,
}

impl Default for LayoutsConfig {
    fn default() -> Self {
        Self {
            current_layout: MonitorLayout::current(),
            user_layouts: MonitorLayouts::from_config(),
        }
    }
}

impl LayoutsConfig {
    pub fn add_or_overwrite_if_exists(&mut self, layout: &MonitorLayout) {
        if let Ok(pos) = self.user_layouts.find_layout_pos(&layout.name) {
            self.user_layouts.layouts[pos] = layout.clone();
            return LayoutsConfig::overwrite_with(&self.user_layouts);
        }
        LayoutsConfig::add(layout);
    }

    fn add(layout: &MonitorLayout) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(Params::path_to_config())
            .unwrap();
        file.write_all(
            MonitorLayouts {
                layouts: vec![layout.clone()],
            }
            .as_toml()
            .as_bytes(),
        )
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

    pub fn read_current_layout() -> String {
        fs::read_to_string(Params::path_to_current_layout())
            .expect("Should have been able to read the file")
    }
}
