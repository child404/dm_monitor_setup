#![allow(unused_variables, dead_code)]
use crate::cmd::{term::TermCmd, xrandr::XrandrCmd};
use crate::custom_errors::{LayoutError, ScreenError};
use crate::layouts_config::LayoutsConfig;
use crate::params::Params;
use itertools::Itertools;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::{
    fmt::Write,
    io::Write as IoWrite,
    str::{self, FromStr},
};

#[derive(Debug, Default)]
pub struct ScreenOptions {
    pub resolutions: Vec<String>,
    pub rates: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScreenRes(u16, u16);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScreenRate(f64);

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MonitorLayouts {
    pub layouts: Vec<MonitorLayout>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MonitorLayout {
    pub name: String,
    pub monitors: Vec<Monitor>,
}

// TODO: add ScreenOptions to the monitor to be able to change res/rate on the fly:
//      new options: Change current res, Change current rate
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Monitor {
    pub name: String,
    pub res: ScreenRes,
    pub rate: ScreenRate,
    pub is_primary: bool,
    pub is_auto: bool,
    pub pos: MonitorPosition,
    pub dupl: MonitorDuplicated,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MonitorPosition {
    pub is_related: bool,
    pub related_pos: String,
    pub related_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct MonitorDuplicated {
    pub is_duplicated: bool,
    pub name: String,
}

fn unique_and_sort(v: &mut [String]) -> Vec<String> {
    v.sort(); // TODO: maybe remove that
    v.iter().unique().map(|x| x.to_string()).collect()
}

impl ScreenOptions {
    pub fn is_empty(&self) -> bool {
        self.resolutions.is_empty() || self.rates.is_empty()
    }

    fn remove_duplicates(&mut self) {
        self.resolutions = unique_and_sort(&mut self.resolutions);
        self.rates = unique_and_sort(&mut self.rates);
    }

    fn add(&mut self, res: String, rate: String) {
        self.resolutions.push(res);
        self.rates.push(rate);
    }
}

impl FromStr for ScreenOptions {
    fn from_str(screen_settings: &str) -> Result<Self, Self::Err> {
        let mut screen_opts = Self::default();
        for setting in Regex::new(r"(\d+x\d+) (\d+\.\d+)\n")
            .unwrap()
            .captures_iter(screen_settings)
        {
            // ScreenRes's and ScreenRate's values can be safely unwrapped as we already matched
            // the proper structure of the resolution and rate by the regex
            screen_opts.add(setting[1].to_string(), setting[2].to_string());
        }
        screen_opts.remove_duplicates();
        Ok(screen_opts)
    }

    type Err = ScreenError;
}

impl FromStr for ScreenRes {
    fn from_str(res: &str) -> Result<Self, Self::Err> {
        if let [h, w] = res
            .split('x')
            .take(2)
            .flat_map(|x| x.parse::<u16>())
            .collect::<Vec<u16>>()[..]
        {
            Ok(Self(h, w))
        } else {
            Err(Self::Err::InvalidScreenResolution)
        }
    }

    type Err = ScreenError;
}

impl ToString for ScreenRes {
    fn to_string(&self) -> String {
        format!("{}x{}", self.0, self.1)
    }
}

impl FromStr for ScreenRate {
    fn from_str(rate: &str) -> Result<Self, Self::Err> {
        if let Ok(rate) = rate.parse::<f64>() {
            Ok(Self(rate))
        } else {
            Err(Self::Err::InvalidScreenRate)
        }
    }

    type Err = ScreenError;
}

impl ToString for ScreenRate {
    fn to_string(&self) -> String {
        format!("{:.2}", self.0)
    }
}

impl MonitorPosition {
    pub fn related(related_pos: &str, related_name: &str) -> Self {
        Self {
            is_related: true,
            related_pos: related_pos.to_string(),
            related_name: related_name.to_string(),
        }
    }
}

impl MonitorDuplicated {
    pub fn duplicated(name: &str) -> Self {
        Self {
            is_duplicated: true,
            name: name.to_string(),
        }
    }
}

impl MonitorLayouts {
    pub fn from_config() -> Self {
        let content = LayoutsConfig::read();
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
        TermCmd::exec(&XrandrCmd::from_monitor_layout(self));
    }

    pub fn monitor_names(&self) -> Vec<String> {
        self.monitors
            .iter()
            .map(|monitor_setup| monitor_setup.name.clone())
            .collect::<Vec<String>>()
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
            self.name, self.res.0, self.res.1, self.rate.0
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
        res: ScreenRes::from_str("1920x1200").ok().unwrap(),
        rate: ScreenRate::from_str("120.0").ok().unwrap(),
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
