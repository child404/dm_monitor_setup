use serde_derive::{Deserialize, Serialize};
use std::{error::Error as StdError, fmt};

use super::{
    layout_config::LayoutConfig,
    monitor::{Output, OutputName},
};
use crate::core::handlers::{command_line as cmd, xrandr::Xrandr};

#[derive(Debug)]
pub enum Error {
    LayoutNotFound,
    InternalServerError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::LayoutNotFound => f.write_str("LayoutNotFound"),
            Error::InternalServerError => f.write_str("InternalServerError"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::LayoutNotFound => "Layout not found",
            Error::InternalServerError => "Internal Server Error",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Layouts {
    pub layouts: Vec<Layout>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Layout {
    pub name: String,
    pub outputs: Vec<Output>,
}

// TODO: replace Layouts with LayoutConfig
// layouts: Vec<Layout>
impl Layouts {
    pub fn from_config() -> Option<Self> {
        let content = LayoutConfig::read();
        if !content.is_empty() {
            Some(toml::from_str(&content).expect("Correct monitor_setups.toml structure"))
        } else {
            None
        }
    }

    pub fn as_toml(&self) -> Option<String> {
        if !self.layouts.is_empty() {
            Some(toml::to_string(&self).expect("Convert MonitorLayouts to toml"))
        } else {
            None
        }
    }

    pub fn names(&self) -> Vec<String> {
        self.layouts
            .iter()
            .map(|monitor_setup| monitor_setup.name)
            .collect()
    }

    pub fn find_layout_pos(&self, name: &str) -> Result<usize, Error> {
        if let Some(pos) = self.layouts.iter().position(|layout| layout.name == name) {
            Ok(pos)
        } else {
            Err(Error::LayoutNotFound)
        }
    }

    pub fn get_layout_by(&self, name: &str) -> Result<Layout, Error> {
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

impl Layout {
    pub fn apply(&self) {
        // TODO: send here current layout to daemon or save to file
        cmd::run(&Xrandr::from_monitor_layout(self));
    }

    pub fn output_names(&self) -> Vec<OutputName> {
        self.outputs
            .iter()
            .map(|monitor_setup| monitor_setup.name)
            .collect()
    }

    pub fn current() -> Self {
        let content = LayoutConfig::read_current_layout();
        if content.is_empty() {
            return Self::default();
        }
        toml::from_str(&content).expect("Correct current_layout.toml structure")
    }
}

impl ToString for Layout {
    fn to_string(&self) -> String {
        self.outputs
            .iter()
            .map(|monitor| monitor.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
