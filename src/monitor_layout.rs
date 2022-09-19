#![allow(unused_imports, unused_variables, dead_code)]
use crate::cmd::{term, xrandr};
use crate::custom_errors::LayoutError;
use crate::layouts_config::LayoutsConfig;
use crate::params::Params;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Write;
use std::io::Write as IoWrite;

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
    pub rate: f32,
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

impl MonitorPosition {
    pub fn not_related() -> Self {
        Self {
            is_related: false,
            related_pos: "".to_string(),
            related_name: "".to_string(),
        }
    }

    pub fn related(related_pos: &str, related_name: &str) -> Self {
        Self {
            is_related: true,
            related_pos: related_pos.to_string(),
            related_name: related_name.to_string(),
        }
    }
}

impl MonitorDuplicated {
    pub fn not_duplicated() -> Self {
        Self {
            is_duplicated: false,
            name: "".to_string(),
        }
    }

    pub fn duplicated(name: &str) -> Self {
        Self {
            is_duplicated: true,
            name: name.to_string(),
        }
    }
}

impl MonitorLayouts {
    pub fn from_config() -> Self {
        toml::from_str(&LayoutsConfig::read()).expect("Correct monitor_setups.toml structure")
    }

    pub fn as_toml(&self) -> String {
        toml::to_string(&self).expect("Convert MonitorLayouts to toml")
    }

    pub fn names(&self) -> Vec<String> {
        self.layouts
            .iter()
            .map(|monitor_setup| monitor_setup.name.clone())
            .collect::<Vec<String>>()
    }

    pub fn find_layout(&self, name: &str) -> Result<usize, LayoutError> {
        if let Some(pos) = self
            .layouts
            .iter()
            .position(|layout| layout.name.as_str() == name)
        {
            return Ok(pos);
        }
        Err(LayoutError::LayoutNotFound)
    }

    pub fn get_layout_by(&self, name: &str) -> Result<MonitorLayout, LayoutError> {
        match self.find_layout(name) {
            Ok(pos) => Ok(self.layouts[pos].clone()),
            Err(err) => Err(err),
        }
    }

    pub fn remove_layout(&mut self, pos: usize) {
        self.layouts.remove(pos);
        LayoutsConfig::overwrite_with(self);
    }

    pub fn is_empty(&self) -> bool {
        self.layouts.is_empty()
    }
}

impl MonitorLayout {
    pub fn apply(&self) {
        // TODO: send here current layout to daemon or save to file
        term::exec(&xrandr::from_monitor_layout(self));
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

impl ToString for Monitor {
    fn to_string(&self) -> String {
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

#[test]
fn test_monitor_setup() {
    let monitor = Monitor {
        name: "eDP-1".to_string(),
        height_px: 1920,
        width_px: 1200,
        rate: 120.0,
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
    let layouts = MonitorLayouts {
        layouts: vec![monitor_setup],
    };
    println!("{:#?}", MonitorLayouts::from_config());
    println!("{:#?}", layouts);

    LayoutsConfig::add(layouts);
}
