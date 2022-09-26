use serde_derive::{Deserialize, Serialize};

use super::{layout_config::LayoutConfig, layout_error::LayoutError, monitor::Monitor};
use crate::core::handlers::{term::TermCMD, xrandr::XrandrCMD};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MonitorLayouts {
    pub layouts: Vec<MonitorLayout>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MonitorLayout {
    pub name: String,
    pub monitors: Vec<Monitor>,
}

impl MonitorLayouts {
    pub fn from_config() -> Self {
        let content = LayoutConfig::read();
        if content.is_empty() {
            return Self::default();
        }
        toml::from_str(&content).expect("Correct monitor_setups.toml structure")
    }

    pub fn as_toml(&self) -> String {
        if self.layouts.is_empty() {
            return String::new();
        }
        toml::to_string(&self).expect("Convert MonitorLayouts to toml")
    }

    pub fn names(&self) -> Vec<String> {
        self.layouts
            .iter()
            .map(|monitor_setup| monitor_setup.name.clone())
            .collect::<Vec<String>>()
    }

    pub fn find_layout_pos(&self, name: &str) -> Result<usize, LayoutError> {
        if let Some(pos) = self
            .layouts
            .iter()
            .position(|layout| layout.name.as_str() == name)
        {
            Ok(pos)
        } else {
            Err(LayoutError::LayoutNotFound)
        }
    }

    pub fn get_layout_by(&self, name: &str) -> Result<MonitorLayout, LayoutError> {
        match self.find_layout_pos(name) {
            Ok(pos) => Ok(self.layouts[pos].clone()),
            Err(err) => Err(err),
        }
    }

    pub fn remove_layout(&mut self, pos: usize) {
        self.layouts.remove(pos);
        LayoutConfig::overwrite_with(self);
    }

    pub fn is_empty(&self) -> bool {
        self.layouts.is_empty()
    }
}

impl MonitorLayout {
    pub fn apply(&self) {
        // TODO: send here current layout to daemon or save to file
        TermCMD::exec(&XrandrCMD::from_monitor_layout(self));
    }

    pub fn monitor_names(&self) -> Vec<String> {
        self.monitors
            .iter()
            .map(|monitor_setup| monitor_setup.name.clone())
            .collect::<Vec<String>>()
    }

    pub fn current() -> Self {
        let content = LayoutConfig::read_current_layout();
        if content.is_empty() {
            return Self::default();
        }
        toml::from_str(&content).expect("Correct current_layout.toml structure")
    }
}

impl ToString for MonitorLayout {
    fn to_string(&self) -> String {
        self.monitors
            .iter()
            .map(|monitor| monitor.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[test]
fn test_monitor_setup() {
    let monitor = Monitor {
        name: "eDP-1".to_string(),
        res: "1920x1200".parse().ok().unwrap(),
        rate: "120.0".parse().ok().unwrap(),
        is_primary: true,
        is_auto: true,
        pos: MonitorPosition {
            is_related: false,
            related_pos: "".to_string(),
            related_name: "".to_string(),
        },
        dupl: MonitorDuplicated {
            is_duplicated: false,
            name: "".to_string(),
        },
    };
    let monitor_setup = MonitorLayout {
        name: "Single monitor".to_string(),
        monitors: vec![monitor],
    };
    println!("{:#?}", MonitorLayouts::from_config());
    println!("{:#?}", &monitor_setup);
    let mut layouts_cfg = LayoutConfig::default();

    layouts_cfg.add_or_overwrite_if_exists(&monitor_setup);
}
