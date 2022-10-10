use serde_derive::{Deserialize, Serialize};
use std::{error::Error as StdError, fmt, str};

use super::monitor_options::{RefreshRate, Resolution};

pub type OutputName = String;

#[derive(Debug)]
pub enum Error {
    InvalidResolution,
    InvalidRate,
    InvalidPosition,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidResolution => f.write_str("InvalidOutputResolution"),
            Self::InvalidRate => f.write_str("InvalidOutputRate"),
            Self::InvalidPosition => f.write_str("InvalidOutputPosition"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Self::InvalidResolution => "Invalid Output Resolution",
            Self::InvalidRate => "Invalid Output Rate",
            Self::InvalidPosition => "Invalid Output Resolution",
        }
    }
}

// TODO: add ScreenOptions to the monitor to be able to change res/rate on the fly:
//      new options: Change current res, Change current rate
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Output {
    pub name: OutputName,
    pub mode: Mode,
    pub is_primary: bool,
    pub state: State,
    pub position: Position,
    pub orientation: Orientation,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Mode {
    pub resolution: Resolution,
    pub rate: RefreshRate,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum Position {
    #[default]
    Center,
    LeftOf(OutputName),
    RightOf(OutputName),
    Above(OutputName),
    Below(OutputName),
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum State {
    #[default]
    Disconnected,
    Duplicated(OutputName),
    Connected,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum Orientation {
    #[default]
    Normal,
    Inverted,
    Left,
    Right,
}

impl Position {
    pub fn from_str(position: &str) -> Result<Self, Error> {
        if let [position, output_name] =
            position.split_whitespace().take(2).collect::<Vec<&str>>()[..]
        {
            let output_name = output_name.to_string();
            let position = match position {
                "--left-of" => Self::LeftOf(output_name),
                "--right-of" => Self::RightOf(output_name),
                "--above" => Self::Above(output_name),
                "--below" => Self::Below(output_name),
                _ => Self::Center,
            };
            Ok(position)
        } else {
            Err(Error::InvalidPosition)
        }
    }
}

impl ToString for Position {
    fn to_string(&self) -> String {
        match self {
            Self::LeftOf(output_name) => "--left-of ".to_owned() + output_name,
            Self::RightOf(output_name) => "--right-of ".to_owned() + output_name,
            Self::Above(output_name) => "--above ".to_owned() + output_name,
            Self::Below(output_name) => "--below ".to_owned() + output_name,
            _ => String::new(),
        }
    }
}

impl ToString for State {
    fn to_string(&self) -> String {
        match self {
            Self::Duplicated(output_name) => "--same-as ".to_owned() + output_name,
            Self::Disconnected => "--off".to_string(),
            Self::Connected => "".to_string(),
        }
    }
}

impl ToString for Orientation {
    fn to_string(&self) -> String {
        match self {
            Self::Normal => "normal",
            Self::Inverted => "inverted",
            Self::Left => "left",
            Self::Right => "right",
        }
        .to_string()
    }
}

impl ToString for Output {
    fn to_string(&self) -> String {
        format!(
            "--output {} --mode {} --rate {} --orientation {} {} {} {}",
            self.name,
            self.mode.resolution.to_string(),
            self.mode.rate.to_string(),
            self.orientation.to_string(),
            self.position.to_string(),
            self.state.to_string(),
            {
                if self.is_primary {
                    "--primary"
                } else {
                    ""
                }
            }
        )
    }
}
