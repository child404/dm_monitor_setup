#![allow(unused_imports, unused_variables)]
use crate::dmenu_interface::dmenu_ms::{launch_ms, run_daemon, spawn_help};

pub mod cmd;
pub mod custom_errors;
pub mod dmenu_interface;
pub mod layouts_config;
pub mod monitor_layout;
pub mod params;
use std::env;
use std::str;

fn main() {
    match env::args().nth(1) {
        Some(arg) => match arg.as_str() {
            "--daemon" | "-d" => run_daemon(),
            "--help" | "-h" => spawn_help(),
            _ => println!("Undefined arg: {arg}"),
        },
        None => launch_ms(),
    }
}
