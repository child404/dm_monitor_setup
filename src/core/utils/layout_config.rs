use super::monitor_layout::{MonitorLayout, MonitorLayouts};
use crate::defaults;
use std::fs;
use std::io::Write;

pub struct LayoutConfig {
    pub current_layout: MonitorLayout,
    pub user_layouts: MonitorLayouts,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            current_layout: MonitorLayout::current(),
            user_layouts: MonitorLayouts::from_config(),
        }
    }
}

impl LayoutConfig {
    pub fn add_or_overwrite_if_exists(&mut self, layout: &MonitorLayout) {
        if let Ok(pos) = self.user_layouts.find_layout_pos(&layout.name) {
            self.user_layouts.layouts[pos] = layout.clone();
            return LayoutConfig::overwrite_with(&self.user_layouts);
        }
        LayoutConfig::add(layout);
    }

    fn add(layout: &MonitorLayout) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(defaults::find_path_to_config())
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
        fs::read_to_string(defaults::find_path_to_config())
            .expect("Should have been able to read the file")
    }

    pub fn overwrite_with(layouts: &MonitorLayouts) {
        // TODO: rewrite to not duplicate code in add()
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(defaults::find_path_to_config())
            .unwrap();
        file.write_all(layouts.as_toml().as_bytes())
            .expect("MonitorLayouts written to a file");
    }

    pub fn read_current_layout() -> String {
        fs::read_to_string(defaults::PATH_TO_CURRENT_LAYOUT)
            .expect("Should have been able to read the file")
    }
}
