use itertools::Itertools;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::{
    cmp::{Eq, Ord, Ordering, PartialEq},
    hash::Hash,
    marker::Copy,
    str::{self, FromStr},
};

use super::monitor_error::XrandrError;

#[derive(Debug, Default, Clone)]
pub struct MonitorOptions {
    pub resolutions: Vec<Resolution>,
    pub rates: Vec<RefreshRate>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq, Copy, Default)]
pub struct Resolution(pub u16, pub u16);

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq, Copy, Default)]
pub struct RefreshRate(pub u16);

fn filter_unique<T>(v: &mut [T]) -> Vec<T>
where
    T: Hash + Ord + Copy,
{
    v.sort_by(|a, b| b.cmp(a));
    v.iter().unique().take(10).copied().collect()
}

// TODO: impl such method for Vec
fn map_str<T: ToString>(t: &[T]) -> Vec<String> {
    t.iter().map(T::to_string).collect()
}

impl MonitorOptions {
    pub fn is_empty(&self) -> bool {
        self.resolutions.is_empty() || self.rates.is_empty()
    }

    pub fn resolutions(&self) -> Vec<String> {
        map_str(&self.resolutions)
    }

    pub fn rates(&self) -> Vec<String> {
        map_str(&self.rates)
    }

    fn remove_duplicates(&mut self) {
        self.resolutions = filter_unique(&mut self.resolutions);
        self.rates = filter_unique(&mut self.rates);
    }

    fn add(&mut self, res: Resolution, rate: RefreshRate) {
        self.resolutions.push(res);
        self.rates.push(rate);
    }
}

impl FromStr for MonitorOptions {
    fn from_str(screen_settings: &str) -> Result<Self, Self::Err> {
        let mut screen_opts = Self::default();
        for setting in Regex::new(r"(\d+x\d+) (\d+\.\d+)\n")
            .unwrap()
            .captures_iter(screen_settings)
        {
            screen_opts.add(
                setting[1].parse().ok().unwrap(),
                setting[2].parse().ok().unwrap(),
            );
        }
        screen_opts.remove_duplicates();
        Ok(screen_opts)
    }

    type Err = XrandrError;
}

impl PartialOrd for Resolution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Resolution {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 != other.0 {
            self.0.cmp(&other.0)
        } else {
            self.1.cmp(&other.1)
        }
    }
}

impl FromStr for Resolution {
    fn from_str(res: &str) -> Result<Self, Self::Err> {
        if let [h, w] = res
            .split('x')
            .take(2)
            .flat_map(|x| x.parse::<u16>())
            .collect::<Vec<u16>>()[..]
        {
            Ok(Self(h, w))
        } else {
            Err(Self::Err::InvalidResolution)
        }
    }

    type Err = XrandrError;
}

impl ToString for Resolution {
    fn to_string(&self) -> String {
        format!("{}x{}", self.0, self.1)
    }
}

impl PartialOrd for RefreshRate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RefreshRate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl FromStr for RefreshRate {
    fn from_str(rate: &str) -> Result<Self, Self::Err> {
        if let Ok(rate) = rate.parse::<f64>() {
            Ok(Self(rate.round() as u16))
        } else {
            Err(Self::Err::InvalidRate)
        }
    }

    type Err = XrandrError;
}

impl ToString for RefreshRate {
    fn to_string(&self) -> String {
        format!("{:.1}", self.0)
    }
}
