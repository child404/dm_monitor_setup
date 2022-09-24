use itertools::Itertools;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::{
    cmp::{Eq, Ord, Ordering, PartialEq},
    hash::Hash,
    marker::Copy,
    str::{self, FromStr},
};

use crate::custom_errors::ScreenError;

#[derive(Debug, Default)]
pub struct ScreenOptions {
    pub resolutions: Vec<ScreenRes>,
    pub rates: Vec<ScreenRate>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq, Copy)]
pub struct ScreenRes(pub u16, pub u16);

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq, Copy)]
pub struct ScreenRate(pub u16);

fn filter_unique<T: Hash + Ord + Copy>(v: &mut [T]) -> Vec<T> {
    v.sort_by(|a, b| b.cmp(a));
    v.iter().unique().take(10).copied().collect()
}

fn map_str<T: ToString>(t: &[T]) -> Vec<String> {
    t.iter().map(T::to_string).collect()
}

impl ScreenOptions {
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

    fn add(&mut self, res: ScreenRes, rate: ScreenRate) {
        // FIXME: change res and rate to structs types and sort descending
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
            screen_opts.add(
                setting[1].parse().ok().unwrap(),
                setting[2].parse().ok().unwrap(),
            );
        }
        screen_opts.remove_duplicates();
        Ok(screen_opts)
    }

    type Err = ScreenError;
}

impl PartialOrd for ScreenRes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScreenRes {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 != other.0 {
            self.0.cmp(&other.0)
        } else {
            self.1.cmp(&other.1)
        }
    }
}

impl PartialOrd for ScreenRate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScreenRate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
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
            Ok(Self(rate.round() as u16))
        } else {
            Err(Self::Err::InvalidScreenRate)
        }
    }

    type Err = ScreenError;
}

impl ToString for ScreenRate {
    fn to_string(&self) -> String {
        format!("{:.1}", self.0)
    }
}
